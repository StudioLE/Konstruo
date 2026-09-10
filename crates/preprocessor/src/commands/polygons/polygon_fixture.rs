//! Build a `GeoPackage` for tests.

use crate::prelude::*;
use geo_types::{Coord, Geometry, LineString, Polygon};
use geopackage::core::types::{ColumnType, GeometryType};
use geopackage::{ColumnSpec, GeoPackage, GeometrySpec, NewFeature, TableSchemaBuilder, Value};

/// Name of the `DATETIME` column every fixture holds.
pub(crate) const SURVEYED: &str = "surveyed";

/// Coordinate reference system every fixture declares.
const SRS_ID: i32 = 27700;

/// Build a `GeoPackage` for tests.
pub(crate) struct PolygonFixture {
    /// Name of the layer.
    layer: String,
    /// Name of the geometry column.
    column: String,
    /// Geometry the layer holds.
    geometries: Vec<Geometry<f64>>,
}

impl PolygonFixture {
    /// Create a new [`PolygonFixture`] of an empty layer.
    pub(crate) fn new(layer: impl Into<String>) -> Self {
        Self {
            layer: layer.into(),
            column: "geom".to_owned(),
            geometries: Vec::new(),
        }
    }

    /// Name the geometry column.
    pub(crate) fn with_column(mut self, column: impl Into<String>) -> Self {
        self.column = column.into();
        self
    }

    /// Add the geometry the layer holds.
    pub(crate) fn with_geometries(mut self, geometries: Vec<Geometry<f64>>) -> Self {
        self.geometries = geometries;
        self
    }

    /// Write the fixture to a path, building a spatial index over it.
    ///
    /// - Adds a layer to the file at the path when one is already there
    pub(crate) fn write(self, path: &Path) {
        let package = if path.exists() {
            GeoPackage::open(path).expect("should open package")
        } else {
            GeoPackage::create(path).expect("should create package")
        };
        package
            .add_epsg_srs(SRS_ID)
            .expect("should register coordinate reference system");
        let builder = TableSchemaBuilder::new(self.layer)
            .geometry(GeometrySpec::new(GeometryType::Geometry, SRS_ID).column_name(self.column))
            .column(ColumnSpec::new(SURVEYED, ColumnType::DateTime));
        let layer = package.create_layer(&builder).expect("should create layer");
        let features: Vec<NewFeature<Geometry<f64>>> = self
            .geometries
            .into_iter()
            .map(|geometry| NewFeature::new(geometry, vec![Value::Null]))
            .collect();
        layer.write_all(features, 0).expect("should write features");
    }
}

/// A one hundred meter square with its south west corner at a position.
pub(crate) fn square(easting: f64, northing: f64) -> Geometry<f64> {
    let corners = [
        (easting, northing),
        (easting + 100.0, northing),
        (easting + 100.0, northing + 100.0),
        (easting, northing + 100.0),
        (easting, northing),
    ];
    let positions: Vec<Coord<f64>> = corners
        .iter()
        .map(|(x, y)| Coord { x: *x, y: *y })
        .collect();
    Geometry::Polygon(Polygon::new(LineString::from(positions), Vec::new()))
}
