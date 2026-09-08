//! Read chunk files.

use crate::chunk_index::ChunkIndex;
use crate::chunk_path::ChunkPath;
use crate::height_chunk::HeightChunk;
use crate::spacing::Spacing;
use std::fs::read;
use std::io::ErrorKind;
use studiole_report::prelude::*;
use thiserror::Error;

/// Read chunk files.
pub struct ChunkReader {
    /// Location of chunk files on disk.
    path: ChunkPath,
}

impl ChunkReader {
    /// Create a new [`ChunkReader`].
    #[must_use]
    pub fn new(path: ChunkPath) -> Self {
        Self { path }
    }

    /// Read one chunk.
    ///
    /// - Returns [`None`] IF no file covers the chunk at this spacing
    ///
    /// # Errors
    ///
    /// - IF the file cannot be read
    /// - IF the file length is not four times the vertex count
    pub fn read(
        &self,
        spacing: Spacing,
        index: ChunkIndex,
    ) -> Result<Option<HeightChunk>, Report<ChunkReadError>> {
        let path = self.path.get(spacing, index);
        let bytes = match read(&path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
            Err(error) => {
                return Err(Report::from(error)
                    .change_context(ChunkReadError::Read)
                    .attach_path(&path))
            }
        };
        let chunk = HeightChunk::from_bytes(spacing, index, &bytes)
            .change_context(ChunkReadError::Decode)
            .attach_path(&path)?;
        Ok(Some(chunk))
    }
}

/// Errors returned by [`ChunkReader`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ChunkReadError {
    /// Unable to read chunk.
    #[error("Unable to read chunk")]
    Read,
    /// Unable to decode chunk.
    #[error("Unable to decode chunk")]
    Decode,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_writer::ChunkWriter;
    use std::fs;
    use tempfile::tempdir;

    /// Spacing with the fewest vertices, so tests stay small.
    const SPACING: Spacing = Spacing::Sixteen;

    /// A chunk written then read is identical.
    #[test]
    fn chunk_reader_read() {
        // Arrange
        let directory = tempdir().expect("should create directory");
        let path = ChunkPath::new(directory.path());
        let chunk = HeightChunk::mock(SPACING);
        ChunkWriter::new(path.clone())
            .write(&chunk)
            .expect("should write chunk");
        // Act
        let output = ChunkReader::new(path).read(SPACING, chunk.index);
        // Assert
        assert_eq!(output.expect("should read chunk"), Some(chunk));
    }

    #[test]
    fn chunk_reader_read_missing() {
        // Arrange
        let directory = tempdir().expect("should create directory");
        let reader = ChunkReader::new(ChunkPath::new(directory.path()));
        // Act
        let output = reader.read(SPACING, ChunkIndex::new(0, 0));
        // Assert
        assert_eq!(output.expect("should read chunk"), None);
    }

    #[test]
    fn chunk_reader_read_short() {
        // Arrange
        let directory = tempdir().expect("should create directory");
        let path = ChunkPath::new(directory.path());
        let chunk = HeightChunk::mock(SPACING);
        ChunkWriter::new(path.clone())
            .write(&chunk)
            .expect("should write chunk");
        let file = path.get(SPACING, chunk.index);
        let mut bytes = fs::read(&file).expect("should read file");
        bytes.pop();
        fs::write(&file, bytes).expect("should write file");
        // Act
        let output = ChunkReader::new(path).read(SPACING, chunk.index);
        // Assert
        let report = output.expect_err("should reject chunk");
        assert_eq!(*report.current_context(), ChunkReadError::Decode);
    }
}
