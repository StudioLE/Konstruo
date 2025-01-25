use bevy::math::{vec3, Vec2, Vec3};

#[must_use]
pub fn vector_to_3d(vector: Vec2) -> Vec3 {
    vec3(vector.x, vector.y, 0.0)
}

#[must_use]
pub fn vectors_to_3d(vertices: Vec<Vec2>) -> Vec<Vec3> {
    vertices
        .iter()
        .map(|&vertex| vector_to_3d(vertex))
        .collect()
}
