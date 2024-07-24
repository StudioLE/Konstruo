use bevy::math::Vec3;
use bevy::prelude::CubicBezier;
use kurbo::offset::CubicOffset;
use kurbo::{fit_to_bezpath, CubicBez};

use crate::beziers::from_kurbo::bezier_from_kurbo;
use crate::beziers::internal_kurbo::bezpath_to_cubics;
use crate::beziers::to_kurbo::bezier_to_kurbo;

/// Offset a bezier curve by a given distance.
///
/// See: <https://raphlinus.github.io/curves/2022/09/09/parallel-beziers.html>
pub fn offset_bezier(
    bezier: &CubicBezier<Vec3>,
    distance: f32,
    accuracy: f32,
) -> CubicBezier<Vec3> {
    let kurbo_bezier = bezier_to_kurbo(bezier);
    let segments: Vec<CubicBez> = kurbo_bezier
        .iter()
        .flat_map(|&segment| {
            let offset = CubicOffset::new(segment, f64::from(distance));
            let path = fit_to_bezpath(&offset, f64::from(accuracy));
            bezpath_to_cubics(path)
        })
        .collect();
    bezier_from_kurbo(segments)
}
