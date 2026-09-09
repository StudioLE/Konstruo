use bevy::prelude::*;
use konstruo_geometry::Polygon;

/// Polygons loaded from a `GeoJSON` file, in world space.
///
/// Source coordinates are British National Grid (EPSG:27700) and are converted
/// by [`konstruo_geography_core::BngCoordinates`] during load, so no easting
/// or northing reaches downstream code.
#[derive(Asset, Clone, Debug, TypePath)]
pub struct GeoJsonPolygons {
    /// The polygons, with any inner rings held as interior boundaries.
    ///
    /// - Flattens a `GeoJSON` `MultiPolygon` into its member polygons
    pub polygons: Vec<Polygon>,
}
