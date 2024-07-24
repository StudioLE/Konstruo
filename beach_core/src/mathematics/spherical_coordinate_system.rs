use crate::mathematics::constants::HALF_PI;
use bevy::prelude::*;

pub const RADIAL_AXIS: Vec3 = Vec3::new(1.0, 0.0, 0.0);

pub const POLAR_AXIS: Vec3 = Vec3::new(0.0, 1.0, 0.0);

pub const AZIMUTHAL_AXIS: Vec3 = Vec3::new(0.0, 0.0, 1.0);

pub struct SphericalCoordinateSystem;

impl SphericalCoordinateSystem {
    /// Convert from spherical to cartesian coordinates.
    pub fn spherical_to_cartesian(radius: f32, polar: f32, azimuth: f32) -> Vec3 {
        let x = radius * azimuth.sin() * polar.sin();
        let y = radius * polar.cos();
        let z = radius * azimuth.cos() * polar.sin();
        Vec3::new(x, y, z)
    }

    /// Get the cartesian rotation of spherical coordinates.
    pub fn get_cartesian_rotation(polar: f32, azimuth: f32) -> Vec3 {
        let x = (HALF_PI - polar) * -1.0;
        Vec3::new(x, azimuth, 0.0)
    }
}
