use crate::beziers::{CubicBezier, CubicBezierSpline};
use bevy::math::Vec3;
use kurbo::{CubicBez, Point};

/// Convert from a Kurbo [`Point`] to a [`Vec3`].
pub fn vec3_from_kurbo(point: Point) -> Vec3 {
    Vec3::new(point.x as f32, point.y as f32, 0.0)
}

impl CubicBezier {
    /// Convert from a collection of Kurbo [`CubicBez`] to a [`CubicBezier`].
    #[must_use]
    pub fn from_kurbo(bezier: &CubicBez) -> CubicBezier {
        CubicBezier {
            start: vec3_from_kurbo(bezier.p0),
            start_handle: vec3_from_kurbo(bezier.p1),
            end_handle: vec3_from_kurbo(bezier.p2),
            end: vec3_from_kurbo(bezier.p3),
        }
    }
}

impl CubicBezierSpline {
    /// Convert from a collection of Kurbo [`CubicBez`] to a [`CubicBezierSpline`].
    pub fn from_kurbo(segments: Vec<CubicBez>) -> CubicBezierSpline {
        let curves = segments.iter().map(CubicBezier::from_kurbo).collect();
        CubicBezierSpline { curves }
    }
}
