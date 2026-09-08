//! Handler for the height chunks subcommand.

use crate::prelude::*;

/// Resample source height data into level-of-detail chunk files.
#[derive(FromServices)]
pub struct HeightChunksHandler {
    /// Source file discovery.
    paths: Arc<CellHeightChunkPaths>,
    /// Source file reader.
    reader: Arc<CellHeightChunkReader>,
}

impl HeightChunksHandler {
    /// Generate every chunk the source data covers.
    ///
    /// # Errors
    ///
    /// - IF the source files cannot be found, read or decoded
    /// - IF the chunks they cover differ from [`ChunkBounds::UNION`]
    /// - IF a chunk cannot be written
    pub fn execute(&self, request: HeightChunksRequest) -> Result<(), Report<HeightChunksError>> {
        self.run(request, ChunkBounds::UNION)
    }

    /// Generate every chunk, guarding against bounds other than the expected.
    fn run(
        &self,
        request: HeightChunksRequest,
        expected: ChunkBounds,
    ) -> Result<(), Report<HeightChunksError>> {
        trace!(?request, "Generating height chunks");
        let sources = self.get_sources(&request.source)?;
        let derived = get_bounds(&sources)?;
        if derived != expected {
            return Err(Report::new(HeightChunksError::Bounds)
                .attach("derived", derived)
                .attach("expected", expected));
        }
        debug!(%derived, "Derived chunk bounds");
        let mut resample = Resample::new(&request.output, &sources);
        for (index, source) in sources.iter().enumerate() {
            let file_name = source
                .path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();
            debug!(index, %file_name, "Processing cell height chunk");
            let chunk = self
                .reader
                .read(&source.path)
                .change_context(HeightChunksError::Source)?;
            resample.add(&chunk)?;
            debug!(index, %file_name, "Processed cell height chunk");
        }
        info!(
            written = resample.written,
            skipped = resample.skipped,
            "Wrote height chunks"
        );
        Ok(())
    }

    /// Read the extent of every source file, north row first then west to east.
    fn get_sources(&self, directory: &Path) -> Result<Vec<Source>, Report<HeightChunksError>> {
        let paths = self
            .paths
            .get(directory)
            .change_context(HeightChunksError::Sources)?;
        debug!(count = paths.len(), directory = %directory.display(), "Found cell height chunks");
        let mut sources = Vec::with_capacity(paths.len());
        for path in paths {
            let bounds = self
                .reader
                .read_bounds(&path)
                .change_context(HeightChunksError::Source)?;
            sources.push(Source { path, bounds });
        }
        sources.sort_by(|a, b| {
            b.bounds
                .northing
                .total_cmp(&a.bounds.northing)
                .then(a.bounds.easting.total_cmp(&b.bounds.easting))
        });
        Ok(sources)
    }
}

// TODO: Move `Source` and `Resample` into `resample/`
/// One source file and the extent it covers.
struct Source {
    /// Path of the file.
    path: PathBuf,
    /// Extent of its cells in world space.
    bounds: CellHeightChunkBounds,
}

/// Accumulate every source file into chunks, writing each as it completes.
struct Resample {
    /// Chunk file writer.
    writer: ChunkWriter,
    /// Accumulator of every chunk awaiting a source file.
    accumulators: HashMap<(Spacing, ChunkIndex), ChunkAccumulator>,
    /// Source files each chunk is still owed, one tracker per [`Spacing`].
    ///
    /// - Ordered to match [`Spacing::ALL`], which [`Resample::add`] zips against
    completions: Vec<ChunkCompletion>,
    /// Number of chunks written.
    written: usize,
    /// Number of chunks left unwritten for having no surveyed vertex.
    skipped: usize,
}

impl Resample {
    /// Create a new [`Resample`], counting the files each chunk is owed.
    fn new(output: &Path, sources: &[Source]) -> Self {
        let completions = Spacing::ALL
            .iter()
            .map(|spacing| {
                let mut completion = ChunkCompletion::new();
                for source in sources {
                    completion.add(source.bounds.get_contributions(*spacing));
                }
                completion
            })
            .collect();
        Self {
            writer: ChunkWriter::new(ChunkPath::new(output)),
            accumulators: HashMap::new(),
            completions,
            written: 0,
            skipped: 0,
        }
    }

    /// Add one source file to every chunk it contributes to.
    ///
    /// - Writes and frees each chunk as its last contributing file lands
    fn add(&mut self, chunk: &CellHeightChunk) -> Result<(), Report<HeightChunksError>> {
        trace!("Resampling cell height chunk");
        let mut time_accumulate = Timer::new_stopped();
        let mut time_finish = Timer::new_stopped();
        let mut time_write = Timer::new_stopped();
        for (spacing, completion) in Spacing::ALL.iter().zip(&mut self.completions) {
            for index in chunk.bounds.get_contributions(*spacing).indices() {
                let key = (*spacing, index);
                time_accumulate.resume();
                self.accumulators
                    .entry(key)
                    .or_insert_with(|| ChunkAccumulator::new(*spacing, index))
                    .add(chunk);
                time_accumulate.stop();
                completion.record(index);
                if !completion.is_complete(index) {
                    continue;
                }
                let accumulator = self
                    .accumulators
                    .remove(&key)
                    .expect("completed chunk should hold an accumulator");
                time_finish.resume();
                let finished = accumulator.finish();
                time_finish.stop();
                match finished {
                    Some(height_chunk) => {
                        time_write.resume();
                        self.writer
                            .write(&height_chunk)
                            .change_context(HeightChunksError::Write)?;
                        time_write.stop();
                        self.written += 1;
                    }
                    None => self.skipped += 1,
                }
            }
        }
        trace!(
            accumulate = %time_accumulate,
            finish = %time_finish,
            write = %time_write,
            "Resampled cell height chunk"
        );
        Ok(())
    }
}

/// Union of the chunks every source file covers.
fn get_bounds(sources: &[Source]) -> Result<ChunkBounds, Report<HeightChunksError>> {
    let mut bounds: Option<ChunkBounds> = None;
    for source in sources {
        let chunks = source.bounds.get_chunks();
        bounds = Some(bounds.map_or(chunks, |bounds| bounds.union(chunks)));
    }
    bounds.ok_or_else(|| Report::new(HeightChunksError::Sources))
}

/// Errors returned by [`HeightChunksHandler`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum HeightChunksError {
    /// Unable to find source files.
    #[error("Unable to find source files")]
    Sources,
    /// Unable to read source file.
    #[error("Unable to read source file")]
    Source,
    /// Derived bounds differ from the expected bounds.
    #[error("Derived bounds differ from the expected bounds")]
    Bounds,
    /// Unable to write chunk.
    #[error("Unable to write chunk")]
    Write,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::height_chunks::cell_height_chunks::tiff::tiff_fixture::TiffFixture;
    use std::fs::{read_dir, write};
    use tempfile::{tempdir, TempDir};

    /// Easting of the fixture in EPSG:27700, one chunk east of the origin.
    const EASTING: f64 = 370_512.0;

    /// Northing of the fixture in EPSG:27700, one chunk north of the origin.
    const NORTHING: f64 = 590_512.0;

    /// Height every cell of the fixture reads as.
    const HEIGHT: f32 = 10.0;

    /// Chunks the fixture covers, being one chunk east and level with the origin.
    const DERIVED: ChunkBounds = ChunkBounds::new(1, 1, 0, 0);

    /// Chunks the fixture writes, its border vertices reaching three neighbours.
    const WRITTEN: [&str; 4] = ["E00N00.f32", "E00N01.f32", "E01N00.f32", "E01N01.f32"];

    /// The whole pipeline runs over a synthetic source file.
    #[test]
    fn height_chunks_handler_execute() {
        // Arrange
        let (_source, _output, request) = mock();
        let handler = handler();
        // Act
        let output = handler.run(request.clone(), DERIVED);
        // Assert
        output.expect("should generate chunks");
        for spacing in Spacing::ALL {
            let written = get_names(&request.output.join(spacing.directory()));
            assert_eq!(written, WRITTEN, "at {spacing:?}");
        }
    }

    /// Every vertex the fixture covers reads as the height of its cells.
    #[test]
    fn height_chunks_handler_execute_heights() {
        // Arrange
        let (_source, _output, request) = mock();
        let handler = handler();
        // Act
        handler
            .run(request.clone(), DERIVED)
            .expect("should generate chunks");
        // Assert
        let reader = ChunkReader::new(ChunkPath::new(&request.output));
        let chunk = reader
            .read(Spacing::Sixteen, ChunkIndex::new(1, 0))
            .expect("should read chunk")
            .expect("should have chunk");
        assert_eq!(chunk.get_height(2, 2), Some(HEIGHT));
    }

    /// Bounds other than the expected abort the run before a chunk is written.
    #[test]
    fn height_chunks_handler_execute_bounds() {
        // Arrange
        let (_source, _output, request) = mock();
        let handler = handler();
        // Act
        let output = handler.run(request.clone(), ChunkBounds::UNION);
        // Assert
        let report = output.expect_err("should reject bounds");
        assert_eq!(*report.current_context(), HeightChunksError::Bounds);
        assert!(!request.output.exists());
    }

    /// Resolve a [`HeightChunksHandler`] from the application services.
    fn handler() -> Arc<HeightChunksHandler> {
        let services = ServiceBuilder::new().with_app_services().build();
        services.expect::<HeightChunksHandler>()
    }

    /// Write a source file to a temporary directory.
    fn mock() -> (TempDir, TempDir, HeightChunksRequest) {
        let source = tempdir().expect("should create directory");
        let output = tempdir().expect("should create directory");
        let fixture = TiffFixture::new()
            .with_tiepoint(EASTING, NORTHING)
            .with_cells(vec![HEIGHT; 256 * 256]);
        let path = source.path().join("fixture_DTM_1m.tif");
        write(path, fixture.to_bytes()).expect("should write file");
        let request = HeightChunksRequest {
            source: source.path().to_path_buf(),
            output: output.path().join("terrain"),
        };
        (source, output, request)
    }

    /// File names within a directory, sorted.
    fn get_names(directory: &Path) -> Vec<String> {
        let mut names: Vec<String> = read_dir(directory)
            .expect("should read directory")
            .filter_map(Result::ok)
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        names
    }
}
