use super::*;
use bevy::app::{App, Startup, Update};
use bevy::prelude::*;

pub struct WaysPlugin;

impl Plugin for WaysPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ControlMoved>()
            .add_event::<CurveAdded>()
            .add_event::<SplineChanged>()
            .add_systems(Startup, WayMaterials::startup_system)
            .add_systems(Startup, WayMeshes::startup_system)
            .add_systems(Update, WayControl::on_state_changed)
            .add_systems(Update, WayControlLine::on_state_changed)
            .add_systems(Update, WaySurface::on_state_changed)
            .add_systems(Update, Way::on_spline_changed)
            .add_systems(Update, WayControl::on_control_moved)
            .add_systems(Update, WayControl::on_curve_added)
            .add_systems(Update, WayControlLine::on_control_moved)
            .add_systems(Update, WayControlLine::on_curve_added)
            .add_systems(Update, WaySurface::on_spline_changed)
            .add_systems(Update, SplineChanged::on_control_moved)
            .add_systems(Update, SplineChanged::on_curve_added);
    }
}
