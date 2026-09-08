//! One entry of a TIFF tag directory.

use super::tiff_bytes::{TiffBytes, TiffError};
use std::fmt::{Display, Formatter, Result as FmtResult};
use studiole_report::prelude::*;

/// One entry of a TIFF tag directory.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TiffEntry {
    /// Code identifying the tag.
    pub tag: u16,
    /// Type of the values.
    pub kind: TiffEntryKind,
    /// Number of values.
    pub count: u32,
    /// Values when they fit in four bytes, otherwise their offset.
    pub value: u32,
}

impl TiffEntry {
    /// Get every value of this entry, widened to [`u32`].
    ///
    /// - Reads inline IF the values fit in the four byte value field
    ///
    /// # Errors
    ///
    /// - IF the type is neither `SHORT` nor `LONG`
    /// - IF the values lie outside the file
    pub fn get_values(&self, tiff: &TiffBytes) -> Result<Vec<u32>, Report<TiffError>> {
        let width = self.kind.bytes().ok_or_else(|| {
            Report::new(TiffError::UnsupportedType)
                .attach("tag", self.tag)
                .attach("kind", self.kind)
        })?;
        let count = TiffBytes::widen(self.count)?;
        let length = count * width;
        let inline = self.value.to_le_bytes();
        let bytes = if length <= inline.len() {
            inline.as_slice()
        } else {
            tiff.get_slice(TiffBytes::widen(self.value)?, length)?
        };
        let mut values = Vec::with_capacity(count);
        for index in 0..count {
            values.push(self.kind.get_value(bytes, index * width)?);
        }
        Ok(values)
    }
}

/// Type of the values an entry holds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TiffEntryKind {
    /// Sixteen bit unsigned integer.
    Short,
    /// Thirty two bit unsigned integer.
    Long,
    /// Sixty four bit float.
    Double,
    /// A type this reader does not read.
    Unsupported(u16),
}

impl TiffEntryKind {
    /// Create a new [`TiffEntryKind`] from an entry type code.
    #[must_use]
    pub const fn from_code(code: u16) -> Self {
        match code {
            3 => Self::Short,
            4 => Self::Long,
            12 => Self::Double,
            code => Self::Unsupported(code),
        }
    }

    /// Number of bytes one value occupies.
    ///
    /// - Returns [`None`] IF this reader does not read the type
    #[must_use]
    pub const fn bytes(self) -> Option<usize> {
        match self {
            Self::Short => Some(2),
            Self::Long => Some(4),
            Self::Double => Some(8),
            Self::Unsupported(_) => None,
        }
    }

    /// Get one value from the bytes holding it, widened to [`u32`].
    ///
    /// # Errors
    ///
    /// - IF the type is neither `SHORT` nor `LONG`
    /// - IF the bytes are too short
    fn get_value(self, bytes: &[u8], at: usize) -> Result<u32, Report<TiffError>> {
        match self {
            Self::Short => {
                let value = Self::get_bytes::<2>(bytes, at)?;
                Ok(u32::from(u16::from_le_bytes(value)))
            }
            Self::Long => {
                let value = Self::get_bytes::<4>(bytes, at)?;
                Ok(u32::from_le_bytes(value))
            }
            Self::Double | Self::Unsupported(_) => {
                Err(Report::new(TiffError::UnsupportedType).attach("kind", self))
            }
        }
    }

    /// Get a fixed width slice of bytes.
    fn get_bytes<const N: usize>(bytes: &[u8], at: usize) -> Result<[u8; N], Report<TiffError>> {
        let end = at.checked_add(N).ok_or(TiffError::Truncated)?;
        let slice = bytes
            .get(at..end)
            .ok_or_else(|| Report::new(TiffError::Truncated).attach("at", at))?;
        <[u8; N]>::try_from(slice).change_context(TiffError::Truncated)
    }
}

impl Display for TiffEntryKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match *self {
            Self::Short => f.write_str("SHORT"),
            Self::Long => f.write_str("LONG"),
            Self::Double => f.write_str("DOUBLE"),
            Self::Unsupported(code) => write!(f, "type {code}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiff_entry_kind_from_code() {
        // Act
        let kinds: Vec<TiffEntryKind> = [3, 4, 12, 5]
            .iter()
            .map(|code| TiffEntryKind::from_code(*code))
            .collect();
        // Assert
        assert_eq!(
            kinds,
            vec![
                TiffEntryKind::Short,
                TiffEntryKind::Long,
                TiffEntryKind::Double,
                TiffEntryKind::Unsupported(5),
            ]
        );
    }

    #[test]
    fn tiff_entry_kind_bytes() {
        // Act
        let bytes: Vec<Option<usize>> = [
            TiffEntryKind::Short,
            TiffEntryKind::Long,
            TiffEntryKind::Double,
            TiffEntryKind::Unsupported(5),
        ]
        .iter()
        .map(|kind| kind.bytes())
        .collect();
        // Assert
        assert_eq!(bytes, vec![Some(2), Some(4), Some(8), None]);
    }
}
