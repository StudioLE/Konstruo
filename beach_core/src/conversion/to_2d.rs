use bevy::prelude::*;

pub fn vector_to_2d(vector: Vec3) -> Vec2 {
    if vector.y != 0.0 {
        warn!(
            "Y value of Vec3 should be zero when converting to 2d: {}",
            vector.y
        );
    }
    Vec2::new(vector.x, vector.z)
}

pub fn vectors_to_2d(vertices: Vec<Vec3>) -> Vec<Vec2> {
    vertices
        .iter()
        .map(|&vertex| vector_to_2d(vertex))
        .collect()
}
