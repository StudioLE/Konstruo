use super::*;
use bevy::prelude::*;

/// Plugin to enable Floating Action Buttons (FAB).
pub struct FloatingActionPlugin;

impl Plugin for FloatingActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, Self::startup_system);
    }
}

impl FloatingActionPlugin {
    fn startup_system(mut commands: Commands) {
        let container = commands.spawn(FloatingActionContainer).id();
        for _i in 0..4 {
            commands.spawn(FloatingActionButton).set_parent(container);
        }
    }
}
