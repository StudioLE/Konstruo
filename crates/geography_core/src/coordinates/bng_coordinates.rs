//! Coordinates on the British National Grid.

use glam::{DVec2, Vec2};

/// Coordinates on the British National Grid.
///
/// - Reads as OSGB36 / British National Grid, EPSG:27700
/// - Distinct from world space, so neither is passed where the other is expected
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BngCoordinates {
    /// Distance east of the false origin, in meters.
    pub easting: f64,
    /// Distance north of the false origin, in meters.
    pub northing: f64,
}

impl BngCoordinates {
    /// Coordinates of world `(0, 0)`.
    ///
    /// - Sits at the center of Kielder Water
    /// - Lands on a 10 km grid intersection, so Ordnance Survey tile corners
    ///   fall on round world coordinates
    /// - Every chunk written by the preprocessor is relative to it
    pub const ORIGIN: Self = Self {
        easting: 370_000.0,
        northing: 590_000.0,
    };

    /// Create a new [`BngCoordinates`].
    #[must_use]
    pub const fn new(easting: f64, northing: f64) -> Self {
        Self { easting, northing }
    }

    /// Convert to a world space position, in meters.
    ///
    /// - Subtracts in `f64` before casting to `f32`
    #[must_use]
    pub fn to_world(self) -> Vec2 {
        DVec2::new(
            self.easting - Self::ORIGIN.easting,
            self.northing - Self::ORIGIN.northing,
        )
        .as_vec2()
    }

    /// Create a new [`BngCoordinates`] from a world space position.
    #[must_use]
    pub fn from_world(position: Vec2) -> Self {
        Self {
            easting: f64::from(position.x) + Self::ORIGIN.easting,
            northing: f64::from(position.y) + Self::ORIGIN.northing,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// South west corner of the Kielder Water outer bounds.
    const CORNER: BngCoordinates = BngCoordinates::new(363_048.85, 585_619.34);

    /// Largest easting or northing error allowed after a round trip.
    const TOLERANCE: f64 = 0.01;

    #[test]
    fn bng_coordinates_to_world_origin() {
        let world = BngCoordinates::ORIGIN.to_world();
        assert_eq!(world, Vec2::ZERO);
    }

    #[test]
    fn bng_coordinates_to_world_corner() {
        let world = CORNER.to_world();
        assert_eq!(world, Vec2::new(-6951.15, -4380.66));
    }

    #[test]
    fn bng_coordinates_from_world() {
        // Arrange
        let world = CORNER.to_world();
        // Act
        let output = BngCoordinates::from_world(world);
        // Assert
        assert!((output.easting - CORNER.easting).abs() < TOLERANCE);
        assert!((output.northing - CORNER.northing).abs() < TOLERANCE);
    }
}
