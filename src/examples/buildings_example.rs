use crate::architecture::*;
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
        commands.spawn(BuildingTemplates::_4b8p2s2g_perp());
    }
}
