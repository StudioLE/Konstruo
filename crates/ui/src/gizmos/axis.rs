use super::*;
use bevy::color::palettes::tailwind;
use bevy::math::Vec3;
use bevy::prelude::Gizmos;

const ORIGIN_LENGTH: f32 = 100.0;

/// A representation of each axis as a gizmo.
pub struct AxisGizmo;

impl AxisGizmo {
    /// Draw a representation of each axis as a gizmo at the origin.
    pub fn draw_at_origin(gizmos: Gizmos<Bold>) {
        Self::draw_at(gizmos, Vec3::ZERO, ORIGIN_LENGTH);
    }

    /// Draw a representation of each axis as a gizmo.
    pub fn draw_at(mut gizmos: Gizmos<Bold>, origin: Vec3, length: f32) {
        gizmos.ray(
            origin - Vec3::X * length * 0.5,
            Vec3::X * length,
            tailwind::RED_700,
        );
        gizmos.ray(
            origin - Vec3::Y * length * 0.5,
            Vec3::Y * length,
            tailwind::GREEN_700,
        );
        gizmos.ray(
            origin - Vec3::Z * length * 0.5,
            Vec3::Z * length,
            tailwind::SKY_700,
        );
    }
}
