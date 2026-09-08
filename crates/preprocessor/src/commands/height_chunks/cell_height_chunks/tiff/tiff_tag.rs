//! Tags this reader knows how to read.

use std::fmt::{Display, Formatter, Result as FmtResult};

/// Tags this reader knows how to read.
///
/// - Named so errors report the tag rather than its code
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum TiffTag {
    /// Number of columns in the image.
    ImageWidth = 256,
    /// Number of rows in the image.
    ImageLength = 257,
    /// Number of bits in one sample.
    BitsPerSample = 258,
    /// Compression applied to the image data.
    Compression = 259,
    /// Number of samples per pixel.
    SamplesPerPixel = 277,
    /// Layout of the samples of a pixel.
    PlanarConfiguration = 284,
    /// Predictor applied before compression.
    Predictor = 317,
    /// Number of columns in one tile.
    TileWidth = 322,
    /// Number of rows in one tile.
    TileLength = 323,
    /// Offset of each tile from the start of the file.
    TileOffsets = 324,
    /// Compressed length of each tile.
    TileByteCounts = 325,
    /// Numeric format of the samples.
    SampleFormat = 339,
    /// Size of one cell in model space.
    ModelPixelScale = 33550,
    /// Model space coordinates of one raster point.
    ModelTiepoint = 33922,
}

impl TiffTag {
    /// Code identifying this tag within a directory entry.
    #[expect(clippy::as_conversions, reason = "reads the discriminant")]
    #[must_use]
    pub const fn code(self) -> u16 {
        self as u16
    }
}

impl Display for TiffTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let name = match *self {
            Self::ImageWidth => "ImageWidth",
            Self::ImageLength => "ImageLength",
            Self::BitsPerSample => "BitsPerSample",
            Self::Compression => "Compression",
            Self::SamplesPerPixel => "SamplesPerPixel",
            Self::PlanarConfiguration => "PlanarConfiguration",
            Self::Predictor => "Predictor",
            Self::TileWidth => "TileWidth",
            Self::TileLength => "TileLength",
            Self::TileOffsets => "TileOffsets",
            Self::TileByteCounts => "TileByteCounts",
            Self::SampleFormat => "SampleFormat",
            Self::ModelPixelScale => "ModelPixelScale",
            Self::ModelTiepoint => "ModelTiepoint",
        };
        f.write_str(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiff_tag_code() {
        assert_eq!(TiffTag::ImageWidth.code(), 256);
        assert_eq!(TiffTag::TileOffsets.code(), 324);
        assert_eq!(TiffTag::ModelTiepoint.code(), 33922);
    }

    #[test]
    fn tiff_tag_display() {
        assert_eq!(TiffTag::SampleFormat.to_string(), "SampleFormat");
    }
}
