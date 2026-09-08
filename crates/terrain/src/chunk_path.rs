//! Location of chunk files on disk.

use crate::chunk_index::ChunkIndex;
use crate::spacing::Spacing;
use std::path::PathBuf;

/// Location of chunk files on disk.
///
/// - Shared by [`crate::ChunkReader`] and [`crate::ChunkWriter`], so a chunk
///   written then read is identical
#[derive(Clone, Debug, PartialEq)]
pub struct ChunkPath {
    /// Directory holding one subdirectory per [`Spacing`].
    directory: PathBuf,
}

impl ChunkPath {
    /// Create a new [`ChunkPath`].
    #[must_use]
    pub fn new(directory: impl Into<PathBuf>) -> Self {
        Self {
            directory: directory.into(),
        }
    }

    /// Get the path of one chunk file.
    #[must_use]
    pub fn get(&self, spacing: Spacing, index: ChunkIndex) -> PathBuf {
        self.directory
            .join(spacing.directory())
            .join(format!("{index}.f32"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_path_get() {
        // Arrange
        let path = ChunkPath::new("/tmp/terrain");
        // Act
        let output = path.get(Spacing::Sixteen, ChunkIndex::new(0, 0));
        // Assert
        assert_eq!(output, PathBuf::from("/tmp/terrain/16m/E00N00.f32"));
    }
}
