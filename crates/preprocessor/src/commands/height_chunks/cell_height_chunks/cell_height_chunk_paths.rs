//! Find the source files beneath a directory.

use crate::prelude::*;
use std::fs::read_dir;

/// Suffix identifying a source file.
const SUFFIX: &str = "_DTM_1m.tif";

/// Find the source files beneath a directory.
#[derive(Default, FromServices)]
pub struct CellHeightChunkPaths;

impl CellHeightChunkPaths {
    /// Get every source file beneath a directory, sorted by path.
    ///
    /// - Descends one level, each source file sitting in its own directory
    ///
    /// # Errors
    ///
    /// - IF a directory cannot be read
    #[expect(clippy::unused_self, reason = "resolved as a service")]
    pub fn get(&self, source: &Path) -> Result<Vec<PathBuf>, Report<CellHeightChunkPathsError>> {
        let mut paths = Vec::new();
        for path in Self::get_entries(source)? {
            if path.is_dir() {
                paths.extend(Self::get_entries(&path)?.filter(|path| is_source(path)));
            } else if is_source(&path) {
                paths.push(path);
            }
        }
        paths.sort();
        Ok(paths)
    }

    /// Get the path of every entry of a directory.
    fn get_entries(
        directory: &Path,
    ) -> Result<impl Iterator<Item = PathBuf>, Report<CellHeightChunkPathsError>> {
        let entries = read_dir(directory)
            .change_context(CellHeightChunkPathsError::ReadDirectory)
            .attach_path(directory)?;
        Ok(entries.filter_map(|entry| entry.ok().map(|entry| entry.path())))
    }
}

/// Is a path a source file?
fn is_source(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(SUFFIX))
}

/// Errors returned by [`CellHeightChunkPaths`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CellHeightChunkPathsError {
    /// Unable to read directory.
    #[error("Unable to read directory")]
    ReadDirectory,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{create_dir, write};
    use tempfile::tempdir;

    /// Source files are found one level down, other files are left behind.
    #[test]
    fn cell_height_chunk_paths_get() {
        // Arrange
        let directory = tempdir().expect("should create directory");
        let nested = directory.path().join("lidar_composite_dtm-2022-1-NY79sw");
        create_dir(&nested).expect("should create directory");
        write(nested.join("NY79sw_DTM_1m.tif"), []).expect("should write file");
        write(nested.join("NY79sw_DTM_1m.tif.aux.xml"), []).expect("should write file");
        write(directory.path().join("NY78sw_DTM_1m.tif"), []).expect("should write file");
        write(directory.path().join("readme.md"), []).expect("should write file");
        // Act
        let output = CellHeightChunkPaths.get(directory.path());
        // Assert
        let paths = output.expect("should find sources");
        let names: Vec<String> = paths
            .iter()
            .filter_map(|path| path.file_name())
            .map(|name| name.to_string_lossy().into_owned())
            .collect();
        assert_eq!(names, vec!["NY78sw_DTM_1m.tif", "NY79sw_DTM_1m.tif"]);
    }

    #[test]
    fn cell_height_chunk_paths_get_missing() {
        // Arrange
        let directory = tempdir().expect("should create directory");
        let missing = directory.path().join("missing");
        // Act
        let output = CellHeightChunkPaths.get(&missing);
        // Assert
        let report = output.expect_err("should reject directory");
        assert_eq!(
            *report.current_context(),
            CellHeightChunkPathsError::ReadDirectory
        );
    }
}
