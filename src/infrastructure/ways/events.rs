use crate::beziers::CubicBezierSpline;
use crate::ui::EntityState;
use bevy::prelude::*;

#[derive(Debug, Event)]
pub struct SplineChangedEvent {
    pub way: Entity,
    pub spline: CubicBezierSpline,
}

#[derive(Debug, Event)]
pub struct StateChangedEvent {
    pub way: Entity,
    pub state: EntityState,
}
