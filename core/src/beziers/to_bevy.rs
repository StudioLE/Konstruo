use crate::beziers::cubic_bezier_spline::CubicBezierSpline;
use bevy::prelude::CubicBezier as BevyCubicBezier;
use bevy::prelude::*;

impl CubicBezierSpline {
    #[must_use]
    pub fn to_bevy(&self) -> BevyCubicBezier<Vec3> {
        let controls: Vec<_> = self
            .curves
            .iter()
            .map(super::cubic_bezier::CubicBezier::get_controls)
            .collect();
        BevyCubicBezier::new(controls)
    }
}
