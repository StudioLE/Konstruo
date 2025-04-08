use crate::*;
use bevy::prelude::*;

pub struct BuildingsExample;

impl Plugin for BuildingsExample {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, Self::startup_system);
    }
}

impl BuildingsExample {
    fn startup_system(
        commands: Commands,
        meshes: ResMut<Assets<Mesh>>,
        building_meshes: Res<BuildingMeshes>,
        materials: Res<BuildingMaterials>,
    ) {
        let mut factory = ModularBuildingFactory {
            commands,
            meshes,
            building_meshes,
            materials,
        };
        factory.spawn(BuildingTemplates::_4b8p2s2g());
    }
}
