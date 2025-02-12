use crate::architecture::{BuildingMaterials, BuildingMeshes, ModularBuildingFactory};
use bevy::app::{App, Startup};
use bevy::prelude::*;

pub struct BuildingsPlugin;

impl Plugin for BuildingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, BuildingMaterials::startup_system)
            .add_systems(Startup, BuildingMeshes::startup_system);
    }
}
