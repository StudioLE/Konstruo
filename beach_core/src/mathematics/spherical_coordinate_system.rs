use crate::mathematics::constants::HALF_PI;
use bevy::prelude::*;

pub const RADIAL_AXIS: Vec3 = Vec3::new(1.0, 0.0, 0.0);

pub const POLAR_AXIS: Vec3 = Vec3::new(0.0, 1.0, 0.0);

pub const AZIMUTHAL_AXIS: Vec3 = Vec3::new(0.0, 0.0, 1.0);

/// Convert from spherical to cartesian coordinates.
/// <https://mathworld.wolfram.com/SphericalCoordinates.html>
pub fn spherical_to_cartesian(radius: f32, polar: f32, azimuth: f32) -> Vec3 {
    let x = radius * azimuth.sin() * polar.sin();
    let y = radius * azimuth.cos() * polar.sin();
    let z = radius * polar.cos();
    Vec3::new(x, y, z)
}

/// Convert from cartesian to spherical coordinates.
/// <https://mathworld.wolfram.com/SphericalCoordinates.html>
pub fn cartesian_to_spherical(vector: Vec3) -> Vec3 {
    let radius = (vector.x.powi(2) + vector.y.powi(2) + vector.z.powi(2)).sqrt();
    let polar = (vector.z / radius).acos();
    let azimuth = (vector.y / vector.x).atan();
    Vec3::new(radius, polar, azimuth)
}

/// Get the cartesian rotation of spherical coordinates.
/// <https://mathworld.wolfram.com/SphericalCoordinates.html>
pub fn get_cartesian_rotation(polar: f32, azimuth: f32) -> Vec3 {
    let x = (HALF_PI - polar) * -1.0;
    Vec3::new(x, azimuth, 0.0)
}
