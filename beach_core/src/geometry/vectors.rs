use bevy::prelude::*;

pub fn is_almost_zero(vector: Vec3) -> bool {
    vector.abs_diff_eq(Vec3::ZERO, f32::EPSILON)
}

pub fn is_almost_equal_to(a: Vec3, b: Vec3) -> bool {
    a.abs_diff_eq(b, f32::EPSILON)
}
