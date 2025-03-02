use super::*;
use crate::ui::{PrimaryCamera, CLOSE_ICON, EDIT_ICON, GESTURE_ICON, MORE_ICON};
use bevy::prelude::*;

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

        let button = commands.spawn(ActionButton).set_parent(bar).id();
        let icon = (
            ImageNode::new(assets.load(CLOSE_ICON)),
            Node {
                height: Val::Px(24.0),
                width: Val::Px(24.0),
                ..default()
            },
        );
        commands.spawn(icon).set_parent(button);

        let button = commands.spawn(ActionButton).set_parent(bar).id();
        let icon = (
            ImageNode::new(assets.load(EDIT_ICON)),
            Node {
                height: Val::Px(24.0),
                width: Val::Px(24.0),
                ..default()
            },
        );
        commands.spawn(icon).set_parent(button);

        let button = commands.spawn(ActionButton).set_parent(bar).id();
        let icon = (
            ImageNode::new(assets.load(MORE_ICON)),
            Node {
                height: Val::Px(24.0),
                width: Val::Px(24.0),
                ..default()
            },
        );
        commands.spawn(icon).set_parent(button);

        let button = commands.spawn(FloatingActionButton).set_parent(bar).id();
        let icon = (
            ImageNode::new(assets.load(GESTURE_ICON)),
            Node {
                height: Val::Px(24.0),
                width: Val::Px(24.0),
                ..default()
            },
        );
        commands.spawn(icon).set_parent(button);
    }
}
