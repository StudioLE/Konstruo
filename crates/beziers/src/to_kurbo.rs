use crate::CubicBezier;
use crate::*;
use bevy::prelude::*;
use kurbo::{BezPath, CubicBez, PathEl, Point};
use ControlType::*;

const KURBO_EPSILON: f32 = 0.000_1;

impl CubicBezier {
    /// Convert to a kurbo [`CubicBez`].
    #[must_use]
    pub fn to_kurbo(&self) -> CubicBez {
        CubicBez::new(
            vec3_to_kurbo(self.get_control(Start)),
            vec3_to_kurbo(self.get_control(StartHandle)),
            vec3_to_kurbo(self.get_control(EndHandle)),
            vec3_to_kurbo(self.get_control(End)),
        )
    }

    /// Convert to a kurbo [`PathEl`].
    ///
    /// CAUTION: This excludes the start therefore that will need to be manually specified.
    #[must_use]
    fn to_kurbo_path(&self) -> PathEl {
        PathEl::CurveTo(
            vec3_to_kurbo(self.get_control(StartHandle)),
            vec3_to_kurbo(self.get_control(EndHandle)),
            vec3_to_kurbo(self.get_control(End)),
        )
    }
}

impl CubicBezierSpline {
    /// Convert to a kurbo [`BezPath`].
    pub(super) fn to_kurbo(&self) -> BezPath {
        let start = vec3_to_kurbo(self.get_start());
        let mut path = BezPath::new();
        path.push(PathEl::MoveTo(start));
        for curve in self.get_curves() {
            path.push(curve.to_kurbo_path());
        }
        path
    }
}

/// Convert from a [`Vec3`] to a kurbo [`Point`].
pub fn vec3_to_kurbo(vector: Vec3) -> Point {
    if vector.z.abs() > KURBO_EPSILON {
        warn!(
            "Kurbo only supports 2D coordinates. Ignoring Z value: {}",
            vector.z
        );
    }
    Point::new(f64::from(vector.x), f64::from(vector.y))
}
