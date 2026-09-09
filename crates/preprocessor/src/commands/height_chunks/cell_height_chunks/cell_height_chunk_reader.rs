//! Read source files of pixel-registered height cells.

use super::tiff::{TiffBytes, TiffEntryKind, TiffTag, TiffTags};
use crate::prelude::*;
use std::fs::{read, File};
use std::io::{Read as _, Seek, SeekFrom};
use weezl::decode::Decoder;
use weezl::BitOrder;

/// Byte length of the header, which every TIFF opens with.
const HEADER_BYTES: usize = 8;

/// Byte offset of the tag directory position within the header.
const DIRECTORY_POSITION: usize = 4;

/// Byte offset of the easting within `ModelTiepoint`.
const TIEPOINT_EASTING: usize = 24;

/// Byte offset of the northing within `ModelTiepoint`.
const TIEPOINT_NORTHING: usize = 32;

/// Encodings this reader handles, by tag and the only value it accepts.
const SUPPORTED: [(TiffTag, u32); 6] = [
    (TiffTag::BitsPerSample, 32),
    (TiffTag::Compression, 5),
    (TiffTag::SamplesPerPixel, 1),
    (TiffTag::PlanarConfiguration, 1),
    (TiffTag::Predictor, 1),
    (TiffTag::SampleFormat, 3),
];

/// Read source files of pixel-registered height cells.
#[derive(Default, FromServices)]
pub struct CellHeightChunkReader;

impl CellHeightChunkReader {
    /// Read the extent of a source file without decoding it.
    ///
    /// - Reads only the tag directory and the values after it, which the source
    ///   files place within 15 KB of `EOF`
    /// - Falls back to reading every byte IF a value lies before the directory
    /// - Validates the encoding, so [`CellHeightChunkReader::read`] can assume it
    ///
    /// # Errors
    ///
    /// - IF the file cannot be read or parsed
    /// - IF the file uses an encoding this reader does not handle
    /// - IF `ModelTiepoint` is missing or malformed
    #[expect(clippy::unused_self, reason = "resolved as a service")]
    pub fn read_bounds(
        &self,
        path: &Path,
    ) -> Result<CellHeightChunkBounds, Report<CellHeightChunkError>> {
        match Self::read_bounds_from_tail(path) {
            Ok(bounds) => Ok(bounds),
            Err(report) => {
                trace!(path = %path.display(), "{}", report.render());
                let tiff = Self::get_tiff(path)?;
                let tags = Self::get_tags(&tiff, path)?;
                Self::validate(&tiff, &tags).attach_path(path)?;
                Self::get_bounds(&tiff, &tags).attach_path(path)
            }
        }
    }

    /// Read the extent from the tag directory and the values after it.
    ///
    /// # Errors
    ///
    /// - IF the file cannot be read or parsed
    /// - IF a value the extent needs lies before the directory
    fn read_bounds_from_tail(
        path: &Path,
    ) -> Result<CellHeightChunkBounds, Report<CellHeightChunkError>> {
        let mut file = File::open(path)
            .change_context(CellHeightChunkError::Read)
            .attach_path(path)?;
        let mut header = [0; HEADER_BYTES];
        file.read_exact(&mut header)
            .change_context(CellHeightChunkError::Read)
            .attach_path(path)?;
        let header = TiffBytes::new(header.to_vec())
            .change_context(CellHeightChunkError::Tiff)
            .attach_path(path)?;
        let at = widen(
            header
                .get_u32(DIRECTORY_POSITION)
                .change_context(CellHeightChunkError::Tiff)?,
        )?;
        let start = u64::try_from(at).change_context(CellHeightChunkError::Offset)?;
        file.seek(SeekFrom::Start(start))
            .change_context(CellHeightChunkError::Read)
            .attach_path(path)
            .attach("at", at)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .change_context(CellHeightChunkError::Read)
            .attach_path(path)?;
        let tiff = TiffBytes::new_at(bytes, at);
        let tags = tiff
            .get_tags_at(at)
            .change_context(CellHeightChunkError::Tiff)
            .attach_path(path)?;
        Self::validate(&tiff, &tags).attach_path(path)?;
        Self::get_bounds(&tiff, &tags).attach_path(path)
    }

    /// Read and decode a source file.
    ///
    /// - Assumes [`CellHeightChunkReader::read_bounds`] already accepted the encoding
    ///
    /// # Errors
    ///
    /// - IF the file cannot be read or parsed
    /// - IF a tile cannot be decoded
    #[expect(clippy::unused_self, reason = "resolved as a service")]
    pub fn read(&self, path: &Path) -> Result<CellHeightChunk, Report<CellHeightChunkError>> {
        let file_name = path.file_name().unwrap_or_default().to_string_lossy();
        trace!(%file_name, "Reading cell height chunk");
        let mut time_get_tiff = Timer::new();
        let tiff = Self::get_tiff(path)?;
        time_get_tiff.stop();
        let tags = Self::get_tags(&tiff, path)?;
        let bounds = Self::get_bounds(&tiff, &tags).attach_path(path)?;
        let mut time_decode = Timer::new();
        let cells = Self::decode(&tiff, &tags, bounds).attach_path(path)?;
        time_decode.stop();
        trace!(
            %file_name,
            get_tiff = %time_get_tiff,
            decode = %time_decode,
            "Read cell height chunk"
        );
        Ok(CellHeightChunk::new(bounds, cells))
    }

    /// Read every byte of a source file.
    fn get_tiff(path: &Path) -> Result<TiffBytes, Report<CellHeightChunkError>> {
        let bytes = read(path)
            .change_context(CellHeightChunkError::Read)
            .attach_path(path)?;
        TiffBytes::new(bytes)
            .change_context(CellHeightChunkError::Tiff)
            .attach_path(path)
    }

    /// Read the tag directory of a source file.
    fn get_tags(tiff: &TiffBytes, path: &Path) -> Result<TiffTags, Report<CellHeightChunkError>> {
        tiff.get_tags()
            .change_context(CellHeightChunkError::Tiff)
            .attach_path(path)
    }

    /// Reject any file whose encoding differs from the one this reader targets.
    fn validate(tiff: &TiffBytes, tags: &TiffTags) -> Result<(), Report<CellHeightChunkError>> {
        for (tag, expected) in SUPPORTED {
            let found = tags
                .get_scalar(tag)
                .change_context(CellHeightChunkError::Tiff)?;
            if found != expected {
                return Err(Report::new(CellHeightChunkError::Unsupported)
                    .attach("tag", tag)
                    .attach("expected", expected)
                    .attach("found", found));
            }
        }
        Self::validate_scale(tiff, tags)
    }

    /// Reject any file whose cells are not one meter.
    ///
    /// - Every other reader assumes one meter, so a differing scale would
    ///   silently misregister the file rather than fail
    fn validate_scale(
        tiff: &TiffBytes,
        tags: &TiffTags,
    ) -> Result<(), Report<CellHeightChunkError>> {
        let entry = tags
            .get(TiffTag::ModelPixelScale)
            .change_context(CellHeightChunkError::Tiff)?;
        if entry.count != 3 || entry.kind != TiffEntryKind::Double {
            return Err(Report::new(CellHeightChunkError::PixelScale)
                .attach("count", entry.count)
                .attach("kind", entry.kind));
        }
        let at = widen(entry.value)?;
        for (axis, offset) in [("easting", 0), ("northing", 8)] {
            let scale = tiff
                .get_f64(at + offset)
                .change_context(CellHeightChunkError::Tiff)?;
            if (scale - 1.0).abs() > f64::EPSILON {
                return Err(Report::new(CellHeightChunkError::PixelScale)
                    .attach("axis", axis)
                    .attach("scale", scale));
            }
        }
        Ok(())
    }

    /// Read the extent, converting EPSG:27700 to world space.
    fn get_bounds(
        tiff: &TiffBytes,
        tags: &TiffTags,
    ) -> Result<CellHeightChunkBounds, Report<CellHeightChunkError>> {
        let columns = Self::get_count(tags, TiffTag::ImageWidth)?;
        let rows = Self::get_count(tags, TiffTag::ImageLength)?;
        let entry = tags
            .get(TiffTag::ModelTiepoint)
            .change_context(CellHeightChunkError::Tiff)?;
        if entry.count != 6 || entry.kind != TiffEntryKind::Double {
            return Err(Report::new(CellHeightChunkError::Tiepoint)
                .attach("count", entry.count)
                .attach("kind", entry.kind));
        }
        let at = widen(entry.value)?;
        let easting = tiff
            .get_f64(at + TIEPOINT_EASTING)
            .change_context(CellHeightChunkError::Tiff)?;
        let northing = tiff
            .get_f64(at + TIEPOINT_NORTHING)
            .change_context(CellHeightChunkError::Tiff)?;
        Ok(CellHeightChunkBounds::new(
            easting - BngCoordinates::ORIGIN.easting,
            northing - BngCoordinates::ORIGIN.northing,
            columns,
            rows,
        ))
    }

    /// Decode every tile into one row major raster, north row first.
    fn decode(
        tiff: &TiffBytes,
        tags: &TiffTags,
        bounds: CellHeightChunkBounds,
    ) -> Result<Vec<f32>, Report<CellHeightChunkError>> {
        trace!("Decoding cell height chunk");
        let tile_columns = Self::get_count(tags, TiffTag::TileWidth)?;
        let tile_rows = Self::get_count(tags, TiffTag::TileLength)?;
        let offsets = Self::get_array(tiff, tags, TiffTag::TileOffsets)?;
        let counts = Self::get_array(tiff, tags, TiffTag::TileByteCounts)?;
        let across = bounds.columns.div_ceil(tile_columns);
        let down = bounds.rows.div_ceil(tile_rows);
        if offsets.len() != across * down || counts.len() != offsets.len() {
            return Err(Report::new(CellHeightChunkError::Tiles)
                .attach("expected", across * down)
                .attach("found", offsets.len()));
        }
        let mut cells = vec![NO_DATA; bounds.columns * bounds.rows];
        let mut time_lzw = Timer::new_stopped();
        let mut time_copy = Timer::new_stopped();
        for (index, (offset, length)) in offsets.iter().zip(&counts).enumerate() {
            let raw = tiff
                .get_slice(widen(*offset)?, widen(*length)?)
                .change_context(CellHeightChunkError::Tiff)
                .attach("tile", index)?;
            time_lzw.resume();
            let tile = Decoder::with_tiff_size_switch(BitOrder::Msb, 8)
                .decode(raw)
                .change_context(CellHeightChunkError::Decode)
                .attach("tile", index)?;
            time_lzw.stop();
            let expected = tile_columns * tile_rows * 4;
            if tile.len() != expected {
                return Err(Report::new(CellHeightChunkError::Tile)
                    .attach("tile", index)
                    .attach("expected", expected)
                    .attach("found", tile.len()));
            }
            let at = Tile::new(
                index.div_euclid(across) * tile_rows,
                index.rem_euclid(across) * tile_columns,
                tile_columns,
                tile_rows,
            );
            time_copy.resume();
            at.copy_into(&tile, &mut cells, bounds);
            time_copy.stop();
        }
        trace!(
            tiles = offsets.len(),
            lzw = %time_lzw,
            copy = %time_copy,
            "Decoded cell height chunk"
        );
        Ok(cells)
    }

    /// Get a count held by a scalar tag.
    fn get_count(tags: &TiffTags, tag: TiffTag) -> Result<usize, Report<CellHeightChunkError>> {
        let value = tags
            .get_scalar(tag)
            .change_context(CellHeightChunkError::Tiff)?;
        widen(value)
    }

    /// Get every value of an array tag.
    fn get_array(
        tiff: &TiffBytes,
        tags: &TiffTags,
        tag: TiffTag,
    ) -> Result<Vec<u32>, Report<CellHeightChunkError>> {
        tags.get_array(tiff, tag)
            .change_context(CellHeightChunkError::Tiff)
            .attach("tag", tag)
    }
}

/// Position and size of one tile within the raster.
struct Tile {
    /// Row of the north west cell of the tile within the raster.
    north: usize,
    /// Column of the north west cell of the tile within the raster.
    west: usize,
    /// Number of columns in the tile.
    columns: usize,
    /// Number of rows in the tile.
    rows: usize,
}

impl Tile {
    /// Create a new [`Tile`].
    const fn new(north: usize, west: usize, columns: usize, rows: usize) -> Self {
        Self {
            north,
            west,
            columns,
            rows,
        }
    }

    /// Copy this decoded tile into the raster, discarding cells beyond its edges.
    fn copy_into(&self, tile: &[u8], cells: &mut [f32], bounds: CellHeightChunkBounds) {
        for row in 0..self.rows {
            let target_row = self.north + row;
            if target_row >= bounds.rows {
                break;
            }
            for column in 0..self.columns {
                let target_column = self.west + column;
                if target_column >= bounds.columns {
                    break;
                }
                let source = (row * self.columns + column) * 4;
                let Some(bytes) = tile.get(source..source + 4) else {
                    break;
                };
                let Ok(bytes) = <[u8; 4]>::try_from(bytes) else {
                    break;
                };
                let Some(cell) = cells.get_mut(target_row * bounds.columns + target_column) else {
                    break;
                };
                *cell = f32::from_le_bytes(bytes);
            }
        }
    }
}

/// Widen a TIFF offset or count to an index.
fn widen(value: u32) -> Result<usize, Report<CellHeightChunkError>> {
    usize::try_from(value).change_context(CellHeightChunkError::Offset)
}

/// Errors returned by [`CellHeightChunkReader`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CellHeightChunkError {
    /// Unable to read source file.
    #[error("Unable to read source file")]
    Read,
    /// Unable to parse source file.
    #[error("Unable to parse source file")]
    Tiff,
    /// Unsupported encoding.
    #[error("Unsupported encoding")]
    Unsupported,
    /// Missing or malformed `ModelTiepoint`.
    #[error("Missing or malformed ModelTiepoint")]
    Tiepoint,
    /// Cells are not one meter.
    #[error("Cells are not one meter")]
    PixelScale,
    /// Offset exceeds the address space.
    #[error("Offset exceeds the address space")]
    Offset,
    /// Unexpected number of tiles.
    #[error("Unexpected number of tiles")]
    Tiles,
    /// Unexpected tile length.
    #[error("Unexpected tile length")]
    Tile,
    /// Unable to decode tile.
    #[error("Unable to decode tile")]
    Decode,
}

#[cfg(test)]
mod tests {
    use super::super::tiff::tiff_fixture::TiffFixture;
    use super::*;
    use std::fs::write;
    use tempfile::{tempdir, TempDir};

    /// Easting of the fixture in EPSG:27700, one chunk east of the origin.
    const EASTING: f64 = BngCoordinates::ORIGIN.easting + CHUNK_SIZE_F64;

    /// Northing of the fixture in EPSG:27700, at the origin.
    const NORTHING: f64 = BngCoordinates::ORIGIN.northing;

    #[test]
    fn cell_height_chunk_reader_read_bounds() {
        // Arrange
        let (_directory, path) = write_fixture(TiffFixture::new().with_tiepoint(EASTING, NORTHING));
        // Act
        let output = CellHeightChunkReader.read_bounds(&path);
        // Assert
        let bounds = output.expect("should read bounds");
        assert_eq!(bounds, CellHeightChunkBounds::new(512.0, 0.0, 256, 256));
    }

    /// The tail holds the tiepoint, so the extent is read without the whole file.
    #[test]
    fn cell_height_chunk_reader_read_bounds_from_tail() {
        // Arrange
        let (_directory, path) = write_fixture(TiffFixture::new().with_tiepoint(EASTING, NORTHING));
        // Act
        let output = CellHeightChunkReader::read_bounds_from_tail(&path);
        // Assert
        let bounds = output.expect("should read bounds");
        assert_eq!(bounds, CellHeightChunkBounds::new(512.0, 0.0, 256, 256));
    }

    /// The tail starts after the tiepoint, so it cannot supply the extent alone.
    #[test]
    fn cell_height_chunk_reader_read_bounds_from_tail_tiepoint_first() {
        // Arrange
        let (_directory, path) = write_fixture(
            TiffFixture::new()
                .with_tiepoint(EASTING, NORTHING)
                .with_tiepoint_first(),
        );
        // Act
        let output = CellHeightChunkReader::read_bounds_from_tail(&path);
        // Assert
        let report = output.expect_err("should reject tail");
        assert_eq!(*report.current_context(), CellHeightChunkError::Tiff);
    }

    /// The tiepoint lies before the directory, so the tail read falls back to a full read.
    #[test]
    fn cell_height_chunk_reader_read_bounds_tiepoint_first() {
        // Arrange
        let (_directory, path) = write_fixture(
            TiffFixture::new()
                .with_tiepoint(EASTING, NORTHING)
                .with_tiepoint_first(),
        );
        // Act
        let output = CellHeightChunkReader.read_bounds(&path);
        // Assert
        let bounds = output.expect("should read bounds");
        assert_eq!(bounds, CellHeightChunkBounds::new(512.0, 0.0, 256, 256));
    }

    /// Every cell survives the tiled round trip in the order it was written.
    #[test]
    fn cell_height_chunk_reader_read() {
        // Arrange
        let cells: Vec<f32> = (0..65_536)
            .map(|index| f32::from(u16::try_from(index % 97).expect("should fit")))
            .collect();
        let (_directory, path) = write_fixture(
            TiffFixture::new()
                .with_tiepoint(EASTING, NORTHING)
                .with_cells(cells.clone()),
        );
        // Act
        let output = CellHeightChunkReader.read(&path);
        // Assert
        let chunk = output.expect("should read chunk");
        let read: Vec<f32> = (0..256)
            .flat_map(|row| (0..256).map(move |column| (row, column)))
            .map(|(row, column)| chunk.get_cell(row, column).expect("should have cell"))
            .collect();
        assert_eq!(read, cells);
    }

    /// An unsupported encoding is rejected during discovery, before any chunk is written.
    #[test]
    fn cell_height_chunk_reader_read_bounds_unsupported() {
        // Arrange
        let (_directory, path) =
            write_fixture(TiffFixture::new().with_scalar(TiffTag::Compression, 1));
        // Act
        let output = CellHeightChunkReader.read_bounds(&path);
        // Assert
        let report = output.expect_err("should reject file");
        assert_eq!(*report.current_context(), CellHeightChunkError::Unsupported);
    }

    /// Cells that are not one meter are rejected rather than misregistered.
    #[test]
    fn cell_height_chunk_reader_read_bounds_pixel_scale() {
        // Arrange
        let (_directory, path) = write_fixture(
            TiffFixture::new()
                .with_tiepoint(EASTING, NORTHING)
                .with_pixel_scale(2.0, 2.0),
        );
        // Act
        let output = CellHeightChunkReader.read_bounds(&path);
        // Assert
        let report = output.expect_err("should reject file");
        assert_eq!(*report.current_context(), CellHeightChunkError::PixelScale);
    }

    #[test]
    fn cell_height_chunk_reader_read_missing_tag() {
        // Arrange
        let (_directory, path) = write_fixture(TiffFixture::new().without(TiffTag::ModelTiepoint));
        // Act
        let output = CellHeightChunkReader.read(&path);
        // Assert
        let report = output.expect_err("should reject file");
        assert_eq!(*report.current_context(), CellHeightChunkError::Tiff);
    }

    /// Write a fixture to a temporary directory.
    fn write_fixture(fixture: TiffFixture) -> (TempDir, PathBuf) {
        let directory = tempdir().expect("should create directory");
        let path = directory.path().join("fixture_DTM_1m.tif");
        write(&path, fixture.to_bytes()).expect("should write file");
        (directory, path)
    }
}
