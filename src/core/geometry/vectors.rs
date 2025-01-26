use crate::mathematics::floats::fix_floating;
use bevy::prelude::*;

#[must_use]
pub fn is_almost_zero(vector: Vec3) -> bool {
    vector.abs_diff_eq(Vec3::ZERO, f32::EPSILON)
}

#[must_use]
pub fn is_almost_equal_to(a: Vec3, b: Vec3) -> bool {
    a.abs_diff_eq(b, f32::EPSILON)
}

#[must_use]
pub fn fix_floating_vectors(vector: Vec3) -> Vec3 {
    let x = fix_floating(vector.x);
    let y = fix_floating(vector.y);
    let z = fix_floating(vector.z);
    Vec3::new(x, y, z)
}
