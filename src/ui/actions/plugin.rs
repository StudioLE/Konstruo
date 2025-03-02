use super::*;
use crate::ui::PrimaryCamera;
use bevy::prelude::*;

/// Plugin to enable Floating Action Buttons (FAB).
pub struct FloatingActionPlugin;

impl Plugin for FloatingActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, Self::startup_system);
    }
}

impl FloatingActionPlugin {
    fn startup_system(mut commands: Commands, query: Query<Entity, With<PrimaryCamera>>) {
        let Ok(camera) = query.get_single() else {
            warn!("Failed to get PrimaryCamera");
            return;
        };
        let bundle = (TargetCamera(camera), FloatingActionContainer);
        let container = commands.spawn(bundle).id();
        for _i in 0..4 {
            commands.spawn(FloatingActionButton).set_parent(container);
        }
    }
}
