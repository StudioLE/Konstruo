use crate::*;
use bevy::app::{App, Startup};
use bevy::prelude::*;

pub struct ModularBuildingsPlugin;

impl Plugin for ModularBuildingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, BuildingMaterials::startup_system)
            .add_systems(Startup, BuildingMeshes::startup_system);
    }
}
