use super::*;
use crate::ui::{PrimaryCamera, BEZIER_ICON, CLOSE_ICON, EDIT_ICON, MORE_ICON};
use bevy::prelude::*;
use FloatingActionButtonSize::*;

/// Plugin to spawn a bottom app bar with floating action buttons.
/// - <https://m3.material.io/components/bottom-app-bar/overview>
/// - <https://m3.material.io/components/floating-action-button/overview>
pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, Self::startup_system);
    }
}

impl ActionsPlugin {
    fn startup_system(
        mut commands: Commands,
        assets: Res<AssetServer>,
        query: Query<Entity, With<PrimaryCamera>>,
    ) {
        let Ok(camera) = query.get_single() else {
            warn!("Failed to get PrimaryCamera");
            return;
        };
        let parent = commands
            .spawn((TargetCamera(camera), ActionsBarParent))
            .id();
        let bar = commands.spawn(ActionsBar).set_parent(parent).id();
        let close = FloatingActionButton::new(Small, assets.load(CLOSE_ICON));
        let edit = FloatingActionButton::new(Small, assets.load(EDIT_ICON));
        let more = FloatingActionButton::new(Small, assets.load(MORE_ICON));
        let bezier = FloatingActionButton::new(Medium, assets.load(BEZIER_ICON));
        close.spawn(&mut commands, bar);
        edit.spawn(&mut commands, bar);
        more.spawn(&mut commands, bar);
        bezier.spawn(&mut commands, bar);
    }
}
