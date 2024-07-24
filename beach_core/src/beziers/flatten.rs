use bevy::prelude::*;
use kurbo::{flatten, PathEl};

use crate::beziers::from_kurbo::vec3_from_kurbo;
use crate::beziers::to_kurbo::bezier_to_kurbo_bez_path;

/// Flatten a [`CubicBezier`] into a polyline.
/// See: <https://raphlinus.github.io/graphics/curves/2019/12/23/flatten-quadbez.html>
pub fn flatten_bezier(bezier: &CubicBezier<Vec3>, tolerance: f32) -> Vec<Vec3> {
    let path = bezier_to_kurbo_bez_path(bezier);
    let mut points = Vec::new();
    flatten(path, f64::from(tolerance), &mut |segment| match segment {
        PathEl::LineTo(point) => points.push(vec3_from_kurbo(point)),
        PathEl::MoveTo(point) => points.push(vec3_from_kurbo(point)),
        PathEl::QuadTo(_, _) => panic!("Failed to flatten CubicBezier. Unexpected QuadTo"),
        PathEl::CurveTo(_, _, _) => panic!("Failed to flatten CubicBezier. Unexpected CurveTo"),
        PathEl::ClosePath => panic!("Failed to flatten CubicBezier. Unexpected ClosePath"),
    });
    points
}
