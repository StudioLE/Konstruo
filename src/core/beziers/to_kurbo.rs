use crate::beziers::cubic_bezier_spline::CubicBezierSpline;
use crate::beziers::CubicBezier;
use bevy::log::warn;
use bevy::math::Vec3;
use kurbo::{BezPath, CubicBez, PathSeg, Point, Shape};

const KURBO_EPSILON: f32 = 0.000_1;

impl CubicBezier {
    /// Convert to a kurbo [`CubicBez`].
    #[must_use]
    pub fn to_kurbo(&self) -> CubicBez {
        CubicBez::new(
            vec3_to_kurbo(self.start),
            vec3_to_kurbo(self.start_handle),
            vec3_to_kurbo(self.end_handle),
            vec3_to_kurbo(self.end),
        )
    }
}

impl CubicBezierSpline {
    /// Convert to a collection of kurbo [`CubicBez`].
    pub(super) fn to_kurbo(&self) -> Vec<CubicBez> {
        self.curves
            .iter()
            .map(super::cubic_bezier::CubicBezier::to_kurbo)
            .collect()
    }

    /// Convert to a kurbo [`BezPath`].
    #[must_use]
    pub fn to_kurbo_bez_path(&self) -> BezPath {
        let segments = self
            .to_kurbo()
            .into_iter()
            .flat_map(|bezier| bezier.path_segments(1.0).collect::<Vec<PathSeg>>());
        BezPath::from_path_segments(segments)
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
