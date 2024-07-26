use bevy::log::warn;
use bevy::math::Vec3;
use bevy::prelude::CubicBezier;
use kurbo::{BezPath, CubicBez, PathSeg, Point, Shape};

/// Convert from a [`Vec3`] to a kurbo [`Point`].
fn vec3_to_kurbo(vector: Vec3) -> Point {
    if vector.z != 0.0 {
        warn!(
            "Kurbo only supports 2D coordinates. Ignoring Z value: {}",
            vector.z
        );
    }
    Point::new(f64::from(vector.x), f64::from(vector.y))
}

/// Convert from a [`CubicBezier`] to a collection of kurbo [`CubicBez`].
pub fn bezier_to_kurbo(bezier: &CubicBezier<Vec3>) -> Vec<CubicBez> {
    bezier
        .control_points
        .iter()
        .map(|points| {
            CubicBez::new(
                vec3_to_kurbo(points[0]),
                vec3_to_kurbo(points[1]),
                vec3_to_kurbo(points[2]),
                vec3_to_kurbo(points[3]),
            )
        })
        .collect()
}

/// Convert from a [`CubicBezier`] to a kurbo [`BezPath`].
pub fn bezier_to_kurbo_bez_path(bezier: &CubicBezier<Vec3>) -> BezPath {
    let cubics: Vec<CubicBez> = bezier_to_kurbo(bezier);
    let segments = cubics
        .iter()
        .flat_map(|cubic| cubic.path_segments(1.0).collect::<Vec<PathSeg>>());
    BezPath::from_path_segments(segments)
}
