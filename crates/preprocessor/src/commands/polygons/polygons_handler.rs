//! Handler for the polygons subcommand.

use crate::prelude::*;

/// Export polygon layers as `GeoJSON` files.
#[derive(FromServices)]
pub struct PolygonsHandler {
    /// Source layer reader.
    reader: Arc<PolygonReader>,
    /// Output file writer.
    writer: Arc<PolygonWriter>,
}

impl PolygonsHandler {
    /// Export every layer the source dataset holds.
    ///
    /// # Errors
    ///
    /// - IF a source file cannot be opened, or holds no such layer
    /// - IF an output file cannot be written
    pub fn execute(&self, request: PolygonsRequest) -> Result<(), Report<PolygonsError>> {
        self.run(request, &PolygonLayer::ALL, ChunkBounds::UNION.get_extent())
    }

    /// Export every layer of a list, clipped to an extent.
    ///
    /// - Aborts on the first layer that fails, rather than exporting the rest
    fn run(
        &self,
        request: PolygonsRequest,
        layers: &[PolygonLayer],
        extent: ChunkExtent,
    ) -> Result<(), Report<PolygonsError>> {
        trace!(?request, "Exporting polygons");
        for layer in layers {
            let source = request.source.join(layer.source);
            let geometries = self
                .reader
                .read(&source, layer.layer, extent)
                .change_context(PolygonsError::Read)?;
            let output = request.output.join(format!("{}.geojson", layer.output));
            self.writer
                .write(&output, &geometries)
                .change_context(PolygonsError::Write)?;
            info!(
                layer = layer.layer,
                count = geometries.len(),
                "Exported polygons"
            );
        }
        Ok(())
    }
}

/// Errors returned by [`PolygonsHandler`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum PolygonsError {
    /// Unable to read source layer.
    #[error("Unable to read source layer")]
    Read,
    /// Unable to write output file.
    #[error("Unable to write output file")]
    Write,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::polygons::polygon_fixture::{square, PolygonFixture};
    use std::fs::read_to_string;
    use tempfile::{tempdir, TempDir};

    /// Extent covering the chunk at the origin.
    const EXTENT: ChunkExtent = ChunkExtent {
        min: BngCoordinates::new(370_000.0, 590_000.0),
        max: BngCoordinates::new(370_512.0, 590_512.0),
    };

    /// Surface water layer the fixture holds.
    const WATER: PolygonLayer = PolygonLayer {
        source: "fixture.gpkg",
        layer: "surface_water_area",
        output: "surface-water",
    };

    /// Woodland layer the fixture holds.
    const WOODLAND: PolygonLayer = PolygonLayer {
        source: "fixture.gpkg",
        layer: "woodland",
        output: "woodland",
    };

    /// Layers the fixture holds.
    const LAYERS: [PolygonLayer; 2] = [WATER, WOODLAND];

    /// Every layer of the list lands in its own file.
    #[test]
    fn polygons_handler_execute() {
        // Arrange
        let (_source, _output, request) = mock();
        let handler = handler();
        // Act
        let output = handler.run(request.clone(), &LAYERS, EXTENT);
        // Assert
        output.expect("should export polygons");
        for layer in &LAYERS {
            let path = request.output.join(format!("{}.geojson", layer.output));
            let contents = read_to_string(&path).expect("should read file");
            assert!(
                contents.contains("urn:ogc:def:crs:EPSG::27700"),
                "at {path:?}"
            );
        }
    }

    /// A layer absent from the source file aborts before a file is written.
    #[test]
    fn polygons_handler_execute_missing_layer() {
        // Arrange
        let (_source, _output, request) = mock();
        let handler = handler();
        let layers = [PolygonLayer {
            source: "fixture.gpkg",
            layer: "missing",
            output: "missing",
        }];
        // Act
        let output = handler.run(request.clone(), &layers, EXTENT);
        // Assert
        let report = output.expect_err("should reject layer");
        assert_eq!(*report.current_context(), PolygonsError::Read);
        assert!(!request.output.exists());
    }

    /// Resolve a [`PolygonsHandler`] from the application services.
    fn handler() -> Arc<PolygonsHandler> {
        let services = ServiceBuilder::new().with_app_services().build();
        services.expect::<PolygonsHandler>()
    }

    /// Write a source file holding every layer of [`LAYERS`].
    fn mock() -> (TempDir, TempDir, PolygonsRequest) {
        let source = tempdir().expect("should create directory");
        let output = tempdir().expect("should create directory");
        let path = source.path().join("fixture.gpkg");
        PolygonFixture::new(WATER.layer)
            .with_geometries(vec![square(370_100.0, 590_100.0)])
            .write(&path);
        PolygonFixture::new(WOODLAND.layer)
            .with_geometries(vec![square(370_200.0, 590_200.0)])
            .write(&path);
        let request = PolygonsRequest {
            source: source.path().to_path_buf(),
            output: output.path().join("polygons"),
        };
        (source, output, request)
    }
}
