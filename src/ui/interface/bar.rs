use crate::ui::FloatingActionButtonSize::{Medium, Small};
use crate::ui::*;
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

    /// System to update the [`ActionBar`] when [`InterfaceState`] is changed.
    pub(super) fn update_system(
        mut commands: Commands,
        state: Res<InterfaceState>,
        assets: Res<AssetServer>,
        buttons: Query<Entity, With<FloatingActionButton>>,
        bars: Query<Entity, With<ActionBar>>,
    ) {
        if !state.is_changed() {
            return;
        }
        let Ok(bar) = bars.get_single() else {
            warn!("Failed to get ActionsBar");
            return;
        };
        for entity in buttons.iter() {
            commands.entity(entity).despawn_recursive();
        }
        let actions = state.get_actions();
        spawn_actions(&mut commands, &assets, actions, bar);
    }
}

fn parent_bundle(camera: Entity) -> (TargetCamera, Node, ZIndex, PickingBehavior) {
    (
        TargetCamera(camera),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::End,
            justify_content: JustifyContent::End,
            ..default()
        },
        ZIndex(ACTION_BAR_Z),
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

fn spawn_actions(
    commands: &mut Commands,
    assets: &Res<AssetServer>,
    actions: Vec<Action>,
    bar: Entity,
) {
    let last = actions.len() - 1;
    for (i, action) in actions.into_iter().enumerate() {
        let size = if i == last { Medium } else { Small };
        let icon = action.icon.get_path();
        FloatingActionButton::spawn(commands, size, assets.load(icon), bar, action.on_press);
    }
}
