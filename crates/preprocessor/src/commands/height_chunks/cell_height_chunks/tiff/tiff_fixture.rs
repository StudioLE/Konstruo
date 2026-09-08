//! Assemble a tiled little-endian `GeoTIFF` in memory.

use super::tiff_tag::TiffTag;
use weezl::encode::Encoder;
use weezl::BitOrder;

/// Entry type code of a sixteen bit unsigned integer.
const SHORT: u16 = 3;

/// Entry type code of a thirty two bit unsigned integer.
const LONG: u16 = 4;

/// Entry type code of a sixty four bit float.
const DOUBLE: u16 = 12;

/// Byte length of the six doubles of `ModelTiepoint`.
const TIEPOINT_BYTES: usize = 48;

/// Byte length of the three doubles of `ModelPixelScale`.
const PIXEL_SCALE_BYTES: usize = 24;

/// Assemble a tiled little-endian `GeoTIFF` in memory.
///
/// - Encodes tile data exactly as the source files do, so the reader is
///   exercised rather than mocked
pub(crate) struct TiffFixture {
    /// Number of columns in the image.
    columns: usize,
    /// Number of rows in the image.
    rows: usize,
    /// Side length of one tile, in cells.
    tile_size: usize,
    /// Easting of the north west corner in EPSG:27700.
    easting: f64,
    /// Northing of the north west corner in EPSG:27700.
    northing: f64,
    /// Size of one cell in model space, easting then northing.
    pixel_scale: (f64, f64),
    /// Cells, row major, north row first.
    cells: Vec<f32>,
    /// Scalar tags to write instead of the default value.
    overrides: Vec<(TiffTag, u32)>,
    /// Tags to leave out of the directory.
    omitted: Vec<TiffTag>,
    /// Write `ModelTiepoint` before the tag directory?
    ///
    /// Default: `false`
    tiepoint_first: bool,
}

impl TiffFixture {
    /// Create a new [`TiffFixture`] of 256 x 256 cells in four tiles.
    pub(crate) fn new() -> Self {
        let columns = 256;
        let rows = 256;
        Self {
            columns,
            rows,
            tile_size: 128,
            easting: 0.0,
            northing: 0.0,
            pixel_scale: (1.0, 1.0),
            cells: vec![0.0; columns * rows],
            overrides: Vec::new(),
            omitted: Vec::new(),
            tiepoint_first: false,
        }
    }

    /// Set the side length of one tile.
    pub(crate) fn with_tile_size(mut self, tile_size: usize) -> Self {
        self.tile_size = tile_size;
        self
    }

    /// Set the north west corner in EPSG:27700.
    pub(crate) fn with_tiepoint(mut self, easting: f64, northing: f64) -> Self {
        self.easting = easting;
        self.northing = northing;
        self
    }

    /// Set the size of one cell in model space.
    pub(crate) fn with_pixel_scale(mut self, easting: f64, northing: f64) -> Self {
        self.pixel_scale = (easting, northing);
        self
    }

    /// Set every cell of the image.
    pub(crate) fn with_cells(mut self, cells: Vec<f32>) -> Self {
        assert_eq!(cells.len(), self.columns * self.rows, "cell count");
        self.cells = cells;
        self
    }

    /// Write a scalar tag with a value this reader rejects.
    pub(crate) fn with_scalar(mut self, tag: TiffTag, value: u32) -> Self {
        self.overrides.push((tag, value));
        self
    }

    /// Leave a tag out of the directory.
    pub(crate) fn without(mut self, tag: TiffTag) -> Self {
        self.omitted.push(tag);
        self
    }

    /// Write `ModelTiepoint` before the tag directory rather than after it.
    ///
    /// - The format permits it, so reading from the directory to `EOF` misses it
    pub(crate) fn with_tiepoint_first(mut self) -> Self {
        self.tiepoint_first = true;
        self
    }

    /// Encode the file.
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let tiles = self.encode_tiles();
        let scalars = self.scalars();
        let arrays = [
            TiffTag::TileOffsets,
            TiffTag::TileByteCounts,
            TiffTag::ModelPixelScale,
            TiffTag::ModelTiepoint,
        ];
        let entry_count =
            scalars.len() + arrays.iter().filter(|tag| self.is_written(**tag)).count();
        let directory_bytes = 2 + entry_count * 12 + 4;
        let array_bytes = tiles.len() * 4;
        let is_inline = array_bytes <= 4;
        let (directory_at, tiepoint_at) = if self.tiepoint_first {
            (8 + TIEPOINT_BYTES, 8)
        } else {
            (8, 8 + directory_bytes)
        };
        let scale_at = 8 + directory_bytes + TIEPOINT_BYTES;
        let mut cursor = scale_at + PIXEL_SCALE_BYTES;
        let offsets_at = cursor;
        if !is_inline {
            cursor += array_bytes;
        }
        let counts_at = cursor;
        if !is_inline {
            cursor += array_bytes;
        }
        let mut offsets = Vec::with_capacity(tiles.len());
        for tile in &tiles {
            offsets.push(expect_u32(cursor));
            cursor += tile.len();
        }
        let counts: Vec<u32> = tiles.iter().map(|tile| expect_u32(tile.len())).collect();
        let positions = Positions {
            offsets: offsets_at,
            counts: counts_at,
            scale: scale_at,
            tiepoint: tiepoint_at,
        };
        let mut entries = self.entries(&scalars, &offsets, &counts, is_inline, &positions);
        entries.sort_unstable();
        let directory = encode_directory(&entries);
        let tiepoint = encode_doubles(&[0.0, 0.0, 0.0, self.easting, self.northing, 0.0]);
        let scale = encode_doubles(&[self.pixel_scale.0, self.pixel_scale.1, 0.0]);
        let mut bytes = Vec::with_capacity(cursor);
        bytes.extend_from_slice(b"II");
        push_u16(&mut bytes, 42);
        push_u32(&mut bytes, expect_u32(directory_at));
        if self.tiepoint_first {
            bytes.extend_from_slice(&tiepoint);
            bytes.extend_from_slice(&directory);
        } else {
            bytes.extend_from_slice(&directory);
            bytes.extend_from_slice(&tiepoint);
        }
        bytes.extend_from_slice(&scale);
        if !is_inline {
            for offset in &offsets {
                push_u32(&mut bytes, *offset);
            }
            for count in &counts {
                push_u32(&mut bytes, *count);
            }
        }
        for tile in &tiles {
            bytes.extend_from_slice(tile);
        }
        bytes
    }

    /// Directory entries, in the order the tags were assembled.
    fn entries(
        &self,
        scalars: &[(TiffTag, u16, u32)],
        offsets: &[u32],
        counts: &[u32],
        is_inline: bool,
        positions: &Positions,
    ) -> Vec<[u32; 4]> {
        let mut entries: Vec<[u32; 4]> = scalars
            .iter()
            .map(|(tag, kind, value)| [u32::from(tag.code()), u32::from(*kind), 1, *value])
            .collect();
        let tiles = expect_u32(offsets.len());
        if self.is_written(TiffTag::TileOffsets) {
            let value = first_or(offsets, is_inline, expect_u32(positions.offsets));
            entries.push([
                u32::from(TiffTag::TileOffsets.code()),
                u32::from(LONG),
                tiles,
                value,
            ]);
        }
        if self.is_written(TiffTag::TileByteCounts) {
            let value = first_or(counts, is_inline, expect_u32(positions.counts));
            entries.push([
                u32::from(TiffTag::TileByteCounts.code()),
                u32::from(LONG),
                tiles,
                value,
            ]);
        }
        if self.is_written(TiffTag::ModelPixelScale) {
            entries.push([
                u32::from(TiffTag::ModelPixelScale.code()),
                u32::from(DOUBLE),
                3,
                expect_u32(positions.scale),
            ]);
        }
        if self.is_written(TiffTag::ModelTiepoint) {
            entries.push([
                u32::from(TiffTag::ModelTiepoint.code()),
                u32::from(DOUBLE),
                6,
                expect_u32(positions.tiepoint),
            ]);
        }
        entries
    }

    /// Scalar tags and their values.
    fn scalars(&self) -> Vec<(TiffTag, u16, u32)> {
        let defaults = [
            (TiffTag::ImageWidth, LONG, expect_u32(self.columns)),
            (TiffTag::ImageLength, LONG, expect_u32(self.rows)),
            (TiffTag::BitsPerSample, SHORT, 32),
            (TiffTag::Compression, SHORT, 5),
            (TiffTag::SamplesPerPixel, SHORT, 1),
            (TiffTag::PlanarConfiguration, SHORT, 1),
            (TiffTag::Predictor, SHORT, 1),
            (TiffTag::TileWidth, LONG, expect_u32(self.tile_size)),
            (TiffTag::TileLength, LONG, expect_u32(self.tile_size)),
            (TiffTag::SampleFormat, SHORT, 3),
        ];
        defaults
            .into_iter()
            .filter(|(tag, _, _)| self.is_written(*tag))
            .map(|(tag, kind, value)| {
                let value = self
                    .overrides
                    .iter()
                    .find(|(overridden, _)| *overridden == tag)
                    .map_or(value, |(_, overridden)| *overridden);
                (tag, kind, value)
            })
            .collect()
    }

    /// Encode every tile, left to right then top to bottom.
    fn encode_tiles(&self) -> Vec<Vec<u8>> {
        let across = self.columns.div_ceil(self.tile_size);
        let down = self.rows.div_ceil(self.tile_size);
        let mut tiles = Vec::with_capacity(across * down);
        for tile_row in 0..down {
            for tile_column in 0..across {
                let mut raw = Vec::with_capacity(self.tile_size * self.tile_size * 4);
                for row in 0..self.tile_size {
                    for column in 0..self.tile_size {
                        let cell = self.get_cell(
                            tile_row * self.tile_size + row,
                            tile_column * self.tile_size + column,
                        );
                        raw.extend_from_slice(&cell.to_le_bytes());
                    }
                }
                let tile = Encoder::with_tiff_size_switch(BitOrder::Msb, 8)
                    .encode(&raw)
                    .expect("should encode tile");
                tiles.push(tile);
            }
        }
        tiles
    }

    /// Get one cell, padding beyond the image with zero.
    fn get_cell(&self, row: usize, column: usize) -> f32 {
        if row >= self.rows || column >= self.columns {
            return 0.0;
        }
        self.cells
            .get(row * self.columns + column)
            .copied()
            .unwrap_or_default()
    }

    /// Is a tag written to the directory?
    fn is_written(&self, tag: TiffTag) -> bool {
        !self.omitted.contains(&tag)
    }
}

/// Byte position of each value written out of line.
struct Positions {
    /// Position of the tile offsets array.
    offsets: usize,
    /// Position of the tile byte counts array.
    counts: usize,
    /// Position of `ModelPixelScale`.
    scale: usize,
    /// Position of `ModelTiepoint`.
    tiepoint: usize,
}

/// Encode little-endian [`f64`].
fn encode_doubles(values: &[f64]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(values.len() * 8);
    for value in values {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    bytes
}

/// Encode the tag directory, which counts its entries then terminates with zero.
fn encode_directory(entries: &[[u32; 4]]) -> Vec<u8> {
    let mut directory = Vec::with_capacity(2 + entries.len() * 12 + 4);
    push_u16(
        &mut directory,
        u16::try_from(entries.len()).expect("should fit"),
    );
    for entry in entries {
        let [tag, kind, count, value] = *entry;
        push_u16(&mut directory, u16::try_from(tag).expect("should fit"));
        push_u16(&mut directory, u16::try_from(kind).expect("should fit"));
        push_u32(&mut directory, count);
        push_u32(&mut directory, value);
    }
    push_u32(&mut directory, 0);
    directory
}

/// Convert an index to the [`u32`] TIFF writes it as.
///
/// # Panics
///
/// - Panics IF the value exceeds [`u32`]
fn expect_u32(value: usize) -> u32 {
    u32::try_from(value).expect("should fit")
}

/// Get the first value when it lies inline, otherwise the offset of the array.
fn first_or(values: &[u32], is_inline: bool, offset: u32) -> u32 {
    if is_inline {
        values.first().copied().unwrap_or_default()
    } else {
        offset
    }
}

/// Append a little-endian [`u16`].
fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

/// Append a little-endian [`u32`].
fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}
