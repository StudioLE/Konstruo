use crate::beziers::from_kurbo::{f32_from_f64, vec3_from_kurbo};
use crate::beziers::internal_kurbo::bezpath_to_cubics;
use crate::beziers::{ControlType, CubicBezier};
use bevy::prelude::*;
use kurbo::offset::CubicOffset;
use kurbo::{
    fit_to_bezpath, flatten, stroke, Cap, CubicBez, Join, ParamCurveArclen, PathEl, Stroke,
    StrokeOpts,
};
use ControlType::*;

/// A spline formed of one or more connected [`CubicBezier`].
#[derive(Clone, Debug, Default)]
pub struct CubicBezierSpline {
    pub curves: Vec<CubicBezier>,
}

impl CubicBezierSpline {
    /// Get a control.
    #[must_use]
    pub fn get_control(&self, control_type: ControlType, curve: usize) -> Option<Vec3> {
        let control = self.curves.get(curve)?.get_control(control_type);
        Some(control)
    }

    /// Get the control points.
    #[must_use]
    pub fn get_controls(&self) -> Vec<Vec3> {
        self.curves
            .iter()
            .flat_map(CubicBezier::get_controls)
            .collect()
    }

    /// The arc length of the curve.
    ///
    /// The result is accurate to the given accuracy
    /// (subject to roundoff errors for ridiculously low values).
    /// Compute time may vary with accuracy, if the curve needs to be subdivided.
    ///
    /// This is an adaptive subdivision approach using Legendre-Gauss quadrature in the base case,
    /// and an error estimate to decide when to subdivide.
    #[must_use]
    pub fn get_length(&self, accuracy: f32) -> f32 {
        let kurbo = self.to_kurbo();
        let length = kurbo
            .iter()
            .map(|&bezier| bezier.arclen(accuracy.into()))
            .sum();
        f32_from_f64(length).expect("should not exceed f32 range")
    }

    /// Get the param at the length along the curve.
    ///
    /// Returns `None` if the length exceeds the length of the curve.
    ///
    /// Solve for the parameter that has the given arc length from the start.
    /// This implementation uses the IPT method, as provided by `common::solve_itp`.
    /// This is as robust as bisection but typically converges faster. In addition,
    /// the method takes care to compute arc lengths of increasingly smaller segments of the curve,
    /// as that is likely faster than repeatedly computing the arc length of the segment starting
    /// at t=0.
    #[must_use]
    #[allow(clippy::as_conversions, clippy::cast_precision_loss, clippy::panic)]
    pub fn get_param_at_length(&self, length: f32, accuracy: f32) -> Option<f32> {
        let mut preceding_length = 0.0;
        for (index, curve) in self.curves.iter().enumerate() {
            let curve_length = curve.get_length(accuracy);
            if preceding_length + curve_length > length {
                let curve_param = curve.get_param_at_length(length - preceding_length, accuracy);
                let param = (curve_param + index as f32) / self.curves.len() as f32;
                return Some(param);
            }
            preceding_length += curve_length;
        }
        None
    }

    /// Get the param nearest to the vector.
    #[must_use]
    #[allow(clippy::as_conversions, clippy::cast_precision_loss)]
    pub fn get_param_nearest_to(&self, vector: Vec3, accuracy: f32) -> f32 {
        let (index, param, _distance) = self
            .curves
            .iter()
            .enumerate()
            .map(|(index, curve)| {
                let param = curve.get_param_nearest_to(vector, accuracy);
                let point = curve.get_point_at_param(param);
                let distance = (point - vector).length();
                (index, param, distance)
            })
            .min_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).expect("should be able to compare"))
            .expect("should not be empty");
        (param + index as f32) / self.curves.len() as f32
    }

    /// Get the curve at the param and recalculate the param so it's relative to the curve.
    #[must_use]
    #[allow(
        clippy::as_conversions,
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap,
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss
    )]
    fn get_curve_at_param(&self, param: f32) -> (&CubicBezier, f32) {
        let scaled_param = param * self.curves.len() as f32;
        let index = scaled_param.floor() as usize;
        let param = scaled_param - index as f32;
        let curve = self
            .curves
            .get(index)
            .expect("param should not be out of range");
        (curve, param)
    }

    /// Compute the signed curvature at parameter.
    #[must_use]
    pub fn get_curvature_at_param(&self, param: f32) -> f32 {
        let (curve, param) = self.get_curve_at_param(param);
        curve.get_curvature_at_param(param)
    }

    /// Get a point at param.
    #[must_use]
    pub fn get_point_at_param(&self, param: f32) -> Vec3 {
        let (curve, param) = self.get_curve_at_param(param);
        curve.get_point_at_param(param)
    }

    /// Get the tangent at param.
    #[must_use]
    pub fn get_tangent_at_param(&self, param: f32) -> Vec3 {
        let (curve, param) = self.get_curve_at_param(param);
        curve.get_tangent_at_param(param)
    }

    /// Compute the extrema of the curve.
    /// Only extrema within the interior of the curve count. At most four extrema can be reported, which is sufficient for cubic Béziers.
    /// The extrema should be reported in increasing parameter order.
    #[must_use]
    pub fn get_extrema(&self) -> Vec<f32> {
        self.curves
            .iter()
            .flat_map(CubicBezier::get_extrema)
            .collect()
    }

    /// Update the location of a control point at `index`.
    ///
    /// If the control point is:
    /// -  an anchor: the next or previous anchor and handles are moved.
    /// -  a handle: the opposing handle is rotated but its distance from anchor unchanged.
    #[allow(clippy::indexing_slicing, clippy::integer_division)]
    pub fn update_control(&mut self, control_type: ControlType, curve: usize, point: Vec3) {
        if curve >= self.curves.len() {
            error!("Failed to update control point. Curve index is out of range: {curve}");
            return;
        }
        let is_first = curve == 0;
        let is_last = curve == self.curves.len() - 1;
        match control_type {
            Start => {
                let translation = point - self.curves[curve].start;
                self.curves[curve].start = point;
                self.curves[curve].start_handle += translation;
                if !is_first {
                    self.curves[curve - 1].end = point;
                    self.curves[curve - 1].end_handle += translation;
                }
            }
            StartHandle => {
                self.curves[curve].start_handle = point;
                if !is_first {
                    let anchor = self.curves[curve].start;
                    let direction = (anchor - point).normalize();
                    let distance = self.curves[curve - 1].end_handle.distance(anchor);
                    self.curves[curve - 1].end_handle = anchor + direction * distance;
                }
            }
            EndHandle => {
                self.curves[curve].end_handle = point;
                if !is_last {
                    let anchor = self.curves[curve].end;
                    let direction = (anchor - point).normalize();
                    let distance = self.curves[curve + 1].start_handle.distance(anchor);
                    self.curves[curve + 1].start_handle = anchor + direction * distance;
                }
            }
            End => {
                let translation = point - self.curves[curve].end;
                self.curves[curve].end = point;
                self.curves[curve].end_handle += translation;
                if !is_last {
                    self.curves[curve + 1].start = point;
                    self.curves[curve + 1].start_handle += translation;
                }
            }
        };
    }

    /// Flatten a [`CubicBezier`] into a polyline.
    /// - <https://raphlinus.github.io/graphics/curves/2019/12/23/flatten-quadbez.html>
    /// - TODO: Flatten may panic
    #[must_use]
    #[allow(clippy::panic)]
    pub fn flatten(&self, tolerance: f32) -> Vec<Vec3> {
        let path = self.to_kurbo_bez_path();
        let mut points = Vec::new();
        flatten(path, tolerance.into(), &mut |segment| match segment {
            PathEl::MoveTo(point) | PathEl::LineTo(point) => {
                let point = vec3_from_kurbo(point).expect("should not exceed f32 range");
                points.push(point);
            }
            PathEl::QuadTo(_, _) => panic!("Failed to flatten CubicBezier. Unexpected QuadTo"),
            PathEl::CurveTo(_, _, _) => panic!("Failed to flatten CubicBezier. Unexpected CurveTo"),
            PathEl::ClosePath => panic!("Failed to flatten CubicBezier. Unexpected ClosePath"),
        });
        points
    }

    /// Offset a bezier curve by a given distance.    ///
    /// - <https://raphlinus.github.io/curves/2022/09/09/parallel-beziers.html>
    #[must_use]
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
        CubicBezierSpline::from_kurbo(segments).expect("should not exceed f32 range")
    }

    #[must_use]
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
        CubicBezierSpline::from_kurbo(segments).expect("should not exceed f32 range")
    }
}
