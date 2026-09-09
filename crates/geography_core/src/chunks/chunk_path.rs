//! Location of chunk files on disk.

use crate::ChunkIndex;
use crate::HeightSpacing;
use std::path::PathBuf;

/// Location of chunk files on disk.
///
/// - Shared by the preprocessor writing chunks and the game loading them, so
///   both agree on where a chunk lives
#[derive(Clone, Debug, PartialEq)]
pub struct ChunkPath {
    /// Directory holding one subdirectory per [`HeightSpacing`].
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
    pub fn get(&self, spacing: HeightSpacing, index: ChunkIndex) -> PathBuf {
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
        let output = path.get(HeightSpacing::Sixteen, ChunkIndex::new(0, 0));
        // Assert
        assert_eq!(output, PathBuf::from("/tmp/terrain/16m/E00N00.f32"));
    }
}
