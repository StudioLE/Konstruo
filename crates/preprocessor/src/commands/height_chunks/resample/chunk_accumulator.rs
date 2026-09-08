//! Resample cells into the vertices of one chunk.

use crate::prelude::*;
use std::ops::Range;

/// Resample cells into the vertices of one chunk.
///
/// - Keeps sums and counts rather than heights, so a vertex drawing on three
///   cells and one drawing on four are not averaged together
pub struct ChunkAccumulator {
    /// Interval between vertices.
    spacing: Spacing,
    /// Position of the chunk.
    index: ChunkIndex,
    /// Sum of the surveyed cells of each vertex.
    sums: Vec<f32>,
    /// Number of surveyed cells of each vertex.
    counts: Vec<u16>,
}

impl ChunkAccumulator {
    /// Create a new [`ChunkAccumulator`].
    #[must_use]
    pub fn new(spacing: Spacing, index: ChunkIndex) -> Self {
        let vertices = spacing.vertices_across().pow(2);
        Self {
            spacing,
            index,
            sums: vec![0.0; vertices],
            counts: vec![0; vertices],
        }
    }

    /// Add every cell of a source file that falls within a vertex of this chunk.
    ///
    /// - Cells reading as [`NO_DATA`] are excluded from both the sum and the count
    pub fn add(&mut self, chunk: &CellHeightChunk) {
        // TODO: Hoist the column ranges out of the row loop IF `time_accumulate` dominates
        let across = self.spacing.vertices_across();
        let meters = f64::from(self.spacing.meters());
        let half = meters / 2.0;
        let west = f64::from(self.index.x * CHUNK_SIZE);
        let north = f64::from(self.index.y * CHUNK_SIZE + CHUNK_SIZE);
        for row in 0..across {
            let northing = north - offset(row) * meters;
            let rows = get_range(
                chunk.bounds.northing - 0.5 - northing - half,
                chunk.bounds.northing - 0.5 - northing + half,
                chunk.bounds.rows,
            );
            if rows.is_empty() {
                continue;
            }
            for column in 0..across {
                let easting = west + offset(column) * meters;
                let columns = get_range(
                    easting - half - chunk.bounds.easting - 0.5,
                    easting + half - chunk.bounds.easting - 0.5,
                    chunk.bounds.columns,
                );
                self.add_cells(chunk, row * across + column, rows.clone(), columns);
            }
        }
    }

    /// Create a [`HeightChunk`] of the mean height of every vertex.
    ///
    /// - Returns [`None`] IF no vertex has a surveyed cell
    #[must_use]
    pub fn finish(self) -> Option<HeightChunk> {
        if self.counts.iter().all(|count| *count == 0) {
            return None;
        }
        let heights = self
            .sums
            .iter()
            .zip(&self.counts)
            .map(|(sum, count)| mean(*sum, *count))
            .collect();
        let chunk = HeightChunk::new(self.spacing, self.index, heights)
            .expect("accumulator holds one value per vertex");
        Some(chunk)
    }

    /// Add every surveyed cell of one region to one vertex.
    fn add_cells(
        &mut self,
        chunk: &CellHeightChunk,
        vertex: usize,
        rows: Range<usize>,
        columns: Range<usize>,
    ) {
        for row in rows {
            for column in columns.clone() {
                let Some(height) = chunk.get_cell(row, column) else {
                    continue;
                };
                if height.is_no_data() {
                    continue;
                }
                let Some(sum) = self.sums.get_mut(vertex) else {
                    continue;
                };
                *sum += height;
                let Some(count) = self.counts.get_mut(vertex) else {
                    continue;
                };
                *count += 1;
            }
        }
    }
}

/// Mean of the surveyed cells of one vertex.
fn mean(sum: f32, count: u16) -> f32 {
    if count == 0 {
        return NO_DATA;
    }
    sum / f32::from(count)
}

/// Offset of a vertex from the west or north edge of its chunk, in spacings.
///
/// - Column and row `1` sit on the edge, `0` one vertex beyond it
#[expect(
    clippy::as_conversions,
    clippy::cast_precision_loss,
    reason = "counts are small"
)]
fn offset(vertex: usize) -> f64 {
    vertex as f64 - 1.0
}

/// Range of cell indices whose sample point falls within a half-open span.
///
/// - Sample points sit at half meters and spacings are even, so a span never
///   ends on a sample point and both ends round the same way
#[expect(
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    reason = "clamped to the cell count"
)]
fn get_range(from: f64, to: f64, limit: usize) -> Range<usize> {
    let limit = limit as f64;
    let from = from.ceil().clamp(0.0, limit);
    let to = to.ceil().clamp(from, limit);
    (from as usize)..(to as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spacing with the fewest vertices, so tests stay small.
    const SPACING: Spacing = Spacing::Sixteen;

    /// Column of the vertex on the east edge of a chunk at [`SPACING`].
    const EAST: usize = 33;

    /// A vertex over uniform cells reads as that height.
    #[test]
    fn chunk_accumulator_add() {
        // Arrange
        let chunk = mock(vec![10.0; 512 * 512]);
        let mut accumulator = ChunkAccumulator::new(SPACING, ChunkIndex::new(0, 0));
        // Act
        accumulator.add(&chunk);
        // Assert
        let output = accumulator.finish().expect("should have heights");
        assert_eq!(output.get_height(1, 1), Some(10.0));
        assert_eq!(output.get_height(17, 17), Some(10.0));
    }

    /// A vertex beyond the surveyed cells reads as missing.
    #[test]
    fn chunk_accumulator_add_border() {
        // Arrange
        let chunk = mock(vec![10.0; 512 * 512]);
        let mut accumulator = ChunkAccumulator::new(SPACING, ChunkIndex::new(0, 0));
        // Act
        accumulator.add(&chunk);
        // Assert
        let output = accumulator.finish().expect("should have heights");
        assert_eq!(output.get_height(0, 0), Some(NO_DATA));
        assert_eq!(output.get_height(34, 34), Some(NO_DATA));
    }

    /// A vertex over two distinct heights reads as their unweighted mean.
    #[test]
    fn chunk_accumulator_add_mean() {
        // Arrange
        let cells = (0..512 * 512)
            .map(|index| if index % 2 == 0 { 0.0 } else { 10.0 })
            .collect();
        let chunk = mock(cells);
        let mut accumulator = ChunkAccumulator::new(SPACING, ChunkIndex::new(0, 0));
        // Act
        accumulator.add(&chunk);
        // Assert
        let output = accumulator.finish().expect("should have heights");
        assert_eq!(output.get_height(17, 17), Some(5.0));
    }

    /// A vertex over half missing cells reads as the mean of the surveyed half.
    #[test]
    fn chunk_accumulator_add_partial() {
        // Arrange
        let cells = (0..512 * 512)
            .map(|index| if index % 2 == 0 { NO_DATA } else { 10.0 })
            .collect();
        let chunk = mock(cells);
        let mut accumulator = ChunkAccumulator::new(SPACING, ChunkIndex::new(0, 0));
        // Act
        accumulator.add(&chunk);
        // Assert
        let output = accumulator.finish().expect("should have heights");
        assert_eq!(output.get_height(17, 17), Some(10.0));
    }

    /// A chunk of entirely missing cells is not written.
    #[test]
    fn chunk_accumulator_finish_empty() {
        // Arrange
        let chunk = mock(vec![NO_DATA; 512 * 512]);
        let mut accumulator = ChunkAccumulator::new(SPACING, ChunkIndex::new(0, 0));
        // Act
        accumulator.add(&chunk);
        // Assert
        assert!(accumulator.finish().is_none());
    }

    /// A file east of a chunk still reaches its outermost border vertex.
    #[test]
    fn chunk_accumulator_add_neighbour() {
        // Arrange
        let bounds = CellHeightChunkBounds::new(512.0, 512.0, 512, 512);
        let chunk = CellHeightChunk::new(bounds, vec![10.0; 512 * 512]);
        let mut accumulator = ChunkAccumulator::new(SPACING, ChunkIndex::new(0, 0));
        // Act
        accumulator.add(&chunk);
        // Assert
        let output = accumulator.finish().expect("should have heights");
        assert_eq!(output.get_height(17, EAST + 1), Some(10.0));
        assert_eq!(output.get_height(17, EAST - 1), Some(NO_DATA));
    }

    /// Create a [`CellHeightChunk`] covering chunk `(0, 0)` exactly.
    fn mock(cells: Vec<f32>) -> CellHeightChunk {
        let bounds = CellHeightChunkBounds::new(0.0, 512.0, 512, 512);
        CellHeightChunk::new(bounds, cells)
    }
}
