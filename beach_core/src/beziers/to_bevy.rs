use crate::beziers::cubic_bezier_spline::CubicBezierSpline;
use bevy::prelude::CubicBezier as BevyCubicBezier;
use bevy::prelude::*;

impl CubicBezierSpline {
    pub fn to_bevy(&self) -> BevyCubicBezier<Vec3> {
        BevyCubicBezier::new(self.get_controls())
    }
}
