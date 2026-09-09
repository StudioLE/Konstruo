//! Write chunk files.

use crate::prelude::*;
use std::fs::{create_dir_all, write};

/// Write chunk files.
pub struct ChunkWriter {
    /// Location of chunk files on disk.
    path: ChunkPath,
}

impl ChunkWriter {
    /// Create a new [`ChunkWriter`].
    #[must_use]
    pub fn new(path: ChunkPath) -> Self {
        Self { path }
    }

    /// Write one chunk, creating its spacing directory when absent.
    ///
    /// # Errors
    ///
    /// - IF the directory cannot be created
    /// - IF the file cannot be written
    pub fn write(&self, chunk: &HeightChunk) -> Result<(), Report<ChunkWriteError>> {
        let path = self.path.get(chunk.spacing, chunk.index);
        if let Some(directory) = path.parent() {
            create_dir_all(directory)
                .change_context(ChunkWriteError::CreateDirectory)
                .attach_path(directory)?;
        }
        write(&path, chunk.to_bytes())
            .change_context(ChunkWriteError::Write)
            .attach_path(&path)
    }
}

/// Errors returned by [`ChunkWriter`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ChunkWriteError {
    /// Unable to create directory.
    #[error("Unable to create directory")]
    CreateDirectory,
    /// Unable to write chunk.
    #[error("Unable to write chunk")]
    Write,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn chunk_writer_write() {
        // Arrange
        let directory = tempdir().expect("should create directory");
        let path = ChunkPath::new(directory.path());
        let writer = ChunkWriter::new(path.clone());
        let chunk = HeightChunk::mock(HeightSpacing::Sixteen);
        // Act
        let output = writer.write(&chunk);
        // Assert
        output.expect("should write chunk");
        let written = fs::read(path.get(chunk.spacing, chunk.index)).expect("should read file");
        assert_eq!(written, chunk.to_bytes());
    }
}
