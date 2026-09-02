use crate::{GeoJsonPolygons, NationalGrid};
use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::prelude::*;
use geojson::{FeatureCollection, GeoJson, Geometry, GeometryValue, JsonValue, Position};
use konstruo_geometry::{LinearRing, LinearRingError, Polygon};
use std::io::Error as IoError;
use std::str::FromStr;
use std::string::FromUtf8Error;
use thiserror::Error;

/// The coordinate reference system this loader reads coordinates as.
const EXPECTED_CRS: &str = "urn:ogc:def:crs:EPSG::27700";

/// The fewest coordinates in a `GeoJSON` position, per RFC 7946 section 3.1.1.
const MINIMUM_POSITION_COORDINATES: usize = 2;

/// The fewest positions in a closed `GeoJSON` ring, per RFC 7946 section 3.1.6.
const MINIMUM_RING_POSITIONS: usize = 4;

/// An [`AssetLoader`] for `GeoJSON` files in British National Grid (EPSG:27700).
///
/// Only polygonal geometry is loaded. Coordinates are converted to world space
/// by [`NationalGrid`].
#[derive(Default, TypePath)]
pub struct GeoJsonLoader;

impl AssetLoader for GeoJsonLoader {
    type Asset = GeoJsonPolygons;
    type Settings = ();
    type Error = GeoJsonLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let text = String::from_utf8(bytes)?;
        let geo_json = GeoJson::from_str(&text)?;
        let polygons = to_polygons(&geo_json)?;
        debug!(
            "Loaded {} polygon(s) from {}",
            polygons.len(),
            load_context.path()
        );
        Ok(GeoJsonPolygons { polygons })
    }

    fn extensions(&self) -> &[&str] {
        &["geojson"]
    }
}

/// Collect every [`Polygon`] in a [`GeoJson`].
///
/// # Errors
///
/// - [`GeoJsonLoaderError`] when polygonal geometry violates RFC 7946
fn to_polygons(geo_json: &GeoJson) -> Result<Vec<Polygon>, GeoJsonLoaderError> {
    let mut polygons = Vec::new();
    match geo_json {
        GeoJson::Geometry(geometry) => push_geometry(geometry, &mut polygons)?,
        GeoJson::Feature(feature) => {
            if let Some(geometry) = feature.geometry.as_ref() {
                push_geometry(geometry, &mut polygons)?;
            }
        }
        GeoJson::FeatureCollection(collection) => {
            warn_unexpected_crs(collection);
            for feature in &collection.features {
                if let Some(geometry) = feature.geometry.as_ref() {
                    push_geometry(geometry, &mut polygons)?;
                }
            }
        }
    }
    Ok(polygons)
}

/// Warn when a collection declares a coordinate reference system other than [`EXPECTED_CRS`].
///
/// - Reads `crs`, a `GeoJSON` 2008 member that RFC 7946 dropped, from the foreign members
/// - Stays silent when the member is absent, which RFC 7946 takes to mean WGS 84
fn warn_unexpected_crs(collection: &FeatureCollection) {
    let Some(crs) = collection
        .foreign_members
        .as_ref()
        .and_then(|members| members.get("crs"))
    else {
        return;
    };
    let Some(name) = crs.pointer("/properties/name").and_then(JsonValue::as_str) else {
        warn!("Ignored a GeoJSON crs member of an unrecognized shape: {crs}");
        return;
    };
    if name != EXPECTED_CRS {
        warn!("GeoJSON declares {name} but coordinates are read as {EXPECTED_CRS}");
    }
}

/// Push every [`Polygon`] in a [`Geometry`] onto `polygons`.
///
/// - Warns and skips geometry that is valid but not polygonal
///
/// # Errors
///
/// - [`GeoJsonLoaderError`] when polygonal geometry violates RFC 7946
fn push_geometry(
    geometry: &Geometry,
    polygons: &mut Vec<Polygon>,
) -> Result<(), GeoJsonLoaderError> {
    match &geometry.value {
        GeometryValue::Polygon { coordinates } => polygons.push(to_polygon(coordinates)?),
        GeometryValue::MultiPolygon { coordinates } => {
            for rings in coordinates {
                polygons.push(to_polygon(rings)?);
            }
        }
        GeometryValue::GeometryCollection { geometries } => {
            for geometry in geometries {
                push_geometry(geometry, polygons)?;
            }
        }
        value => warn!(
            "Skipped GeoJSON geometry of unsupported type: {}",
            value.type_name()
        ),
    }
    Ok(())
}

/// Create a [`Polygon`] from `GeoJSON` rings, the first being the exterior ring.
///
/// # Errors
///
/// - [`GeoJsonLoaderError::MissingExterior`] when `rings` is empty
/// - [`GeoJsonLoaderError`] when any ring violates RFC 7946
fn to_polygon(rings: &[Vec<Position>]) -> Result<Polygon, GeoJsonLoaderError> {
    let mut rings = rings.iter();
    let exterior = rings.next().ok_or(GeoJsonLoaderError::MissingExterior)?;
    let exterior = to_ring(exterior)?;
    let interiors = rings
        .map(|ring| to_ring(ring))
        .collect::<Result<Vec<LinearRing>, GeoJsonLoaderError>>()?;
    Ok(Polygon::new(exterior, interiors))
}

/// Convert a `GeoJSON` ring to a world space [`LinearRing`].
///
/// - Checks closure after conversion, so two eastings that collapse to one `f32` still close
///
/// # Errors
///
/// - [`GeoJsonLoaderError::InvalidRing`] when the ring holds too few positions
/// - [`GeoJsonLoaderError::UnclosedRing`] when the first and last vertices differ
/// - [`GeoJsonLoaderError::InvalidPosition`] when any position holds too few coordinates
fn to_ring(positions: &[Position]) -> Result<LinearRing, GeoJsonLoaderError> {
    if positions.len() < MINIMUM_RING_POSITIONS {
        return Err(GeoJsonLoaderError::InvalidRing(positions.len()));
    }
    let vertices = positions
        .iter()
        .map(to_vertex)
        .collect::<Result<Vec<Vec3>, GeoJsonLoaderError>>()?;
    LinearRing::new(vertices).map_err(GeoJsonLoaderError::from)
}

/// Convert a `GeoJSON` position to a world space vertex on the ground plane.
///
/// - Discards any elevation or measure beyond the easting and northing
///
/// # Errors
///
/// - [`GeoJsonLoaderError::InvalidPosition`] when the position holds too few coordinates
fn to_vertex(position: &Position) -> Result<Vec3, GeoJsonLoaderError> {
    let [easting, northing, ..] = *position.as_slice() else {
        return Err(GeoJsonLoaderError::InvalidPosition(position.len()));
    };
    Ok(NationalGrid::to_world(easting, northing).extend(0.0))
}

/// Errors returned by [`GeoJsonLoader`].
#[derive(Debug, Error)]
pub enum GeoJsonLoaderError {
    /// Failed to read the asset bytes.
    #[error("failed to read GeoJSON asset")]
    Read(#[from] IoError),
    /// The asset bytes are not valid UTF-8.
    #[error("GeoJSON asset is not valid UTF-8")]
    Utf8(#[from] FromUtf8Error),
    /// Failed to parse the asset as `GeoJSON`.
    #[error("failed to parse GeoJSON asset")]
    Parse(#[from] geojson::Error),
    /// A position holds too few coordinates to place a vertex.
    #[error(
        "GeoJSON position holds {0} coordinates, expected at least {MINIMUM_POSITION_COORDINATES}"
    )]
    InvalidPosition(usize),
    /// A ring holds too few positions to enclose an area.
    #[error("GeoJSON ring holds {0} positions, expected at least {MINIMUM_RING_POSITIONS}")]
    InvalidRing(usize),
    /// A ring does not repeat its first position as its last.
    #[error("GeoJSON ring is not closed")]
    UnclosedRing(#[from] LinearRingError),
    /// A polygon holds no exterior ring.
    #[error("GeoJSON polygon holds no exterior ring")]
    MissingExterior,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A square kilometre with a square hole, positioned at the world origin.
    const FIXTURE: &str = r#"{
        "type": "FeatureCollection",
        "features": [{
            "type": "Feature",
            "properties": {},
            "geometry": {
                "type": "Polygon",
                "coordinates": [
                    [[369500.0, 589500.0], [370500.0, 589500.0], [370500.0, 590500.0], [369500.0, 590500.0], [369500.0, 589500.0]],
                    [[369900.0, 589900.0], [369900.0, 590100.0], [370100.0, 590100.0], [370100.0, 589900.0], [369900.0, 589900.0]]
                ]
            }
        }]
    }"#;

    #[test]
    fn geo_json_loader_to_polygons() {
        // Arrange
        let geo_json = GeoJson::from_str(FIXTURE).expect("fixture should parse");

        // Act
        let polygons = to_polygons(&geo_json).expect("fixture should convert");

        // Assert
        assert_eq!(polygons.len(), 1);
        let polygon = polygons.first().expect("polygon should exist");
        assert_eq!(polygon.get_exterior().get_vertices().len(), 5);
        assert_eq!(polygon.get_interiors().len(), 1);
        assert_eq!(
            polygon.get_exterior().get_vertices().first(),
            Some(&Vec3::new(-500.0, -500.0, 0.0))
        );
    }

    /// Geometry that is valid but not polygonal is skipped rather than failing the load.
    #[test]
    fn geo_json_loader_to_polygons_unsupported() {
        // Arrange
        let source = r#"{"type": "Point", "coordinates": [370000.0, 590000.0]}"#;
        let geo_json = GeoJson::from_str(source).expect("source should parse");

        // Act
        let polygons = to_polygons(&geo_json).expect("point should be skipped");

        // Assert
        assert_eq!(polygons, Vec::new());
    }

    #[test]
    fn geo_json_loader_to_ring_too_short() {
        // Arrange
        let positions = vec![Position::from([370_000.0, 590_000.0])];

        // Act
        let output = to_ring(&positions);

        // Assert
        assert!(matches!(output, Err(GeoJsonLoaderError::InvalidRing(1))));
    }

    #[test]
    fn geo_json_loader_to_ring_unclosed() {
        // Arrange
        let positions = vec![
            Position::from([369_500.0, 589_500.0]),
            Position::from([370_500.0, 589_500.0]),
            Position::from([370_500.0, 590_500.0]),
            Position::from([369_500.0, 590_500.0]),
        ];

        // Act
        let output = to_ring(&positions);

        // Assert
        assert!(matches!(
            output,
            Err(GeoJsonLoaderError::UnclosedRing(LinearRingError::NotClosed))
        ));
    }

    /// A position of one coordinate is invalid data, not an absent vertex.
    #[test]
    fn geo_json_loader_to_vertex_too_few_coordinates() {
        // Arrange
        let position = Position::from(vec![370_000.0]);

        // Act
        let output = to_vertex(&position);

        // Assert
        assert!(matches!(
            output,
            Err(GeoJsonLoaderError::InvalidPosition(1))
        ));
    }

    #[test]
    fn geo_json_loader_to_vertex_discards_elevation() {
        let position = Position::from([370_500.0, 590_500.0, 120.0]);
        let vertex = to_vertex(&position).expect("position should convert");
        assert_eq!(vertex, Vec3::new(500.0, 500.0, 0.0));
    }
}
