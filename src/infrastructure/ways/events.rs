use crate::beziers::CubicBezierSpline;
use bevy::prelude::*;

#[derive(Debug, Event)]
pub struct SplineChangedEvent {
    pub way: Entity,
    pub spline: CubicBezierSpline,
}
