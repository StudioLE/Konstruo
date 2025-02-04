use crate::beziers::{CubicBezier, CubicBezierSpline};
use bevy::math::Vec3;
use kurbo::{CubicBez, Point};
use F32ConversionError::*;

/// Convert from a Kurbo [`Point`] to a [`Vec3`].
pub fn vec3_from_kurbo(point: Point) -> Result<Vec3, F32ConversionError> {
    let x = f32_from_f64(point.x)?;
    let y = f32_from_f64(point.y)?;
    Ok(Vec3::new(x, y, 0.0))
}

#[derive(Clone, Debug)]
pub enum F32ConversionError {
    TooLarge(f64),
    TooSmall(f64),
}

/// Convert from a [`f64`] to a [`f32`].
///
/// Returns an [`F32ConversionError`] if the value is outside the range of an [`f32`].
/// Therefore [`cast_possible_truncation`](https://rust-lang.github.io/rust-clippy/master/index.html#cast_possible_truncation) are avoided.
///
/// Ideally this would be handled by `f32::try_from(f64)` but that doesn't seem to exist.
#[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
pub fn f32_from_f64(value: f64) -> Result<f32, F32ConversionError> {
    let max = f64::from(f32::MAX);
    let min = f64::from(f32::MIN);
    if value > max {
        Err(TooLarge(value))
    } else if value < min {
        Err(TooSmall(value))
    } else {
        Ok(value as f32)
    }
}

impl CubicBezier {
    /// Convert from a collection of Kurbo [`CubicBez`] to a [`CubicBezier`].
    pub fn from_kurbo(bezier: &CubicBez) -> Result<CubicBezier, F32ConversionError> {
        Ok(CubicBezier {
            start: vec3_from_kurbo(bezier.p0)?,
            start_handle: vec3_from_kurbo(bezier.p1)?,
            end_handle: vec3_from_kurbo(bezier.p2)?,
            end: vec3_from_kurbo(bezier.p3)?,
        })
    }
}

impl CubicBezierSpline {
    /// Convert from a collection of Kurbo [`CubicBez`] to a [`CubicBezierSpline`].
    pub fn from_kurbo(segments: Vec<CubicBez>) -> Result<CubicBezierSpline, F32ConversionError> {
        let curves = segments
            .iter()
            .map(CubicBezier::from_kurbo)
            .collect::<Result<_, _>>()?;
        Ok(CubicBezierSpline { curves })
    }
}
