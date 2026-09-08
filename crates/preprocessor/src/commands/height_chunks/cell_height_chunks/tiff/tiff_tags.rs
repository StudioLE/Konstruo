//! Parsed tag directory of a TIFF file.

use super::tiff_bytes::{TiffBytes, TiffError};
use super::tiff_entry::{TiffEntry, TiffEntryKind};
use super::tiff_tag::TiffTag;
use studiole_report::prelude::*;

/// Parsed tag directory of a TIFF file.
///
/// - Named for the tags it holds, so it does not read as a filesystem directory
#[derive(Debug)]
pub struct TiffTags {
    /// Every entry of the directory, in the order the file lists them.
    entries: Vec<TiffEntry>,
}

impl TiffTags {
    /// Create a new [`TiffTags`].
    #[must_use]
    pub fn new(entries: Vec<TiffEntry>) -> Self {
        Self { entries }
    }

    /// Get one entry.
    ///
    /// # Errors
    ///
    /// - IF the directory has no such tag
    pub fn get(&self, tag: TiffTag) -> Result<TiffEntry, Report<TiffError>> {
        self.entries
            .iter()
            .find(|entry| entry.tag == tag.code())
            .copied()
            .ok_or_else(|| Report::new(TiffError::MissingTag).attach("tag", tag))
    }

    /// Get the single value of one entry.
    ///
    /// # Errors
    ///
    /// - IF the directory has no such tag
    /// - IF the entry holds more or fewer than one value
    /// - IF the type is neither `SHORT` nor `LONG`
    pub fn get_scalar(&self, tag: TiffTag) -> Result<u32, Report<TiffError>> {
        let entry = self.get(tag)?;
        if entry.count != 1 {
            return Err(Report::new(TiffError::NotScalar)
                .attach("tag", tag)
                .attach("count", entry.count));
        }
        match entry.kind {
            TiffEntryKind::Short | TiffEntryKind::Long => Ok(entry.value),
            kind => Err(Report::new(TiffError::UnsupportedType)
                .attach("tag", tag)
                .attach("kind", kind)),
        }
    }

    /// Get every value of one entry.
    ///
    /// # Errors
    ///
    /// - IF the directory has no such tag
    /// - IF the type is neither `SHORT` nor `LONG`
    /// - IF the values lie outside the file
    pub fn get_array(&self, tiff: &TiffBytes, tag: TiffTag) -> Result<Vec<u32>, Report<TiffError>> {
        self.get(tag)?.get_values(tiff)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tiff_fixture::TiffFixture;
    use super::*;

    #[test]
    fn tiff_tags_get_scalar() {
        // Arrange
        let tags = tags(TiffFixture::new());
        // Act
        let width = tags.get_scalar(TiffTag::ImageWidth);
        let format = tags.get_scalar(TiffTag::SampleFormat);
        // Assert
        assert_eq!(width.expect("should read tag"), 256);
        assert_eq!(format.expect("should read tag"), 3);
    }

    #[test]
    fn tiff_tags_get_missing() {
        // Arrange
        let tags = tags(TiffFixture::new().without(TiffTag::Compression));
        // Act
        let output = tags.get(TiffTag::Compression);
        // Assert
        let report = output.expect_err("should reject tag");
        assert_eq!(*report.current_context(), TiffError::MissingTag);
    }

    /// Four tiles do not fit the four byte value field, so they lie out of line.
    #[test]
    fn tiff_tags_get_array() {
        // Arrange
        let bytes = TiffFixture::new().to_bytes();
        let tiff = TiffBytes::new(bytes).expect("should read header");
        let tags = tiff.get_tags().expect("should read directory");
        // Act
        let offsets = tags.get_array(&tiff, TiffTag::TileOffsets);
        let counts = tags.get_array(&tiff, TiffTag::TileByteCounts);
        // Assert
        assert_eq!(offsets.expect("should read tag").len(), 4);
        assert_eq!(counts.expect("should read tag").len(), 4);
    }

    /// One tile fits the four byte value field, so it lies inline.
    #[test]
    fn tiff_tags_get_array_inline() {
        // Arrange
        let bytes = TiffFixture::new().with_tile_size(256).to_bytes();
        let tiff = TiffBytes::new(bytes).expect("should read header");
        let tags = tiff.get_tags().expect("should read directory");
        // Act
        let offsets = tags.get_array(&tiff, TiffTag::TileOffsets);
        // Assert
        assert_eq!(offsets.expect("should read tag").len(), 1);
    }

    /// Read the tag directory of a fixture.
    fn tags(fixture: TiffFixture) -> TiffTags {
        let tiff = TiffBytes::new(fixture.to_bytes()).expect("should read header");
        tiff.get_tags().expect("should read directory")
    }
}
