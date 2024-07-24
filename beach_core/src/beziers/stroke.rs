use bevy::math::Vec3;
use bevy::prelude::CubicBezier;
use kurbo::{stroke, Cap, Join, Stroke, StrokeOpts};

use crate::beziers::from_kurbo::bezier_from_kurbo;
use crate::beziers::internal_kurbo::bezpath_to_cubics;
use crate::beziers::to_kurbo::bezier_to_kurbo_bez_path;

pub fn stroke_bezier(
    bezier: &CubicBezier<Vec3>,
    distance: f32,
    tolerance: f32,
) -> CubicBezier<Vec3> {
    let source = bezier_to_kurbo_bez_path(bezier);
    // let style = Stroke::new(distance);
    let style = Stroke {
        width: f64::from(distance),
        join: Join::Miter,
        miter_limit: 0.1,
        start_cap: Cap::Butt,
        end_cap: Cap::Butt,
        ..Stroke::default()
    };
    let options = StrokeOpts::default();
    // let options = StrokeOpts::default().opt_level(StrokeOptLevel::Optimized);
    let result = stroke(source, &style, &options, f64::from(tolerance));
    let cubic = bezpath_to_cubics(result);
    bezier_from_kurbo(cubic)
}
