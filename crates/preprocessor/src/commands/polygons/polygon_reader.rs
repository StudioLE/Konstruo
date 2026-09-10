//! Read polygons from a `GeoPackage` layer.

use crate::prelude::*;
use geo_types::Geometry;
use geopackage::{BoundingBox, ConversionOptions, GeoPackage};

/// Read polygons from a `GeoPackage` layer.
#[derive(Default, FromServices)]
pub struct PolygonReader;

impl PolygonReader {
    /// Read every polygon of a layer that intersects an extent.
    ///
    /// - Resolves the geometry column from the file rather than assuming a name
    /// - Selects the geometry alone, so attribute columns are never fetched
    /// - Reads through the spatial index when the layer holds one
    /// - Warns and skips geometry that is neither a polygon nor a multipolygon
    ///
    /// # Errors
    ///
    /// - IF the file cannot be opened
    /// - IF the layer is absent, or declares no geometry column
    /// - IF a feature cannot be read or converted
    #[expect(clippy::unused_self, reason = "resolved as a service")]
    pub fn read(
        &self,
        path: &Path,
        layer: &str,
        extent: ChunkExtent,
    ) -> Result<Vec<Geometry<f64>>, Report<PolygonReaderError>> {
        trace!(path = %path.display(), layer, "Reading polygons");
        let package = GeoPackage::open_read_only(path)
            .change_context(PolygonReaderError::Open)
            .attach_path(path)?;
        let table = package
            .layer(layer)
            .change_context(PolygonReaderError::Layer)
            .attach_source(path, layer)?;
        let column = table
            .geometry_column()
            .map(|column| column.column_name.clone())
            .ok_or_else(|| Report::new(PolygonReaderError::GeometryColumn))
            .attach_source(path, layer)?;
        let table = table
            .with_conversion_options(ConversionOptions::lenient())
            .with_columns(&[&column])
            .change_context(PolygonReaderError::Project)
            .attach_source(path, layer)
            .attach("column", &column)?;
        let features = table
            .features_in(get_bounding_box(extent))
            .change_context(PolygonReaderError::Query)
            .attach_source(path, layer)?;
        let mut geometries = Vec::with_capacity(features.len());
        let mut skipped = 0_usize;
        for feature in features {
            let feature = feature
                .change_context(PolygonReaderError::Feature)
                .attach_source(path, layer)?;
            let read = feature
                .geometry()
                .change_context(PolygonReaderError::Feature)
                .attach_source(path, layer)?;
            let Some(read) = read else {
                skipped += 1;
                warn!(layer, "Skipped a feature holding no geometry");
                continue;
            };
            let geometry = read
                .to_geo()
                .ok_or_else(|| Report::new(PolygonReaderError::Convert))
                .attach_source(path, layer)?;
            match geometry {
                Geometry::Polygon(_) | Geometry::MultiPolygon(_) => geometries.push(geometry),
                _ => {
                    skipped += 1;
                    let geometry_type = get_type_name(&geometry);
                    warn!(layer, geometry_type, "Skipped an unsupported geometry");
                }
            }
        }
        debug!(layer, read = geometries.len(), skipped, "Read polygons");
        Ok(geometries)
    }
}

/// Bounding box of an extent, in the order `min_x, min_y, max_x, max_y`.
fn get_bounding_box(extent: ChunkExtent) -> BoundingBox {
    BoundingBox::new(
        extent.min.easting,
        extent.min.northing,
        extent.max.easting,
        extent.max.northing,
    )
}

/// Name of the type of a geometry.
fn get_type_name(geometry: &Geometry<f64>) -> &'static str {
    match geometry {
        Geometry::Point(_) => "Point",
        Geometry::Line(_) => "Line",
        Geometry::LineString(_) => "LineString",
        Geometry::Polygon(_) => "Polygon",
        Geometry::MultiPoint(_) => "MultiPoint",
        Geometry::MultiLineString(_) => "MultiLineString",
        Geometry::MultiPolygon(_) => "MultiPolygon",
        Geometry::GeometryCollection(_) => "GeometryCollection",
        Geometry::Rect(_) => "Rect",
        Geometry::Triangle(_) => "Triangle",
    }
}

/// Attach the source a read failed on.
trait AttachSource: Sized {
    /// Attach the path of the file and the name of the layer.
    #[must_use]
    fn attach_source(self, path: &Path, layer: &str) -> Self;
}

impl<T> AttachSource for Result<T, Report<PolygonReaderError>> {
    fn attach_source(self, path: &Path, layer: &str) -> Self {
        self.attach_path(path).attach("layer", layer)
    }
}

/// Errors returned by [`PolygonReader`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum PolygonReaderError {
    /// Unable to open source file.
    #[error("Unable to open source file")]
    Open,
    /// Unable to read layer.
    #[error("Unable to read layer")]
    Layer,
    /// Layer declares no geometry column.
    #[error("Layer declares no geometry column")]
    GeometryColumn,
    /// Unable to select geometry column.
    #[error("Unable to select geometry column")]
    Project,
    /// Unable to query layer.
    #[error("Unable to query layer")]
    Query,
    /// Unable to read feature.
    #[error("Unable to read feature")]
    Feature,
    /// Unable to convert geometry.
    #[error("Unable to convert geometry")]
    Convert,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::polygons::polygon_fixture::{square, PolygonFixture, SURVEYED};
    use geo_types::Point;
    use tempfile::{tempdir, TempDir};

    /// Name of the layer every fixture holds.
    const LAYER: &str = "surface_water_area";

    /// Extent covering the chunk at the origin.
    const EXTENT: ChunkExtent = ChunkExtent {
        min: BngCoordinates::new(370_000.0, 590_000.0),
        max: BngCoordinates::new(370_512.0, 590_512.0),
    };

    /// `DATETIME` text with one fractional digit where the specification wants three.
    const NON_CONFORMING: &str = "2018-01-01T00:00:00.0Z";

    /// Only the polygons intersecting the extent come back.
    #[test]
    fn polygon_reader_read() {
        // Arrange
        let (_directory, path) = mock(vec![
            square(370_100.0, 590_100.0),
            square(380_000.0, 600_000.0),
        ]);
        // Act
        let output = PolygonReader.read(&path, LAYER, EXTENT);
        // Assert
        let geometries = output.expect("should read polygons");
        assert_eq!(geometries, vec![square(370_100.0, 590_100.0)]);
    }

    /// A geometry column named anything but the default is resolved from the file.
    #[test]
    fn polygon_reader_read_geometry_column() {
        // Arrange
        let (_directory, path) = mock_named(vec![square(370_100.0, 590_100.0)], "SHAPE");
        // Act
        let output = PolygonReader.read(&path, LAYER, EXTENT);
        // Assert
        let geometries = output.expect("should read polygons");
        assert_eq!(geometries.len(), 1);
    }

    /// Geometry that is neither a polygon nor a multipolygon is skipped.
    #[test]
    fn polygon_reader_read_unsupported() {
        // Arrange
        let (_directory, path) = mock(vec![
            Geometry::Point(Point::new(370_100.0, 590_100.0)),
            square(370_200.0, 590_200.0),
        ]);
        // Act
        let output = PolygonReader.read(&path, LAYER, EXTENT);
        // Assert
        let geometries = output.expect("should read polygons");
        assert_eq!(geometries, vec![square(370_200.0, 590_200.0)]);
    }

    /// `DATETIME` text the specification rejects does not fail the read.
    #[test]
    fn polygon_reader_read_non_conforming_datetime() {
        // Arrange
        let (_directory, path) = mock(vec![square(370_100.0, 590_100.0)]);
        set_surveyed(&path, NON_CONFORMING);
        // Act
        let output = PolygonReader.read(&path, LAYER, EXTENT);
        // Assert
        let geometries = output.expect("should read polygons");
        assert_eq!(geometries.len(), 1);
    }

    #[test]
    fn polygon_reader_read_missing_file() {
        // Arrange
        let directory = tempdir().expect("should create directory");
        let path = directory.path().join("missing.gpkg");
        // Act
        let output = PolygonReader.read(&path, LAYER, EXTENT);
        // Assert
        let report = output.expect_err("should reject file");
        assert_eq!(*report.current_context(), PolygonReaderError::Open);
    }

    #[test]
    fn polygon_reader_read_missing_layer() {
        // Arrange
        let (_directory, path) = mock(vec![square(370_100.0, 590_100.0)]);
        // Act
        let output = PolygonReader.read(&path, "woodland", EXTENT);
        // Assert
        let report = output.expect_err("should reject layer");
        assert_eq!(*report.current_context(), PolygonReaderError::Layer);
    }

    /// Write a `GeoPackage` holding one layer of geometry.
    fn mock(geometries: Vec<Geometry<f64>>) -> (TempDir, PathBuf) {
        mock_named(geometries, "geom")
    }

    /// Write a `GeoPackage` whose geometry column carries a name.
    fn mock_named(geometries: Vec<Geometry<f64>>, column: &str) -> (TempDir, PathBuf) {
        let directory = tempdir().expect("should create directory");
        let path = directory.path().join("fixture.gpkg");
        PolygonFixture::new(LAYER)
            .with_column(column)
            .with_geometries(geometries)
            .write(&path);
        (directory, path)
    }

    /// Overwrite the `DATETIME` column with text the crate's writer never emits.
    fn set_surveyed(path: &Path, value: &str) {
        let package = GeoPackage::open(path).expect("should open package");
        package
            .connection()
            .execute(
                &format!("UPDATE \"{LAYER}\" SET \"{SURVEYED}\" = ?1"),
                [value],
            )
            .expect("should update column");
    }
}
