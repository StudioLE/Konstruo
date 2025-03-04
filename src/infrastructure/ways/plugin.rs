use super::*;
use bevy::app::{App, Startup, Update};
use bevy::prelude::*;

pub struct WaysPlugin;

impl Plugin for WaysPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SplineChangedEvent>()
            .add_event::<StateChangedEvent>()
            .add_systems(Startup, WayMaterials::startup_system)
            .add_systems(Startup, WayMeshes::startup_system)
            .add_systems(Update, Way::added_system)
            .add_systems(Update, WaySurface::on_state_changed)
            .add_systems(Update, Way::on_spline_changed)
            .add_systems(Update, WayControl::on_spline_changed)
            .add_systems(Update, WayControlLine::on_spline_changed)
            .add_systems(Update, WaySurface::on_spline_changed);
    }
}
