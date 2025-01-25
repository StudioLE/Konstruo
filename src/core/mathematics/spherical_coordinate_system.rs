use crate::core::mathematics::constants::HALF_PI;
use crate::core::mathematics::floats::fix_floating;
use bevy::prelude::*;

pub const RADIAL_AXIS: Vec3 = Vec3::new(1.0, 0.0, 0.0);

pub const POLAR_AXIS: Vec3 = Vec3::new(0.0, 1.0, 0.0);

pub const AZIMUTHAL_AXIS: Vec3 = Vec3::new(0.0, 0.0, 1.0);

/// Convert from spherical to cartesian coordinates.
/// <https://mathworld.wolfram.com/SphericalCoordinates.html>
#[must_use]
pub fn spherical_to_cartesian(radius: f32, polar: f32, azimuth: f32) -> Vec3 {
    let x = radius * polar.sin() * azimuth.cos();
    let y = radius * polar.sin() * azimuth.sin();
    let z = radius * polar.cos();
    Vec3::new(fix_floating(x), fix_floating(y), fix_floating(z))
}

/// Convert from cartesian to spherical coordinates.
/// <https://mathworld.wolfram.com/SphericalCoordinates.html>
/// Uses atan2 because programming is superior to mathematics:
/// <https://en.wikipedia.org/wiki/Atan2>
#[must_use]
pub fn cartesian_to_spherical(vector: Vec3) -> Vec3 {
    let radius = (vector.x.powi(2) + vector.y.powi(2) + vector.z.powi(2)).sqrt();
    let polar = (vector.z / radius).acos();
    let azimuth = vector.y.atan2(vector.x);
    Vec3::new(
        fix_floating(radius),
        fix_floating(polar),
        fix_floating(azimuth),
    )
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
    use crate::core::mathematics::constants::*;

    #[test]
    fn _spherical_to_cartesian__polar() {
        assert_eq!(
            spherical_to_cartesian(1.0, -QUARTER_PI, QUARTER_PI),
            Vec3::new(-0.5, -0.5, ONE_OVER_ROOT_2)
        );
        assert_eq!(
            spherical_to_cartesian(1.0, 0.0, QUARTER_PI),
            Vec3::new(0.0, 0.0, 1.0)
        );
        assert_eq!(
            spherical_to_cartesian(1.0, QUARTER_PI, QUARTER_PI),
            Vec3::new(0.5, 0.5, ONE_OVER_ROOT_2)
        );
        assert_eq!(
            spherical_to_cartesian(1.0, HALF_PI, QUARTER_PI),
            Vec3::new(ONE_OVER_ROOT_2, ONE_OVER_ROOT_2, 0.0)
        );
        assert_eq!(
            spherical_to_cartesian(1.0, PI, QUARTER_PI),
            Vec3::new(0.0, 0.0, -1.0)
        );
    }

    #[test]
    fn _spherical_to_cartesian__azimuth() {
        assert_eq!(
            spherical_to_cartesian(1.0, QUARTER_PI, -QUARTER_PI),
            Vec3::new(0.5, -0.5, ONE_OVER_ROOT_2)
        );
        assert_eq!(
            spherical_to_cartesian(1.0, QUARTER_PI, 0.0),
            Vec3::new(ONE_OVER_ROOT_2, 0.0, ONE_OVER_ROOT_2)
        );
        assert_eq!(
            spherical_to_cartesian(1.0, QUARTER_PI, QUARTER_PI),
            Vec3::new(0.5, 0.5, ONE_OVER_ROOT_2)
        );
        assert_eq!(
            spherical_to_cartesian(1.0, QUARTER_PI, HALF_PI),
            Vec3::new(0.0, ONE_OVER_ROOT_2, ONE_OVER_ROOT_2)
        );
        assert_eq!(
            spherical_to_cartesian(1.0, QUARTER_PI, PI),
            Vec3::new(-ONE_OVER_ROOT_2, 0.0, ONE_OVER_ROOT_2)
        );
        assert_eq!(
            spherical_to_cartesian(1.0, QUARTER_PI, PI + HALF_PI),
            Vec3::new(0.0, -ONE_OVER_ROOT_2, ONE_OVER_ROOT_2)
        );
        assert_eq!(
            spherical_to_cartesian(1.0, QUARTER_PI, TWO_PI),
            Vec3::new(ONE_OVER_ROOT_2, 0.0, ONE_OVER_ROOT_2)
        );
    }

    #[test]
    fn _cartesian_to_spherical() {
        assert_eq!(
            cartesian_to_spherical(Vec3::new(0.0, ONE_OVER_ROOT_2, ONE_OVER_ROOT_2)),
            // Vec3::new(1.0, QUARTER_PI, 0.0)
            Vec3::new(1.0, QUARTER_PI, HALF_PI)
        );
        assert_eq!(
            cartesian_to_spherical(Vec3::new(ONE_OVER_ROOT_2, 0.0, ONE_OVER_ROOT_2)),
            // Vec3::new(1.0, QUARTER_PI, HALF_PI)
            Vec3::new(1.0, QUARTER_PI, 0.0)
        );
        assert_eq!(
            cartesian_to_spherical(Vec3::new(0.0, -ONE_OVER_ROOT_2, ONE_OVER_ROOT_2)),
            // Vec3::new(1.0, QUARTER_PI, PI)
            Vec3::new(1.0, QUARTER_PI, -HALF_PI)
        );
        assert_eq!(
            cartesian_to_spherical(Vec3::new(-ONE_OVER_ROOT_2, 0.0, ONE_OVER_ROOT_2)),
            // Vec3::new(1.0, QUARTER_PI, PI + HALF_PI)
            // Vec3::new(1.0, QUARTER_PI, 0.0)
            Vec3::new(1.0, QUARTER_PI, PI) // TODO: Incorrect quadrant
        );
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
