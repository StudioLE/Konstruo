use crate::beziers::from_kurbo::vec3_from_kurbo;
use crate::beziers::to_kurbo::vec3_to_kurbo;
use bevy::prelude::*;
use kurbo::{
    ParamCurve, ParamCurveArclen, ParamCurveCurvature, ParamCurveDeriv, ParamCurveExtrema,
    ParamCurveNearest,
};

/// A single cubic bezier curve of four control points.
#[derive(Clone, Debug, Default)]
pub struct CubicBezier {
    pub start: Vec3,
    pub start_handle: Vec3,
    pub end_handle: Vec3,
    pub end: Vec3,
}

impl CubicBezier {
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
    pub fn get_length(&self, accuracy: f64) -> f64 {
        let kurbo = self.to_kurbo();
        kurbo.arclen(accuracy)
    }

    /// Get a point at param.
    #[must_use]
    pub fn get_point_at_param(&self, param: f64) -> Vec3 {
        let point = self.to_kurbo().eval(param);
        vec3_from_kurbo(point)
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
    pub fn get_param_at_length(&self, length: f64, accuracy: f64) -> f64 {
        self.to_kurbo().inv_arclen(length, accuracy)
    }

    /// Compute the signed curvature at parameter.
    #[must_use]
    pub fn get_curvature_at_param(&self, param: f64) -> f64 {
        self.to_kurbo().curvature(param)
    }

    /// Compute the signed curvature at parameter.
    #[must_use]
    pub fn get_extrema(&self) -> Vec<f64> {
        self.to_kurbo().extrema().to_vec()
    }

    /// Get the quadratic bezier derivative.
    #[must_use]
    pub fn get_derivative(&self) -> [Vec3; 3] {
        let quad_bez = self.to_kurbo().deriv();
        [
            vec3_from_kurbo(quad_bez.p0),
            vec3_from_kurbo(quad_bez.p1),
            vec3_from_kurbo(quad_bez.p2),
        ]
    }

    /// Get the tangent at param.
    #[must_use]
    pub fn get_tangent_at_param(&self, param: f64) -> Vec3 {
        // TODO: Confirm if this is correct
        let quad_bez = self.to_kurbo().deriv();
        let point = quad_bez.eval(param);
        vec3_from_kurbo(point)
    }

    /// Get the param nearest to the vector.
    #[must_use]
    pub fn get_param_nearest_to(&self, vector: Vec3, accuracy: f64) -> f64 {
        let point = vec3_to_kurbo(vector);
        let nearest = self.to_kurbo().nearest(point, accuracy);
        nearest.t
    }
}
