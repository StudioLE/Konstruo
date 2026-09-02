use bevy::math::{DVec2, Vec2};

/// Conversion between British National Grid and world space.
///
/// - Reads easting and northing as EPSG:27700
/// - Translates by [`NationalGrid::ORIGIN`], world space being in meters
pub struct NationalGrid;

impl NationalGrid {
    /// Easting and northing of the world origin in EPSG:27700.
    ///
    /// - Sits at the center of Kielder Water
    /// - Lands on a 10 km grid intersection, so Ordnance Survey tile corners
    ///   fall on round world coordinates
    pub const ORIGIN: DVec2 = DVec2::new(370_000.0, 590_000.0);

    /// Convert an EPSG:27700 easting and northing to a world space position.
    ///
    /// - Subtracts in `f64` before casting to `f32`
    #[must_use]
    pub fn to_world(easting: f64, northing: f64) -> Vec2 {
        (DVec2::new(easting, northing) - Self::ORIGIN).as_vec2()
    }

    /// Convert a world space position to an EPSG:27700 easting and northing.
    #[must_use]
    pub fn from_world(position: Vec2) -> DVec2 {
        position.as_dvec2() + Self::ORIGIN
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Easting of the south west corner of the Kielder Water outer bounds.
    const CORNER_EASTING: f64 = 363_048.85;

    /// Northing of the south west corner of the Kielder Water outer bounds.
    const CORNER_NORTHING: f64 = 585_619.34;

    /// Largest easting or northing error allowed after a round trip.
    const TOLERANCE: f64 = 0.01;

    #[test]
    fn national_grid_to_world_origin() {
        let world = NationalGrid::to_world(NationalGrid::ORIGIN.x, NationalGrid::ORIGIN.y);
        assert_eq!(world, Vec2::ZERO);
    }

    #[test]
    fn national_grid_to_world_corner() {
        let world = NationalGrid::to_world(CORNER_EASTING, CORNER_NORTHING);
        assert_eq!(world, Vec2::new(-6951.15, -4380.66));
    }

    #[test]
    fn national_grid_from_world() {
        // Arrange
        let world = NationalGrid::to_world(CORNER_EASTING, CORNER_NORTHING);

        // Act
        let position = NationalGrid::from_world(world);

        // Assert
        assert!((position.x - CORNER_EASTING).abs() < TOLERANCE);
        assert!((position.y - CORNER_NORTHING).abs() < TOLERANCE);
    }
}
