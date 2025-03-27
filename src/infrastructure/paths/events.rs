use crate::beziers::CubicBezierSpline;
use bevy::prelude::*;

/// Collate [`CurveAdded`] and [`ControlMoved`].
#[derive(Debug, Event)]
pub(super) struct SplineChanged {
    pub path: Entity,
    pub spline: CubicBezierSpline,
}

/// A control of a spline has been moved.
#[derive(Debug, Event)]
pub struct ControlMoved {
    pub path: Entity,
    pub spline: CubicBezierSpline,
}

/// A curve has been added (or removed) to a spline.
#[derive(Debug, Event)]
pub struct CurveAdded {
    pub path: Entity,
    pub spline: CubicBezierSpline,
}

impl SplineChanged {
    /// Trigger a [`SplineChanged`] on [`CurveAdded`].
    pub(super) fn on_curve_added(
        mut events: EventReader<CurveAdded>,
        mut writer: EventWriter<SplineChanged>,
    ) {
        for event in events.read() {
            writer.send(SplineChanged {
                path: event.path,
                spline: event.spline.clone(),
            });
        }
    }

    /// Trigger a [`SplineChanged`] on [`ControlMoved`].
    pub(super) fn on_control_moved(
        mut events: EventReader<ControlMoved>,
        mut writer: EventWriter<SplineChanged>,
    ) {
        for event in events.read() {
            writer.send(SplineChanged {
                path: event.path,
                spline: event.spline.clone(),
            });
        }
    }
}
