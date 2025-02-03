use crate::mathematics::floats::fix_floating;
use bevy::prelude::*;

const EPSILON: f32 = 1e-6;

#[must_use]
pub fn is_almost_zero(vector: Vec3) -> bool {
    vector.abs_diff_eq(Vec3::ZERO, EPSILON)
}

#[must_use]
pub fn is_almost_equal_to(a: Vec3, b: Vec3) -> bool {
    a.abs_diff_eq(b, EPSILON)
}

#[must_use]
pub fn fix_floating_vectors(vector: Vec3) -> Vec3 {
    let x = fix_floating(vector.x);
    let y = fix_floating(vector.y);
    let z = fix_floating(vector.z);
    Vec3::new(x, y, z)
}

/// Calculate the angle between two vectors on a plane.
/// Assumes `plane_normal` is normalized.
///
/// Positive angle typically means that `v1` counter-clockwise relative to `v2` around the normal of the plane.
/// Negative Angle generally means that `v1` is clockwise relative to `v2` around the normal of the plane.
#[must_use]
#[cfg(test)]
pub fn angle_between_on_plane(v1: Vec3, v2: Vec3, plane_normal: Vec3) -> f32 {
    // Project vectors onto the plane
    let v1_proj = v1 - plane_normal * v1.dot(plane_normal);
    let v2_proj = v2 - plane_normal * v2.dot(plane_normal);

    // Normalize the projected vectors
    let v1_proj_norm = v1_proj.normalize();
    let v2_proj_norm = v2_proj.normalize();

    // Calculate the angle using the dot product
    let cos_theta = v1_proj_norm.dot(v2_proj_norm);
    let angle = cos_theta.acos();

    // Optional: Determine the direction using the cross product
    let cross = v1_proj_norm.cross(v2_proj_norm);
    if cross.dot(plane_normal) < 0.0 {
        // If cross is in the opposite direction of the plane normal, make the angle negative
        -angle
    } else {
        angle
    }
}
