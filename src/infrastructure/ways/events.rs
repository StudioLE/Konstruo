use crate::beziers::CubicBezierSpline;
use bevy::prelude::*;

#[derive(Debug, Event)]
pub struct SplineChanged {
    pub way: Entity,
    pub spline: CubicBezierSpline,
}
