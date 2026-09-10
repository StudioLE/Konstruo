//! Write polygons as a `GeoJSON` feature collection.

use crate::prelude::*;
use geo_types::{Coord, Geometry, LineString, MultiPolygon, Polygon};
use geojson::{
    Feature, FeatureCollection, Geometry as JsonGeometry, GeometryValue, JsonObject, JsonValue,
};
use std::fs::{create_dir_all, write};

/// Coordinate reference system every written file declares.
const CRS: &str = "urn:ogc:def:crs:EPSG::27700";

/// Smallest positions a ring may hold, the last repeating the first.
const MIN_POSITIONS: usize = 4;

/// Write polygons as a `GeoJSON` feature collection.
#[derive(Default, FromServices)]
pub struct PolygonWriter;

impl PolygonWriter {
    /// Write every polygon to a file.
    ///
    /// - Rounds each ordinate to two decimal places
    /// - Removes the consecutive duplicates rounding creates
    /// - Warns and skips a ring left too short to close
    /// - Writes an empty collection when no polygon survives
    ///
    /// # Errors
    ///
    /// - IF the parent directory cannot be created
    /// - IF the file cannot be written
    #[expect(clippy::unused_self, reason = "resolved as a service")]
    pub fn write(
        &self,
        path: &Path,
        geometries: &[Geometry<f64>],
    ) -> Result<(), Report<PolygonWriterError>> {
        let features: Vec<Feature> = geometries
            .iter()
            .filter_map(get_rounded_geometry)
            .map(get_feature)
            .collect();
        if features.is_empty() {
            warn!(path = %path.display(), "Writing an empty feature collection");
        }
        let collection = FeatureCollection {
            bbox: None,
            features,
            foreign_members: Some(get_foreign_members()),
        };
        if let Some(directory) = path.parent() {
            create_dir_all(directory)
                .change_context(PolygonWriterError::CreateDirectory)
                .attach_path(directory)?;
        }
        write(path, collection.to_string())
            .change_context(PolygonWriterError::Write)
            .attach_path(path)?;
        debug!(path = %path.display(), count = collection.features.len(), "Wrote polygons");
        Ok(())
    }
}

/// Create a [`Feature`] of a geometry, holding no properties.
fn get_feature(geometry: Geometry<f64>) -> Feature {
    Feature {
        bbox: None,
        geometry: Some(JsonGeometry::new(GeometryValue::from(&geometry))),
        id: None,
        properties: Some(JsonObject::new()),
        foreign_members: None,
    }
}

/// Round a geometry, dropping it when no ring survives.
fn get_rounded_geometry(geometry: &Geometry<f64>) -> Option<Geometry<f64>> {
    match geometry {
        Geometry::Polygon(polygon) => get_rounded_polygon(polygon).map(Geometry::Polygon),
        Geometry::MultiPolygon(multi_polygon) => {
            let polygons: Vec<Polygon<f64>> = multi_polygon
                .iter()
                .filter_map(get_rounded_polygon)
                .collect();
            if polygons.is_empty() {
                return None;
            }
            Some(Geometry::MultiPolygon(MultiPolygon::new(polygons)))
        }
        _ => {
            warn!("Skipped a geometry that is neither a polygon nor a multipolygon");
            None
        }
    }
}

/// Round a polygon, dropping it when its exterior ring does not survive.
///
/// - Drops an interior ring on its own, a polygon having no need of one
fn get_rounded_polygon(polygon: &Polygon<f64>) -> Option<Polygon<f64>> {
    let exterior = get_rounded_ring(polygon.exterior())?;
    let interiors: Vec<LineString<f64>> = polygon
        .interiors()
        .iter()
        .filter_map(get_rounded_ring)
        .collect();
    Some(Polygon::new(exterior, interiors))
}

/// Round a ring, dropping it when too few positions remain.
///
/// - Deduplicates after rounding, rounding being what creates the duplicates
fn get_rounded_ring(ring: &LineString<f64>) -> Option<LineString<f64>> {
    let mut positions: Vec<Coord<f64>> = ring
        .coords()
        .map(|coord| Coord {
            x: get_rounded_ordinate(coord.x),
            y: get_rounded_ordinate(coord.y),
        })
        .collect();
    positions.dedup();
    if positions.len() < MIN_POSITIONS {
        warn!(
            positions = positions.len(),
            "Skipped a ring of too few positions"
        );
        return None;
    }
    Some(LineString::from(positions))
}

/// Round an ordinate to two decimal places, halves away from zero.
fn get_rounded_ordinate(ordinate: f64) -> f64 {
    (ordinate * 100.0).round() / 100.0
}

/// Foreign members declaring the coordinate reference system.
fn get_foreign_members() -> JsonObject {
    let mut properties = JsonObject::new();
    properties.insert("name".to_owned(), JsonValue::from(CRS));
    let mut crs = JsonObject::new();
    crs.insert("type".to_owned(), JsonValue::from("name"));
    crs.insert("properties".to_owned(), JsonValue::Object(properties));
    let mut members = JsonObject::new();
    members.insert("crs".to_owned(), JsonValue::Object(crs));
    members
}

/// Errors returned by [`PolygonWriter`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum PolygonWriterError {
    /// Unable to create directory.
    #[error("Unable to create directory")]
    CreateDirectory,
    /// Unable to write file.
    #[error("Unable to write file")]
    Write,
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;
    use std::fs::read_to_string;
    use std::str::FromStr;
    use tempfile::tempdir;

    /// The whole collection, one polygon of one ring, holds the coordinate
    /// reference system and no properties.
    #[test]
    fn polygon_writer_write() {
        // Act
        let output = mock(&[square(0.0)]);
        // Assert
        assert_yaml_snapshot!(output);
    }

    #[test]
    fn polygon_writer_write_rounding() {
        // Act
        let output = mock(&[square(0.123_456)]);
        // Assert
        assert_eq!(
            get_ring(&output),
            vec![0.12, 0.12, 100.12, 0.12, 100.12, 100.12, 0.12, 100.12, 0.12, 0.12]
        );
    }

    /// A pair of positions rounding leaves identical collapses into one.
    #[test]
    fn polygon_writer_write_duplicate() {
        // Arrange
        let ring = vec![
            (0.0, 0.0),
            (100.0, 0.0),
            (100.0032, 0.0),
            (100.0, 100.0),
            (0.0, 100.0),
            (0.0, 0.0),
        ];
        // Act
        let output = mock(&[polygon(&ring)]);
        // Assert
        assert_eq!(get_ring(&output).len(), 10);
    }

    /// A ring left under four positions is skipped rather than written.
    #[test]
    fn polygon_writer_write_short_ring() {
        // Arrange
        let ring = vec![(0.0, 0.0), (0.001, 0.0), (0.002, 0.0), (0.0, 0.0)];
        // Act
        let output = mock(&[polygon(&ring)]);
        // Assert
        assert_eq!(get_features(&output).len(), 0);
    }

    /// No geometry still writes a collection, so the game finds a file.
    #[test]
    fn polygon_writer_write_empty() {
        // Act
        let output = mock(&[]);
        // Assert
        assert_eq!(get_features(&output).len(), 0);
        assert_eq!(
            output
                .pointer("/crs/properties/name")
                .and_then(JsonValue::as_str),
            Some(CRS)
        );
    }

    /// Write geometry to a temporary file and read back what landed.
    fn mock(geometries: &[Geometry<f64>]) -> JsonValue {
        let directory = tempdir().expect("should create directory");
        let path = directory.path().join("polygons").join("fixture.geojson");
        PolygonWriter
            .write(&path, geometries)
            .expect("should write polygons");
        let contents = read_to_string(&path).expect("should read file");
        JsonValue::from_str(&contents).expect("should parse file")
    }

    /// A one hundred meter square with its south west corner at an offset.
    fn square(offset: f64) -> Geometry<f64> {
        polygon(&[
            (offset, offset),
            (100.0 + offset, offset),
            (100.0 + offset, 100.0 + offset),
            (offset, 100.0 + offset),
            (offset, offset),
        ])
    }

    /// A polygon of one exterior ring.
    fn polygon(ring: &[(f64, f64)]) -> Geometry<f64> {
        let positions: Vec<Coord<f64>> = ring.iter().map(|(x, y)| Coord { x: *x, y: *y }).collect();
        Geometry::Polygon(Polygon::new(LineString::from(positions), Vec::new()))
    }

    /// Every feature of a collection.
    fn get_features(collection: &JsonValue) -> &Vec<JsonValue> {
        collection
            .pointer("/features")
            .and_then(JsonValue::as_array)
            .expect("should hold features")
    }

    /// Every ordinate of the exterior ring of the first feature, in order.
    fn get_ring(collection: &JsonValue) -> Vec<f64> {
        collection
            .pointer("/features/0/geometry/coordinates/0")
            .and_then(JsonValue::as_array)
            .expect("should hold a ring")
            .iter()
            .filter_map(JsonValue::as_array)
            .flatten()
            .filter_map(JsonValue::as_f64)
            .collect()
    }
}
