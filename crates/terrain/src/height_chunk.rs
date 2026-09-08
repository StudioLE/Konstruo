//! Heights of one chunk at one spacing.

use crate::chunk_index::ChunkIndex;
use crate::spacing::Spacing;
use studiole_report::prelude::*;
use thiserror::Error;

/// Heights of one chunk at one spacing.
///
/// - Holds `spacing.vertices_across()` squared heights
/// - Stores them row major, north row first, west to east within a row
#[derive(Clone, Debug, PartialEq)]
pub struct HeightChunk {
    /// Interval between vertices.
    pub spacing: Spacing,
    /// Position of the chunk.
    pub index: ChunkIndex,
    /// Heights in meters.
    heights: Vec<f32>,
}

impl HeightChunk {
    /// Create a new [`HeightChunk`].
    ///
    /// # Errors
    ///
    /// - IF the height count is not `spacing.vertices_across()` squared
    pub fn new(
        spacing: Spacing,
        index: ChunkIndex,
        heights: Vec<f32>,
    ) -> Result<Self, Report<HeightChunkError>> {
        let expected = spacing.vertices_across().pow(2);
        if heights.len() != expected {
            return Err(Report::new(HeightChunkError::Heights)
                .attach("expected", expected)
                .attach("found", heights.len()));
        }
        Ok(Self {
            spacing,
            index,
            heights,
        })
    }

    /// Get the height of one vertex.
    ///
    /// - Rows run north to south, columns west to east
    /// - Returns [`None`] IF either index is beyond the chunk
    #[must_use]
    pub fn get_height(&self, row: usize, column: usize) -> Option<f32> {
        let across = self.spacing.vertices_across();
        if row >= across || column >= across {
            return None;
        }
        self.heights.get(row * across + column).copied()
    }

    /// Encode the heights as little-endian `f32`, without header or padding.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.heights.len() * 4);
        for height in &self.heights {
            bytes.extend_from_slice(&height.to_le_bytes());
        }
        bytes
    }

    /// Create a new [`HeightChunk`] from bytes written by [`HeightChunk::to_bytes`].
    ///
    /// # Errors
    ///
    /// - IF the byte count is not four times the height count
    pub fn from_bytes(
        spacing: Spacing,
        index: ChunkIndex,
        bytes: &[u8],
    ) -> Result<Self, Report<HeightChunkError>> {
        let expected = spacing.vertices_across().pow(2) * 4;
        if bytes.len() != expected {
            return Err(Report::new(HeightChunkError::Bytes)
                .attach("expected", expected)
                .attach("found", bytes.len()));
        }
        let heights = bytes
            .chunks_exact(4)
            .map(|height| {
                let height =
                    <[u8; 4]>::try_from(height).expect("chunks_exact should yield four bytes");
                f32::from_le_bytes(height)
            })
            .collect();
        Self::new(spacing, index, heights)
    }

    /// Create a mock [`HeightChunk`] with each height set to its own offset.
    #[cfg(test)]
    pub(crate) fn mock(spacing: Spacing) -> Self {
        use std::iter::successors;
        let heights = successors(Some(0.0_f32), |height| Some(height + 1.0))
            .take(spacing.vertices_across().pow(2))
            .collect();
        Self::new(spacing, ChunkIndex::new(0, 0), heights).expect("should be valid")
    }
}

/// Errors returned by [`HeightChunk`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum HeightChunkError {
    /// Unexpected number of heights.
    #[error("Unexpected number of heights")]
    Heights,
    /// Unexpected number of bytes.
    #[error("Unexpected number of bytes")]
    Bytes,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spacing with the fewest vertices, so tests stay small.
    const SPACING: Spacing = Spacing::Sixteen;

    #[test]
    fn height_chunk_new_invalid() {
        // Arrange
        let heights = vec![0.0; 42];
        // Act
        let output = HeightChunk::new(SPACING, ChunkIndex::new(0, 0), heights);
        // Assert
        let report = output.expect_err("should reject heights");
        assert_eq!(*report.current_context(), HeightChunkError::Heights);
    }

    #[test]
    fn height_chunk_get_height() {
        // Arrange
        let chunk = HeightChunk::mock(SPACING);
        let across = SPACING.vertices_across();
        // Act
        let north_west = chunk.get_height(0, 0);
        let south_east = chunk.get_height(across - 1, across - 1);
        let beyond_row = chunk.get_height(across, 0);
        let beyond_column = chunk.get_height(0, across);
        // Assert
        assert_eq!(north_west, Some(0.0));
        assert_eq!(south_east, Some(1224.0));
        assert_eq!(beyond_row, None);
        assert_eq!(beyond_column, None);
    }

    #[test]
    fn height_chunk_to_bytes() {
        // Arrange
        let chunk = HeightChunk::mock(SPACING);
        // Act
        let bytes = chunk.to_bytes();
        // Assert
        assert_eq!(bytes.len(), SPACING.vertices_across().pow(2) * 4);
        assert_eq!(bytes.get(0..4), Some(0.0_f32.to_le_bytes().as_slice()));
    }

    #[test]
    fn height_chunk_from_bytes() {
        // Arrange
        let chunk = HeightChunk::mock(SPACING);
        let bytes = chunk.to_bytes();
        // Act
        let output = HeightChunk::from_bytes(SPACING, chunk.index, &bytes);
        // Assert
        assert_eq!(output.expect("should read bytes"), chunk);
    }

    #[test]
    fn height_chunk_from_bytes_short() {
        // Arrange
        let chunk = HeightChunk::mock(SPACING);
        let mut bytes = chunk.to_bytes();
        bytes.pop();
        // Act
        let output = HeightChunk::from_bytes(SPACING, chunk.index, &bytes);
        // Assert
        let report = output.expect_err("should reject bytes");
        assert_eq!(*report.current_context(), HeightChunkError::Bytes);
    }
}
