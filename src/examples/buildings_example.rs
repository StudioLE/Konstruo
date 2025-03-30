use crate::architecture::*;
use bevy::prelude::*;

pub struct BuildingsExample;

impl Plugin for BuildingsExample {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, Self::startup_system);
    }
}

impl BuildingsExample {
    fn startup_system(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        building_meshes: Res<BuildingMeshes>,
        materials: Res<BuildingMaterials>,
    ) {
        let building = BuildingTemplates::_4b8p2s2g();
        building.spawn(&mut commands, &mut meshes, &building_meshes, &materials);
    }
}
