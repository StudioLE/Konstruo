//! Extent of a source file in world space.

use crate::prelude::*;

/// Extent of a source file in world space.
///
/// - Covers easting `[easting, easting + columns]` and northing
///   `[northing - rows, northing]`
/// - Cells are one meter, with their sample point at the center
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CellHeightChunkBounds {
    /// Easting of the north west corner, in meters.
    pub easting: f64,
    /// Northing of the north west corner, in meters.
    pub northing: f64,
    /// Number of columns.
    pub columns: usize,
    /// Number of rows.
    pub rows: usize,
}

impl CellHeightChunkBounds {
    /// Create a new [`CellHeightChunkBounds`].
    #[must_use]
    pub fn new(easting: f64, northing: f64, columns: usize, rows: usize) -> Self {
        Self {
            easting,
            northing,
            columns,
            rows,
        }
    }

    /// Chunks whose 512 m footprint these cells fall within.
    #[must_use]
    pub fn get_chunks(&self) -> ChunkBounds {
        self.get_bounds(0.0)
    }

    /// Chunks with a vertex these cells contribute to.
    ///
    /// - Reaches `1.5` spacings beyond the chunk, being one border vertex
    ///   plus the half spacing that vertex samples across
    #[must_use]
    pub fn get_contributions(&self, spacing: Spacing) -> ChunkBounds {
        self.get_bounds(f64::from(spacing.meters()) * 1.5)
    }

    /// Chunks these cells reach, expanded by a margin in meters.
    fn get_bounds(&self, margin: f64) -> ChunkBounds {
        let west = self.easting + 0.5 - margin;
        let east = self.easting + length(self.columns) - 0.5 + margin;
        let south = self.northing - length(self.rows) + 0.5 - margin;
        let north = self.northing - 0.5 + margin;
        ChunkBounds::new(
            get_chunk(west),
            get_chunk(east),
            get_chunk(south),
            get_chunk(north),
        )
    }
}

/// Length of a count of one meter cells.
#[expect(
    clippy::as_conversions,
    clippy::cast_precision_loss,
    reason = "counts are small"
)]
fn length(cells: usize) -> f64 {
    cells as f64
}

/// Index of the chunk containing a world coordinate, in meters.
#[expect(
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    reason = "world extent is far within i32"
)]
fn get_chunk(world: f64) -> i32 {
    world.div_euclid(f64::from(CHUNK_SIZE)) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A file over the origin covers the ten chunks each axis of it spans.
    #[test]
    fn cell_height_chunk_bounds_get_chunks() {
        // Arrange
        let bounds = CellHeightChunkBounds::new(0.0, 5000.0, 5000, 5000);
        // Act
        let output = bounds.get_chunks();
        // Assert
        assert_eq!(output, ChunkBounds::new(0, 9, 0, 9));
    }

    /// Cells of one chunk contribute to the border vertices of its neighbours.
    #[test]
    fn cell_height_chunk_bounds_get_contributions() {
        // Arrange
        let bounds = CellHeightChunkBounds::new(0.0, 512.0, 512, 512);
        // Act
        let sixteen = bounds.get_contributions(Spacing::Sixteen);
        let two = bounds.get_contributions(Spacing::Two);
        // Assert
        assert_eq!(sixteen, ChunkBounds::new(-1, 1, -1, 1));
        assert_eq!(two, ChunkBounds::new(-1, 1, -1, 1));
    }

    /// A file wholly inside one chunk reaches no further at the finest spacing.
    #[test]
    fn cell_height_chunk_bounds_get_contributions_inside() {
        // Arrange
        let bounds = CellHeightChunkBounds::new(100.0, 412.0, 100, 100);
        // Act
        let output = bounds.get_contributions(Spacing::Two);
        // Assert
        assert_eq!(output, ChunkBounds::new(0, 0, 0, 0));
    }
}
