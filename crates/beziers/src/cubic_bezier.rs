use crate::*;
use bevy::prelude::*;
use konstruo_core::Vec3Extensions;
use kurbo::offset::CubicOffset;
use kurbo::{
    fit_to_bezpath, ParamCurve, ParamCurveArclen, ParamCurveCurvature, ParamCurveDeriv,
    ParamCurveExtrema, ParamCurveNearest,
};
use ControlType::*;
use CubicBezierError::*;

/// A single cubic bezier curve of four control points.
#[derive(Clone, Debug, Default)]
pub struct CubicBezier {
    pub(super) start: Vec3,
    pub(super) start_handle: Vec3,
    pub(super) end_handle: Vec3,
    pub(super) end: Vec3,
}

#[derive(Debug)]
pub enum CubicBezierError {
    TooClose(ControlType, ControlType),
}

impl CubicBezier {
    /// Create a new [`CubicBezier`].
    pub fn new(
        start: Vec3,
        start_handle: Vec3,
        end_handle: Vec3,
        end: Vec3,
    ) -> Result<Self, CubicBezierError> {
        if start.is_almost_equal_to(start_handle) {
            return Err(TooClose(Start, StartHandle));
        }
        if start.is_almost_equal_to(end) {
            return Err(TooClose(Start, End));
        }
        if end_handle.is_almost_equal_to(end) {
            return Err(TooClose(End, EndHandle));
        }
        Ok(Self {
            start,
            start_handle,
            end_handle,
            end,
        })
    }

    /// Get a control.
    #[must_use]
    pub fn get_control(&self, control_type: ControlType) -> Vec3 {
        match control_type {
            Start => self.start,
            StartHandle => self.start_handle,
            EndHandle => self.end_handle,
            End => self.end,
        }
    }
    /// Get the four controls.
    #[must_use]
    pub fn get_controls(&self) -> [Vec3; 4] {
        [self.start, self.start_handle, self.end_handle, self.end]
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
        let length = kurbo.arclen(accuracy.into());
        length.to_f32().expect("should not exceed f32 range")
    }

    /// The arc length of the curve.
    ///
    /// Solve for the parameter that has the given arc length from the start.
    /// This implementation uses the IPT method, as provided by `common::solve_itp`.
    /// This is as robust as bisection but typically converges faster. In addition,
    /// the method takes care to compute arc lengths of increasingly smaller segments of the curve,
    /// as that is likely faster than repeatedly computing the arc length of the segment starting
    /// at t=0.
    #[must_use]
    pub fn get_param_at_length(&self, length: f32, accuracy: f32) -> f32 {
        // TODO: This should return None if length < 0 or greater than curve length
        let length = self.to_kurbo().inv_arclen(length.into(), accuracy.into());
        length.to_f32().expect("should not exceed f32 range")
    }

    /// Get the param nearest to the vector.
    #[must_use]
    pub fn get_param_nearest_to(&self, vector: Vec3, accuracy: f32) -> f32 {
        let point = vector.to_kurbo();
        let nearest = self.to_kurbo().nearest(point, accuracy.into());
        nearest.t.to_f32().expect("should not exceed f32 range")
    }

    /// Compute the signed curvature at parameter.
    #[must_use]
    pub fn get_curvature_at_param(&self, param: f32) -> f32 {
        let curvature = self.to_kurbo().curvature(param.into());
        curvature.to_f32().expect("should not exceed f32 range")
    }

    /// Get a point at param.
    #[must_use]
    pub fn get_point_at_param(&self, param: f32) -> Vec3 {
        let point = self.to_kurbo().eval(param.into());
        point.to_vec3().expect("should not exceed f32 range")
    }

    /// Get the tangent at param.
    #[must_use]
    pub fn get_tangent_at_param(&self, param: f32) -> Vec3 {
        let quad_bez = self.to_kurbo().deriv();
        let point = quad_bez.eval(param.into());
        point
            .to_vec3()
            .expect("should not exceed f32 range")
            .normalize()
    }

    /// Compute the extrema of the curve.
    /// Only extrema within the interior of the curve count. At most four extrema can be reported, which is sufficient for cubic Béziers.
    /// The extrema should be reported in increasing parameter order.
    #[must_use]
    pub fn get_extrema(&self) -> Vec<f32> {
        self.to_kurbo()
            .extrema()
            .iter()
            .map(|&value| value.to_f32().expect("should not exceed f32 range"))
            .collect()
    }

    /// Get the quadratic bezier derivative.
    #[must_use]
    pub fn get_derivative(&self) -> [Vec3; 3] {
        let quad_bez = self.to_kurbo().deriv();
        [
            quad_bez.p0.to_vec3().expect("should not exceed f32 range"),
            quad_bez.p1.to_vec3().expect("should not exceed f32 range"),
            quad_bez.p2.to_vec3().expect("should not exceed f32 range"),
        ]
    }

    /// Reverse the direction of the curve.
    pub fn reverse(&mut self) {
        let start = self.start;
        let start_handle = self.start_handle;
        self.start = self.end;
        self.start_handle = self.end_handle;
        self.end_handle = start_handle;
        self.end = start;
    }

    /// Split the bezier at parameter with De Casteljau's algorithm.
    /// - <https://en.wikipedia.org/wiki/De_Casteljau%27s_algorithm>
    pub fn split_at_param(self, param: f32) -> Result<[CubicBezier; 2], CubicBezierError> {
        let start_handle_0 = self.start.lerp(self.start_handle, param);
        let between_handles = self.start_handle.lerp(self.end_handle, param);
        let end_handle_1 = self.end_handle.lerp(self.end, param);
        let end_handle_0 = start_handle_0.lerp(between_handles, param);
        let start_handle_1 = between_handles.lerp(end_handle_1, param);
        let point_at_param = end_handle_0.lerp(start_handle_1, param);
        Ok([
            CubicBezier::new(self.start, start_handle_0, end_handle_0, point_at_param)?,
            CubicBezier::new(point_at_param, start_handle_1, end_handle_1, self.end)?,
        ])
    }

    /// Offset a bezier curve by a given distance.
    /// - <https://raphlinus.github.io/curves/2022/09/09/parallel-beziers.html>
    pub fn offset(
        &self,
        distance: f32,
        accuracy: f32,
    ) -> Result<Vec<CubicBezier>, F32ConversionError> {
        let bez = self.to_kurbo();
        let offset = CubicOffset::new(bez, f64::from(distance));
        let path = fit_to_bezpath(&offset, f64::from(accuracy));
        let mut curves = Vec::new();
        for seg in path.segments() {
            curves.push(CubicBezier::from_kurbo(&seg.to_cubic())?)
        }
        Ok(curves)
    }
}
