use crate::CubicBezier;
use crate::*;
use bevy::prelude::*;
use kurbo::{BezPath, CubicBez, PathEl, Point};
use ControlType::*;

const KURBO_EPSILON: f32 = 0.000_1;

pub trait Vec3ToPoint {
    fn to_kurbo(&self) -> Point;
}

impl CubicBezier {
    /// Convert to a kurbo [`CubicBez`].
    #[must_use]
    pub fn to_kurbo(&self) -> CubicBez {
        CubicBez::new(
            self.get_control(Start).to_kurbo(),
            self.get_control(StartHandle).to_kurbo(),
            self.get_control(EndHandle).to_kurbo(),
            self.get_control(End).to_kurbo(),
        )
    }

    /// Convert to a kurbo [`PathEl`].
    ///
    /// CAUTION: This excludes the start therefore that will need to be manually specified.
    #[must_use]
    fn to_kurbo_path(&self) -> PathEl {
        PathEl::CurveTo(
            self.get_control(StartHandle).to_kurbo(),
            self.get_control(EndHandle).to_kurbo(),
            self.get_control(End).to_kurbo(),
        )
    }
}

impl CubicBezierSpline {
    /// Convert to a kurbo [`BezPath`].
    pub(super) fn to_kurbo(&self) -> BezPath {
        let start = self.get_start().to_kurbo();
        let mut path = BezPath::new();
        path.push(PathEl::MoveTo(start));
        for curve in self.get_curves() {
            path.push(curve.to_kurbo_path());
        }
        path
    }
}

impl Vec3ToPoint for Vec3 {
    /// Convert from a [`Vec3`] to a kurbo [`Point`].
    fn to_kurbo(&self) -> Point {
        if self.z.abs() > KURBO_EPSILON {
            warn!(
                "Kurbo only supports 2D coordinates. Ignoring Z value: {}",
                self.z
            );
        }
        Point::new(f64::from(self.x), f64::from(self.y))
    }
}
