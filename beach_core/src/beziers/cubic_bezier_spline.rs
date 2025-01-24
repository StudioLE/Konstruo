use crate::beziers::from_kurbo::vec3_from_kurbo;
use crate::beziers::internal_kurbo::bezpath_to_cubics;
use crate::beziers::CubicBezier;
use bevy::prelude::*;
use kurbo::offset::CubicOffset;
use kurbo::{fit_to_bezpath, flatten, stroke, Cap, CubicBez, Join, PathEl, Stroke, StrokeOpts};

/// A spline formed of one or more connected [`CubicBezier`].
#[derive(Clone, Debug, Default)]
pub struct CubicBezierSpline {
    pub curves: Vec<CubicBezier>,
}

impl CubicBezierSpline {
    /// Get the control points.
    pub fn get_controls(&self) -> Vec<Vec3> {
        self.curves.iter().flat_map(|x| x.get_controls()).collect()
    }

    /// Update the location of a control point at `index`.
    ///
    /// If the control point is:
    /// -  an anchor: the next or previous anchor is also moved.
    /// -  a handle: the opposing handle the movement is reflected but its distance from anchor unchanged.
    pub fn update_control(&mut self, index: usize, point: Vec3) {
        let curve = index / 4;
        let control = index % 4;
        if curve >= self.curves.len() {
            error!("Failed to update control point. Control point index is out of range: {index}");
            return;
        }
        let is_first = curve == 0;
        let is_last = curve == self.curves.len() - 1;
        match control {
            0 => {
                self.curves[curve].start = point;
                if !is_first {
                    self.curves[curve - 1].end = point;
                }
            }
            1 => {
                self.curves[curve].start_handle = point;
                if !is_first {
                    let anchor = self.curves[curve].start;
                    let direction = (anchor - point).normalize();
                    let distance = self.curves[curve - 1].end_handle.distance(anchor);
                    self.curves[curve - 1].end_handle = anchor + direction * distance;
                }
            }
            2 => {
                self.curves[curve].end_handle = point;
                if !is_last {
                    let anchor = self.curves[curve].end;
                    let direction = (anchor - point).normalize();
                    let distance = self.curves[curve + 1].start_handle.distance(anchor);
                    self.curves[curve + 1].start_handle = anchor + direction * distance;
                }
            }
            3 => {
                self.curves[curve].end = point;
                if !is_last {
                    self.curves[curve + 1].start = point;
                }
            }
            _ => panic!("Failed to move control point. Id of control is out of range"),
        };
    }

    /// Flatten a [`CubicBezier`] into a polyline.
    /// - <https://raphlinus.github.io/graphics/curves/2019/12/23/flatten-quadbez.html>
    pub fn flatten(&self, tolerance: f32) -> Vec<Vec3> {
        let path = self.to_kurbo_bez_path();
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

    /// Offset a bezier curve by a given distance.    ///
    /// - <https://raphlinus.github.io/curves/2022/09/09/parallel-beziers.html>
    pub fn offset(&self, distance: f32, accuracy: f32) -> CubicBezierSpline {
        let kurbo_bezier = self.to_kurbo();
        let segments: Vec<CubicBez> = kurbo_bezier
            .iter()
            .flat_map(|&segment| {
                let offset = CubicOffset::new(segment, f64::from(distance));
                let path = fit_to_bezpath(&offset, f64::from(accuracy));
                bezpath_to_cubics(path)
            })
            .collect();
        CubicBezierSpline::from_kurbo(segments)
    }

    pub fn stroke(&self, distance: f32, tolerance: f32) -> Self {
        let path = self.to_kurbo_bez_path();
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
        let result = stroke(path, &style, &options, f64::from(tolerance));
        let segments = bezpath_to_cubics(result);
        CubicBezierSpline::from_kurbo(segments)
    }
}
