use crate::mathematics::HALF_PI;
use bevy::prelude::*;

pub const RADIAL_AXIS: Vec3 = Vec3::new(1.0, 0.0, 0.0);

pub const POLAR_AXIS: Vec3 = Vec3::new(0.0, 1.0, 0.0);

pub const AZIMUTHAL_AXIS: Vec3 = Vec3::new(0.0, 0.0, 1.0);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SphericalCoordinates {
    pub vector: Vec3,
}

impl From<Vec3> for SphericalCoordinates {
    fn from(vector: Vec3) -> Self {
        Self { vector }
    }
}

impl SphericalCoordinates {
    /// Create new [`SphericalCoordinates`].
    #[must_use]
    pub fn new(radius: f32, polar: f32, azimuth: f32) -> Self {
        Self {
            vector: Vec3::new(radius, polar, azimuth),
        }
    }

    /// Distance from the origin.
    #[must_use]
    pub fn get_radius(&self) -> f32 {
        self.vector.x
    }

    /// Angle from the zenith in radians.
    ///
    /// Also known as the polar angle or zenith angle
    /// - <https://mathworld.wolfram.com/PolarAngle.html>
    /// - <https://mathworld.wolfram.com/ZenithAngle.html>
    ///
    /// Zenith is the positive Z axis or north pole.
    #[must_use]
    pub fn get_polar(&self) -> f32 {
        self.vector.y
    }

    /// Angle from the horizon in radians.
    /// - <https://mathworld.wolfram.com/Colatitude.html>
    #[must_use]
    pub fn get_altitude(&self) -> f32 {
        HALF_PI - self.get_polar()
    }

    /// Angle from the X axis in the XY plane.
    /// -<https://mathworld.wolfram.com/Azimuth.html>
    #[must_use]
    pub fn get_azimuth(&self) -> f32 {
        self.vector.z
    }

    /// Convert from spherical to cartesian coordinates.
    /// <https://mathworld.wolfram.com/SphericalCoordinates.html>
    #[must_use]
    pub fn to_cartesian(&self) -> Vec3 {
        let radius = self.get_radius();
        let polar = self.get_polar();
        let azimuth = self.get_azimuth();
        let x = radius * polar.sin() * azimuth.cos();
        let y = radius * polar.sin() * azimuth.sin();
        let z = radius * polar.cos();
        Vec3::new(x, y, z)
    }

    /// Convert from cartesian to spherical coordinates.
    /// <https://mathworld.wolfram.com/SphericalCoordinates.html>
    /// Uses atan2 because programming is superior to mathematics:
    /// <https://en.wikipedia.org/wiki/Atan2>
    #[must_use]
    pub fn from_cartesian(vector: Vec3) -> Self {
        let radius = (vector.x.powi(2) + vector.y.powi(2) + vector.z.powi(2)).sqrt();
        let polar = (vector.z / radius).acos();
        let azimuth = vector.y.atan2(vector.x);
        Self {
            vector: Vec3::new(radius, polar, azimuth),
        }
    }
}

/// Get the cartesian rotation of spherical coordinates.
/// <https://mathworld.wolfram.com/SphericalCoordinates.html>
#[must_use]
pub fn get_cartesian_rotation(polar: f32, azimuth: f32) -> Vec3 {
    let x = (HALF_PI - polar) * -1.0;
    Vec3::new(x, azimuth, 0.0)
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use crate::geometry::vectors::{angle_between_on_plane, is_almost_equal_to};
    use crate::mathematics::constants::*;

    fn assert_almost(expected: Vec3, actual: Vec3) {
        let result = is_almost_equal_to(expected, actual);
        assert!(result, "Expected: {expected}, Actual: {actual}");
    }

    #[allow(clippy::absolute_paths)]
    fn assert_f32_almost(expected: f32, actual: f32) {
        let result = (expected - actual).abs() < 1e-3;
        assert!(result, "Expected: {expected}, Actual: {actual}");
    }

    #[test]
    fn _spherical_to_cartesian__polar() {
        let actual = SphericalCoordinates::new(1.0, -QUARTER_PI, QUARTER_PI).to_cartesian();
        let expected = Vec3::new(-0.5, -0.5, ONE_OVER_ROOT_2);
        assert_almost(expected, actual);
        let actual = SphericalCoordinates::new(1.0, 0.0, QUARTER_PI).to_cartesian();
        let expected = Vec3::new(0.0, 0.0, 1.0);
        assert_almost(expected, actual);
        let actual = SphericalCoordinates::new(1.0, QUARTER_PI, QUARTER_PI).to_cartesian();
        let expected = Vec3::new(0.5, 0.5, ONE_OVER_ROOT_2);
        assert_almost(expected, actual);
        let actual = SphericalCoordinates::new(1.0, HALF_PI, QUARTER_PI).to_cartesian();
        let expected = Vec3::new(ONE_OVER_ROOT_2, ONE_OVER_ROOT_2, 0.0);
        assert_almost(expected, actual);
        let actual = SphericalCoordinates::new(1.0, PI, QUARTER_PI).to_cartesian();
        let expected = Vec3::new(0.0, 0.0, -1.0);
        assert_almost(expected, actual);
    }

    #[test]
    fn _spherical_to_cartesian__azimuth() {
        let actual = SphericalCoordinates::new(1.0, QUARTER_PI, -QUARTER_PI).to_cartesian();
        let expected = Vec3::new(0.5, -0.5, ONE_OVER_ROOT_2);
        assert_almost(expected, actual);
        let actual = SphericalCoordinates::new(1.0, QUARTER_PI, 0.0).to_cartesian();
        let expected = Vec3::new(ONE_OVER_ROOT_2, 0.0, ONE_OVER_ROOT_2);
        assert_almost(expected, actual);
        let actual = SphericalCoordinates::new(1.0, QUARTER_PI, QUARTER_PI).to_cartesian();
        let expected = Vec3::new(0.5, 0.5, ONE_OVER_ROOT_2);
        assert_almost(expected, actual);
        let actual = SphericalCoordinates::new(1.0, QUARTER_PI, HALF_PI).to_cartesian();
        let expected = Vec3::new(0.0, ONE_OVER_ROOT_2, ONE_OVER_ROOT_2);
        assert_almost(expected, actual);
        let actual = SphericalCoordinates::new(1.0, QUARTER_PI, PI).to_cartesian();
        let expected = Vec3::new(-ONE_OVER_ROOT_2, 0.0, ONE_OVER_ROOT_2);
        assert_almost(expected, actual);
        let actual = SphericalCoordinates::new(1.0, QUARTER_PI, PI + HALF_PI).to_cartesian();
        let expected = Vec3::new(0.0, -ONE_OVER_ROOT_2, ONE_OVER_ROOT_2);
        assert_almost(expected, actual);
        let actual = SphericalCoordinates::new(1.0, QUARTER_PI, TWO_PI).to_cartesian();
        let expected = Vec3::new(ONE_OVER_ROOT_2, 0.0, ONE_OVER_ROOT_2);
        assert_almost(expected, actual);
    }

    #[test]
    fn _cartesian_to_spherical() {
        // Top back
        let cartesian = Vec3::new(0.0, ONE_OVER_ROOT_2, ONE_OVER_ROOT_2);
        let spherical = SphericalCoordinates::from_cartesian(cartesian);
        let expected = SphericalCoordinates::new(1.0, QUARTER_PI, HALF_PI);
        assert_almost(expected.vector, spherical.vector);
        let round_trip = spherical.to_cartesian();
        assert_almost(cartesian, round_trip);
        let angle = angle_between_on_plane(Vec3::X, cartesian, Vec3::Z);
        assert_f32_almost(HALF_PI, angle);

        // Top right
        let cartesian = Vec3::new(ONE_OVER_ROOT_2, 0.0, ONE_OVER_ROOT_2);
        let spherical = SphericalCoordinates::from_cartesian(cartesian);
        let expected = SphericalCoordinates::new(1.0, QUARTER_PI, 0.0);
        assert_almost(expected.vector, spherical.vector);
        let round_trip = spherical.to_cartesian();
        assert_almost(cartesian, round_trip);
        let angle = angle_between_on_plane(Vec3::X, cartesian, Vec3::Z);
        assert_f32_almost(0.0, angle);

        // Top front
        let cartesian = Vec3::new(0.0, -ONE_OVER_ROOT_2, ONE_OVER_ROOT_2);
        let spherical = SphericalCoordinates::from_cartesian(cartesian);
        let expected = SphericalCoordinates::new(1.0, QUARTER_PI, -HALF_PI);
        assert_almost(expected.vector, spherical.vector);
        let round_trip = spherical.to_cartesian();
        assert_almost(cartesian, round_trip);
        let angle = angle_between_on_plane(Vec3::X, cartesian, Vec3::Z);
        assert_f32_almost(-HALF_PI, angle);

        // Top left
        let cartesian = Vec3::new(-ONE_OVER_ROOT_2, 0.0, ONE_OVER_ROOT_2);
        let spherical = SphericalCoordinates::from_cartesian(cartesian);
        let expected = SphericalCoordinates::new(1.0, QUARTER_PI, PI);
        assert_almost(expected.vector, spherical.vector);
        let round_trip = spherical.to_cartesian();
        assert_almost(cartesian, round_trip);
        let angle = angle_between_on_plane(Vec3::X, cartesian, Vec3::Z);
        assert_f32_almost(PI, angle);
    }

    #[test]
    fn get_cartesian_rotation__polar() {
        assert_eq!(
            get_cartesian_rotation(0.0, QUARTER_PI),
            Vec3::new(-HALF_PI, QUARTER_PI, 0.0)
        );
        assert_eq!(
            get_cartesian_rotation(QUARTER_PI, QUARTER_PI),
            Vec3::new(-QUARTER_PI, QUARTER_PI, 0.0)
        );
        assert_eq!(
            get_cartesian_rotation(HALF_PI, QUARTER_PI),
            Vec3::new(0.0, QUARTER_PI, 0.0)
        );
        assert_eq!(
            get_cartesian_rotation(PI, QUARTER_PI),
            Vec3::new(HALF_PI, QUARTER_PI, 0.0)
        );
        assert_eq!(
            get_cartesian_rotation(-QUARTER_PI, QUARTER_PI),
            Vec3::new(-HALF_PI - QUARTER_PI, QUARTER_PI, 0.0)
        );
    }

    #[test]
    fn get_cartesian_rotation__azimuth() {
        assert_eq!(
            get_cartesian_rotation(QUARTER_PI, 0.0),
            Vec3::new(-QUARTER_PI, 0.0, 0.0)
        );
        assert_eq!(
            get_cartesian_rotation(QUARTER_PI, QUARTER_PI),
            Vec3::new(-QUARTER_PI, QUARTER_PI, 0.0)
        );
        assert_eq!(
            get_cartesian_rotation(QUARTER_PI, HALF_PI),
            Vec3::new(-QUARTER_PI, HALF_PI, 0.0)
        );
        assert_eq!(
            get_cartesian_rotation(QUARTER_PI, PI),
            Vec3::new(-QUARTER_PI, PI, 0.0)
        );
        assert_eq!(
            get_cartesian_rotation(QUARTER_PI, -QUARTER_PI),
            Vec3::new(-QUARTER_PI, -QUARTER_PI, 0.0)
        );
    }
}
