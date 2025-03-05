use crate::ui::PrimaryCamera;
use bevy::prelude::*;

/// Vertical stack of [`FloatingActionButton`].
///
/// Could alternatively be implemented as a bottom app bar:
/// - <https://m3.material.io/components/bottom-app-bar/overview>
#[derive(Component)]
pub struct ActionBar;

impl ActionBar {
    /// System to setup the [`ActionBar`]
    pub(super) fn startup_system(
        mut commands: Commands,
        query: Query<Entity, With<PrimaryCamera>>,
    ) {
        let Ok(camera) = query.get_single() else {
            warn!("Failed to get PrimaryCamera");
            return;
        };
        let parent = commands.spawn(parent_bundle(camera)).id();
        commands.spawn(bundle()).set_parent(parent);
    }
}

fn parent_bundle(camera: Entity) -> (TargetCamera, Node, PickingBehavior) {
    (
        TargetCamera(camera),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::End,
            justify_content: JustifyContent::End,
            ..default()
        },
        PickingBehavior::IGNORE,
    )
}

fn bundle() -> (ActionBar, Node, PickingBehavior) {
    (
        ActionBar,
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::End,
            ..default()
        },
        PickingBehavior::IGNORE,
    )
}
