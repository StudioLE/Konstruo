use bevy::math::Vec3;
use bevy::prelude::CubicBezier;
use kurbo::{CubicBez, Point};

/// Convert from a Kurbo [`Point`] to a [`Vec3`].
pub fn vec3_from_kurbo(point: Point) -> Vec3 {
    Vec3::new(point.x as f32, 0.0, point.y as f32)
}

/// Convert from a collection of Kurbo [`CubicBez`] to a [`CubicBezier`].
pub fn bezier_from_kurbo(segments: Vec<CubicBez>) -> CubicBezier<Vec3> {
    let points: Vec<[Vec3; 4]> = segments
        .iter()
        .map(|cubic| {
            [
                vec3_from_kurbo(cubic.p0),
                vec3_from_kurbo(cubic.p1),
                vec3_from_kurbo(cubic.p2),
                vec3_from_kurbo(cubic.p3),
            ]
        })
        .collect();
    CubicBezier::new(points)
}
