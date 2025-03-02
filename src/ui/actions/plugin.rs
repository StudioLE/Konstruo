use super::*;
use crate::ui::FLOATING_ACTION_CAMERA_ORDER;
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
        let bundle = (
            Camera2d,
            Camera {
                order: FLOATING_ACTION_CAMERA_ORDER,
                ..default()
            },
        );
        commands.spawn(bundle);
        let container = commands.spawn(FloatingActionContainer).id();
        for _i in 0..4 {
            commands.spawn(FloatingActionButton).set_parent(container);
        }
    }
}
