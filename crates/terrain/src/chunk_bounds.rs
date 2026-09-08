//! Rectangle of chunk indices.

use crate::chunk_index::ChunkIndex;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Side length of a chunk, in meters.
pub const CHUNK_SIZE: i32 = 512;

/// Rectangle of chunk indices, inclusive on every edge.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChunkBounds {
    /// Westmost chunk index.
    min_x: i32,
    /// Eastmost chunk index.
    max_x: i32,
    /// Southmost chunk index.
    min_y: i32,
    /// Northmost chunk index.
    max_y: i32,
}

impl ChunkBounds {
    /// Bounds covering every chunk the source dataset surveys.
    // TODO: Move to the preprocessor once a manifest describes the source dataset
    pub const UNION: Self = Self {
        min_x: -30,
        max_x: 29,
        min_y: -40,
        max_y: 29,
    };

    /// Create a new [`ChunkBounds`].
    #[must_use]
    pub const fn new(min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    /// Are these bounds inclusive of a chunk?
    #[must_use]
    pub const fn contains(self, index: ChunkIndex) -> bool {
        index.x >= self.min_x
            && index.x <= self.max_x
            && index.y >= self.min_y
            && index.y <= self.max_y
    }

    /// Smallest bounds containing both of these bounds.
    #[must_use]
    pub fn union(self, other: Self) -> Self {
        Self {
            min_x: self.min_x.min(other.min_x),
            max_x: self.max_x.max(other.max_x),
            min_y: self.min_y.min(other.min_y),
            max_y: self.max_y.max(other.max_y),
        }
    }

    /// Every [`ChunkIndex`] within these bounds, north row first.
    pub fn indices(self) -> impl Iterator<Item = ChunkIndex> {
        (self.min_y..=self.max_y)
            .rev()
            .flat_map(move |y| (self.min_x..=self.max_x).map(move |x| ChunkIndex::new(x, y)))
    }
}

impl Display for ChunkBounds {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "min_x: {}, max_x: {}, min_y: {}, max_y: {}",
            self.min_x, self.max_x, self.min_y, self.max_y
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_bounds_union() {
        // Arrange
        let west = ChunkBounds::new(-2, 0, 0, 1);
        let east = ChunkBounds::new(1, 3, -1, 0);
        // Act
        let output = west.union(east);
        // Assert
        assert_eq!(output, ChunkBounds::new(-2, 3, -1, 1));
    }

    #[test]
    fn chunk_bounds_display() {
        assert_eq!(
            ChunkBounds::UNION.to_string(),
            "min_x: -30, max_x: 29, min_y: -40, max_y: 29"
        );
    }

    #[test]
    fn chunk_bounds_contains_corners() {
        // Arrange
        let corners = [
            ChunkIndex::new(-30, -40),
            ChunkIndex::new(29, -40),
            ChunkIndex::new(-30, 29),
            ChunkIndex::new(29, 29),
        ];
        // Act
        let contained: Vec<bool> = corners
            .iter()
            .map(|index| ChunkBounds::UNION.contains(*index))
            .collect();
        // Assert
        assert_eq!(contained, vec![true; 4]);
    }

    #[test]
    fn chunk_bounds_contains_beyond() {
        // Arrange
        let beyond = [
            ChunkIndex::new(-31, 0),
            ChunkIndex::new(30, 0),
            ChunkIndex::new(0, -41),
            ChunkIndex::new(0, 30),
        ];
        // Act
        let contained: Vec<bool> = beyond
            .iter()
            .map(|index| ChunkBounds::UNION.contains(*index))
            .collect();
        // Assert
        assert_eq!(contained, vec![false; 4]);
    }

    #[test]
    fn chunk_bounds_indices() {
        // Act
        let indices: Vec<ChunkIndex> = ChunkBounds::new(-1, 0, -1, 0).indices().collect();
        // Assert
        assert_eq!(
            indices,
            vec![
                ChunkIndex::new(-1, 0),
                ChunkIndex::new(0, 0),
                ChunkIndex::new(-1, -1),
                ChunkIndex::new(0, -1),
            ]
        );
    }

    #[test]
    fn chunk_bounds_indices_union() {
        // Act
        let indices: Vec<ChunkIndex> = ChunkBounds::UNION.indices().collect();
        // Assert
        assert_eq!(indices.len(), 4200);
    }
}
