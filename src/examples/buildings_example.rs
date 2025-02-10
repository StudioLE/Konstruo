use crate::architecture::*;
use crate::geometry::Vec6;
use bevy::prelude::*;

pub struct BuildingsExample;

impl Plugin for BuildingsExample {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, Self::startup_system)
            .add_systems(Update, ModularBuildingFactory::added_system);
    }
}

impl BuildingsExample {
    fn startup_system(mut commands: Commands) {
        let stacks = vec![
            BuildingModuleStack {
                definition: BuildingModule {
                    width: 4.8,
                    length: 9.0,
                    ..default()
                },
                levels: 2,
                level_height: 2.4,
                roof_height: 1.8,
                roof_style: Some(RoofStyle::PitchLeftToRight),
            },
            BuildingModuleStack {
                definition: BuildingModule {
                    width: 6.0,
                    length: 5.4,
                    margin: Some(Vec6::default().with_back(0.6)),
                    ..default()
                },
                levels: 1,
                level_height: 2.4,
                roof_height: 1.8,
                roof_style: Some(RoofStyle::PitchFrontToBack),
            },
        ];
        let factory = ModularBuildingFactory { stacks };
        commands.spawn(factory);
    }
}
