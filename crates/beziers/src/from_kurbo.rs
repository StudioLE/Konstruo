use crate::*;
use bevy::prelude::Vec3;
use kurbo::{BezPath, CubicBez, Point};
use CubicBezierSplineError::Conversion;
use F32ConversionError::*;

pub trait F64Extensions {
    fn to_f32(self) -> Result<f32, F32ConversionError>;
}

pub trait KurboPointExtensions {
    fn to_vec3(&self) -> Result<Vec3, F32ConversionError>;
}

#[derive(Clone, Debug)]
pub enum F32ConversionError {
    TooLarge(f64),
    TooSmall(f64),
}

impl F64Extensions for f64 {
    /// Convert from a [`f64`] to a [`f32`].
    ///
    /// Returns an [`F32ConversionError`] if the value is outside the range of an [`f32`].
    /// Therefore [`cast_possible_truncation`](https://rust-lang.github.io/rust-clippy/master/index.html#cast_possible_truncation) are avoided.
    #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
    fn to_f32(self) -> Result<f32, F32ConversionError> {
        let max = f64::from(f32::MAX);
        let min = f64::from(f32::MIN);
        if self > max {
            Err(TooLarge(self))
        } else if self < min {
            Err(TooSmall(self))
        } else {
            Ok(self as f32)
        }
    }
}

impl KurboPointExtensions for Point {
    /// Convert from a Kurbo [`Point`] to a [`Vec3`].
    fn to_vec3(&self) -> Result<Vec3, F32ConversionError> {
        let x = self.x.to_f32()?;
        let y = self.y.to_f32()?;
        Ok(Vec3::new(x, y, 0.0))
    }
}

impl CubicBezier {
    /// Convert from a collection of Kurbo [`CubicBez`] to a [`CubicBezier`].
    pub fn from_kurbo(bezier: &CubicBez) -> Result<CubicBezier, F32ConversionError> {
        Ok(CubicBezier {
            start: bezier.p0.to_vec3()?,
            start_handle: bezier.p1.to_vec3()?,
            end_handle: bezier.p2.to_vec3()?,
            end: bezier.p3.to_vec3()?,
        })
    }
}

impl CubicBezierSpline {
    /// Convert from a Kurbo [`BezPath`] to a [`CubicBezierSpline`].
    pub fn from_kurbo(path: BezPath) -> Result<CubicBezierSpline, CubicBezierSplineError> {
        let mut curves = Vec::new();
        for seg in path.segments() {
            let curve = CubicBezier::from_kurbo(&seg.to_cubic()).map_err(Conversion)?;
            curves.push(curve);
        }
        CubicBezierSpline::new(curves)
    }
}
