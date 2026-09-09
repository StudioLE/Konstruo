//! Checked reads over the bytes of a TIFF file.

use super::tiff_entry::{TiffEntry, TiffEntryKind};
use super::tiff_tags::TiffTags;
use studiole_report::prelude::*;
use thiserror::Error;

/// Byte length of one tag directory entry.
const ENTRY_BYTES: usize = 12;

/// Version every TIFF header carries.
const VERSION: u16 = 42;

/// Checked reads over the bytes of a TIFF file.
///
/// - Holds part of the file when constructed by [`TiffBytes::new_at`]
#[derive(Debug)]
pub struct TiffBytes {
    /// The bytes held, which are every byte of the file unless `base` is non-zero.
    bytes: Vec<u8>,
    /// Position in the file of the first byte held.
    ///
    /// TIFF positions address the file, so every read translates through this.
    ///
    /// Default: `0`
    base: usize,
}

impl TiffBytes {
    /// Create a new [`TiffBytes`] over every byte of a file.
    ///
    /// # Errors
    ///
    /// - IF the file is not a little-endian TIFF
    pub fn new(bytes: Vec<u8>) -> Result<Self, Report<TiffError>> {
        let tiff = Self { bytes, base: 0 };
        if tiff.get_slice(0, 2)? != b"II" {
            return Err(Report::new(TiffError::NotLittleEndian));
        }
        let version = tiff.get_u16(2)?;
        if version != VERSION {
            return Err(Report::new(TiffError::NotTiff).attach("version", version));
        }
        Ok(tiff)
    }

    /// Create a new [`TiffBytes`] over the bytes of a file from `base` onwards.
    ///
    /// - Holds no header, so the caller validates it and reads the directory
    ///   through [`TiffBytes::get_tags_at`] rather than [`TiffBytes::get_tags`]
    #[must_use]
    pub const fn new_at(bytes: Vec<u8>, base: usize) -> Self {
        Self { bytes, base }
    }

    /// Get the tag directory the header points at.
    ///
    /// # Errors
    ///
    /// - IF the header or the directory lies outside the bytes held
    pub fn get_tags(&self) -> Result<TiffTags, Report<TiffError>> {
        self.get_tags_at(Self::widen(self.get_u32(4)?)?)
    }

    /// Get the tag directory at a position.
    ///
    /// # Errors
    ///
    /// - IF the directory lies outside the bytes held
    pub fn get_tags_at(&self, at: usize) -> Result<TiffTags, Report<TiffError>> {
        let count = usize::from(self.get_u16(at)?);
        let mut entries = Vec::with_capacity(count);
        for index in 0..count {
            let at = at + 2 + index * ENTRY_BYTES;
            entries.push(TiffEntry {
                tag: self.get_u16(at)?,
                kind: TiffEntryKind::from_code(self.get_u16(at + 2)?),
                count: self.get_u32(at + 4)?,
                value: self.get_u32(at + 8)?,
            });
        }
        Ok(TiffTags::new(entries))
    }

    /// Get a little-endian [`u16`].
    ///
    /// # Errors
    ///
    /// - IF the value lies outside the file
    pub fn get_u16(&self, at: usize) -> Result<u16, Report<TiffError>> {
        let bytes = self.get_array::<2>(at)?;
        Ok(u16::from_le_bytes(bytes))
    }

    /// Get a little-endian [`u32`].
    ///
    /// # Errors
    ///
    /// - IF the value lies outside the file
    pub fn get_u32(&self, at: usize) -> Result<u32, Report<TiffError>> {
        let bytes = self.get_array::<4>(at)?;
        Ok(u32::from_le_bytes(bytes))
    }

    /// Get a little-endian [`f64`].
    ///
    /// # Errors
    ///
    /// - IF the value lies outside the file
    pub fn get_f64(&self, at: usize) -> Result<f64, Report<TiffError>> {
        let bytes = self.get_array::<8>(at)?;
        Ok(f64::from_le_bytes(bytes))
    }

    /// Get a slice of bytes at a position in the file.
    ///
    /// # Errors
    ///
    /// - IF the slice lies outside the bytes held, which includes lying before `base`
    pub fn get_slice(&self, at: usize, length: usize) -> Result<&[u8], Report<TiffError>> {
        let start = at
            .checked_sub(self.base)
            .ok_or_else(|| Report::new(TiffError::Truncated).attach("at", at))?;
        let end = start
            .checked_add(length)
            .ok_or_else(|| Report::new(TiffError::Truncated).attach("at", at))?;
        self.bytes.get(start..end).ok_or_else(|| {
            Report::new(TiffError::Truncated)
                .attach("at", at)
                .attach("length", length)
        })
    }

    /// Widen a TIFF offset or count to an index.
    ///
    /// # Errors
    ///
    /// - IF the value exceeds the address space
    pub(super) fn widen(value: u32) -> Result<usize, Report<TiffError>> {
        usize::try_from(value).change_context(TiffError::Offset)
    }

    /// Get a fixed width array of bytes.
    fn get_array<const N: usize>(&self, at: usize) -> Result<[u8; N], Report<TiffError>> {
        let slice = self.get_slice(at, N)?;
        <[u8; N]>::try_from(slice).change_context(TiffError::Truncated)
    }
}

/// Errors returned by [`TiffBytes`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum TiffError {
    /// Not a little-endian file.
    #[error("Not a little-endian file")]
    NotLittleEndian,
    /// Not a TIFF file.
    #[error("Not a TIFF file")]
    NotTiff,
    /// Truncated file.
    #[error("Truncated file")]
    Truncated,
    /// Offset exceeds the address space.
    #[error("Offset exceeds the address space")]
    Offset,
    /// Missing tag.
    #[error("Missing tag")]
    MissingTag,
    /// Tag holds more than one value.
    #[error("Tag holds more than one value")]
    NotScalar,
    /// Unsupported entry type.
    #[error("Unsupported entry type")]
    UnsupportedType,
}

#[cfg(test)]
mod tests {
    use super::super::tiff_fixture::TiffFixture;
    use super::super::tiff_tag::TiffTag;
    use super::*;

    /// Easting written to the fixture tiepoint.
    ///
    /// - Arbitrary, this module reading a tiepoint without interpreting it
    const EASTING: f64 = 1_000.0;

    /// Northing written to the fixture tiepoint.
    ///
    /// - Arbitrary, this module reading a tiepoint without interpreting it
    const NORTHING: f64 = 2_000.0;

    /// Byte offset of the easting within `ModelTiepoint`.
    const EASTING_BYTES: usize = 24;

    #[test]
    fn tiff_bytes_new() {
        // Arrange
        let bytes = TiffFixture::new().to_bytes();
        // Act
        let output = TiffBytes::new(bytes);
        // Assert
        assert!(output.is_ok());
    }

    #[test]
    fn tiff_bytes_new_big_endian() {
        // Arrange
        let mut bytes = TiffFixture::new().to_bytes();
        bytes.splice(0..2, *b"MM");
        // Act
        let output = TiffBytes::new(bytes);
        // Assert
        let report = output.expect_err("should reject file");
        assert_eq!(*report.current_context(), TiffError::NotLittleEndian);
    }

    #[test]
    fn tiff_bytes_new_truncated() {
        // Arrange
        let bytes = vec![b'I'];
        // Act
        let output = TiffBytes::new(bytes);
        // Assert
        let report = output.expect_err("should reject file");
        assert_eq!(*report.current_context(), TiffError::Truncated);
    }

    /// Values resolve at their position in the file, not their index in the bytes held.
    #[test]
    fn tiff_bytes_new_at() {
        // Arrange
        let bytes = TiffFixture::new()
            .with_tiepoint(EASTING, NORTHING)
            .to_bytes();
        let at = tiepoint_position(&bytes);
        // Act
        let tail = bytes.get(at..).expect("should hold tiepoint").to_vec();
        let tiff = TiffBytes::new_at(tail, at);
        // Assert
        let easting = tiff
            .get_f64(at + EASTING_BYTES)
            .expect("should read easting");
        assert_eq!(easting.to_bits(), EASTING.to_bits());
    }

    #[test]
    fn tiff_bytes_get_slice_before_base() {
        // Arrange
        let tiff = TiffBytes::new_at(vec![0; 16], 64);
        // Act
        let output = tiff.get_slice(32, 4);
        // Assert
        let report = output.expect_err("should reject position");
        assert_eq!(*report.current_context(), TiffError::Truncated);
    }

    #[test]
    fn tiff_bytes_get_tags_at() {
        // Arrange
        let bytes = TiffFixture::new().to_bytes();
        let tiff = TiffBytes::new(bytes).expect("should read header");
        let at = directory_position(&tiff);
        // Act
        let tags = tiff.get_tags_at(at).expect("should read directory");
        // Assert
        let entry = tags
            .get(TiffTag::ModelTiepoint)
            .expect("should have tiepoint");
        assert_eq!(entry.count, 6);
    }

    #[test]
    fn tiff_bytes_get_tags_truncated() {
        // Arrange
        let mut bytes = TiffFixture::new().to_bytes();
        bytes.truncate(16);
        let tiff = TiffBytes::new(bytes).expect("should read header");
        // Act
        let output = tiff.get_tags();
        // Assert
        let report = output.expect_err("should reject directory");
        assert_eq!(*report.current_context(), TiffError::Truncated);
    }

    /// The last entry of the directory parses, so every entry before it did.
    #[test]
    fn tiff_bytes_get_tags() {
        // Arrange
        let bytes = TiffFixture::new().to_bytes();
        let tiff = TiffBytes::new(bytes).expect("should read header");
        // Act
        let tags = tiff.get_tags().expect("should read directory");
        // Assert
        let entry = tags
            .get(TiffTag::ModelTiepoint)
            .expect("should have tiepoint");
        assert_eq!(entry.count, 6);
        assert_eq!(entry.kind, TiffEntryKind::Double);
    }

    /// Get the position of the tag directory.
    fn directory_position(tiff: &TiffBytes) -> usize {
        TiffBytes::widen(tiff.get_u32(4).expect("should read header")).expect("should fit")
    }

    /// Get the position of `ModelTiepoint` within a fixture.
    fn tiepoint_position(bytes: &[u8]) -> usize {
        let tiff = TiffBytes::new(bytes.to_vec()).expect("should read header");
        let tags = tiff.get_tags().expect("should read directory");
        let entry = tags
            .get(TiffTag::ModelTiepoint)
            .expect("should have tiepoint");
        TiffBytes::widen(entry.value).expect("should fit")
    }
}
