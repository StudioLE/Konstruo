use crate::*;
use bevy::prelude::{CubicBezier as BevyCubicBezier, Vec3};

impl CubicBezierSpline {
    #[must_use]
    pub fn to_bevy(&self) -> BevyCubicBezier<Vec3> {
        let controls: Vec<_> = self
            .get_curves()
            .iter()
            .map(super::cubic_bezier::CubicBezier::get_controls)
            .collect();
        BevyCubicBezier::new(controls)
    }
}
