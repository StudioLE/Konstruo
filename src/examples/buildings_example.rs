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
        meshes: Res<BuildingMeshes>,
        materials: Res<BuildingMaterials>,
    ) {
        let stacks = vec![
            BuildingModuleStack {
                definition: BuildingModule {
                    width: 4.8,
                    length: 9.0,
                    front_offset: 0.0,
                    back_offset: 0.0,
                    ..default()
                },
                levels: 2,
                level_height: 2.4,
                roof_style: Some(RoofStyle::PitchLeftToRight),
            },
            BuildingModuleStack {
                definition: BuildingModule {
                    width: 6.0,
                    length: 5.4,
                    front_offset: 3.0,
                    back_offset: 0.6,
                    ..default()
                },
                levels: 1,
                level_height: 2.4,
                roof_style: Some(RoofStyle::PitchFrontToBack),
            },
        ];

        ModularBuildingFactory::spawn(&mut commands, &meshes, &materials, stacks)
            .expect("spawn should not fail");
    }
}
