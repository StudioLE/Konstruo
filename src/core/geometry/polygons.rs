use bevy::math::vec3;
use bevy::prelude::*;

#[allow(clippy::indexing_slicing)]
/// Calculate the area of a polygon.
///
/// If the vertices are in clockwise order, the area will be negative.
///
/// <https://stackoverflow.com/a/18472899/247218>
#[must_use]
pub fn polygon_area_ccw(vertices: &Vec<Vec3>) -> f32 {
    let count = vertices.len();
    let mut sum = 0.0;
    let mut previous = vertices[count - 1];
    for current in vertices {
        sum += (current.x - previous.x) * (current.z + previous.z);
        previous = *current;
    }
    sum / 2.0
}

/// Calculate the area of a polygon.
///
/// The winding order of the vertices does not matter.
#[must_use]
pub fn polygon_area(vertices: &Vec<Vec3>) -> f32 {
    polygon_area_ccw(vertices).abs()
}

/// Check if the vertices of a polygon are in counter-clockwise order.
#[must_use]
pub fn polygon_is_ccw(vertices: &Vec<Vec3>) -> bool {
    polygon_area_ccw(vertices) > 0.0
}

/// Re-order the vertices of a polygon if they aren't ordered counter-clockwise.
pub fn polygon_ensure_ccw(vertices: &mut Vec<Vec3>) {
    if !polygon_is_ccw(vertices) {
        vertices.reverse();
    }
}

#[must_use]
pub fn create_square(origin: Vec3, size: f32) -> [Vec3; 5] {
    let half_size = size / 2.0;
    [
        origin + vec3(-half_size, -half_size, 0.0),
        origin + vec3(half_size, -half_size, 0.0),
        origin + vec3(half_size, half_size, 0.0),
        origin + vec3(-half_size, half_size, 0.0),
        origin + vec3(-half_size, -half_size, 0.0),
    ]
}

#[must_use]
pub fn create_diamond(origin: Vec3, size: f32) -> [Vec3; 5] {
    let hypotenuse = (2.0 * size.powi(2)).sqrt();
    let half = hypotenuse / 2.0;
    [
        origin + vec3(0.0, -half, 0.0),
        origin + vec3(half, 0.0, 0.0),
        origin + vec3(0.0, half, 0.0),
        origin + vec3(-half, 0.0, 0.0),
        origin + vec3(0.0, -half, 0.0),
    ]
}
