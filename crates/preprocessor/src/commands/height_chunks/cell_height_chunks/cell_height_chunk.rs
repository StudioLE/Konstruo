//! Pixel-registered height cells of one source file.

use crate::prelude::*;

/// Pixel-registered height cells of one source file.
///
/// - Stores cells row major, north row first, west to east within a row
/// - A cell covers a one meter square, with its sample point at the center
#[derive(Debug)]
pub struct CellHeightChunk {
    /// Extent of the cells in world space.
    pub bounds: CellHeightChunkBounds,
    /// Heights in meters.
    cells: Vec<f32>,
}

impl CellHeightChunk {
    /// Create a new [`CellHeightChunk`].
    #[must_use]
    pub fn new(bounds: CellHeightChunkBounds, cells: Vec<f32>) -> Self {
        Self { bounds, cells }
    }

    /// Get the height of one cell.
    ///
    /// - Returns [`None`] IF either index is beyond the cells
    #[must_use]
    pub fn get_cell(&self, row: usize, column: usize) -> Option<f32> {
        if row >= self.bounds.rows || column >= self.bounds.columns {
            return None;
        }
        self.cells.get(row * self.bounds.columns + column).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_height_chunk_get_cell() {
        // Arrange
        let bounds = CellHeightChunkBounds::new(0.0, 2.0, 2, 2);
        let chunk = CellHeightChunk::new(bounds, vec![1.0, 2.0, 3.0, 4.0]);
        // Act
        let north_west = chunk.get_cell(0, 0);
        let south_east = chunk.get_cell(1, 1);
        let beyond_row = chunk.get_cell(2, 0);
        let beyond_column = chunk.get_cell(0, 2);
        // Assert
        assert_eq!(north_west, Some(1.0));
        assert_eq!(south_east, Some(4.0));
        assert_eq!(beyond_row, None);
        assert_eq!(beyond_column, None);
    }
}
