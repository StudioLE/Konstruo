use crate::ways::{Way, WayMaterials, WayMeshes};
use bevy::app::{App, Startup, Update};
use bevy::prelude::*;

pub struct WaysPlugin;

impl Plugin for WaysPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, WayMaterials::startup_system)
            .add_systems(Startup, WayMeshes::startup_system)
            .add_systems(Update, Way::added_system);
    }
}
