use bevy::prelude::*;
use konstruo_beziers::CubicBezierSpline;

/// Collate [`CurveAdded`] and [`ControlMoved`].
#[derive(Debug, Message)]
pub(super) struct SplineChanged {
    pub path: Entity,
    pub spline: CubicBezierSpline,
}

/// A control of a spline has been moved.
#[derive(Debug, Message)]
pub struct ControlMoved {
    pub path: Entity,
    pub spline: CubicBezierSpline,
}

/// A curve has been added (or removed) to a spline.
#[derive(Debug, Message)]
pub struct CurveAdded {
    pub path: Entity,
    pub spline: CubicBezierSpline,
}

impl SplineChanged {
    /// Trigger a [`SplineChanged`] on [`CurveAdded`].
    pub(super) fn on_curve_added(
        mut events: MessageReader<CurveAdded>,
        mut writer: MessageWriter<SplineChanged>,
    ) {
        for event in events.read() {
            writer.write(SplineChanged {
                path: event.path,
                spline: event.spline.clone(),
            });
        }
    }

    /// Trigger a [`SplineChanged`] on [`ControlMoved`].
    pub(super) fn on_control_moved(
        mut events: MessageReader<ControlMoved>,
        mut writer: MessageWriter<SplineChanged>,
    ) {
        for event in events.read() {
            writer.write(SplineChanged {
                path: event.path,
                spline: event.spline.clone(),
            });
        }
    }
}
