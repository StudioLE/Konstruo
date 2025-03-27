use super::*;
use bevy::app::{App, Startup, Update};
use bevy::prelude::*;

pub struct PathPlugin;

impl Plugin for PathPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ControlMoved>()
            .add_event::<CurveAdded>()
            .add_event::<SplineChanged>()
            .add_systems(Startup, PathMaterials::startup_system)
            .add_systems(Startup, PathMeshes::startup_system)
            .add_systems(Update, PathControl::on_state_changed)
            .add_systems(Update, PathControlLine::on_state_changed)
            .add_systems(Update, PathSurface::on_state_changed)
            .add_systems(Update, Path::on_spline_changed)
            .add_systems(Update, PathControl::on_control_moved)
            .add_systems(Update, PathControl::on_curve_added)
            .add_systems(Update, PathControlLine::on_control_moved)
            .add_systems(Update, PathControlLine::on_curve_added)
            .add_systems(Update, PathSurface::on_spline_changed)
            .add_systems(Update, SplineChanged::on_control_moved)
            .add_systems(Update, SplineChanged::on_curve_added);
    }
}
