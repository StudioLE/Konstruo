use crate::*;
use bevy::prelude::*;
use konstruo_ui::{PrimaryCamera, ACTION_BAR_Z};
use FloatingActionButtonSize::{Medium, Small};

/// Vertical stack of [`FloatingActionButton`].
///
/// Could alternatively be implemented as a bottom app bar:
/// - <https://m3.material.io/components/bottom-app-bar/overview>
#[derive(Component)]
pub struct ActionBar;

impl ActionBar {
    /// System to create an [`ActionBar`] positioned to the bottom left of the [`PrimaryCamera`].
    pub(crate) fn startup_system(
        mut commands: Commands,
        query: Query<Entity, With<PrimaryCamera>>,
    ) {
        let Ok(camera) = query.single() else {
            warn!("Failed to get PrimaryCamera");
            return;
        };
        commands
            .spawn(fullscreen_bundle(camera))
            .with_child(bar_bundle());
    }

    /// System to update the [`ActionBar`] when [`InterfaceState`] is changed.
    pub(crate) fn update_system(
        mut commands: Commands,
        state: Res<InterfaceState>,
        assets: Res<AssetServer>,
        buttons: Query<Entity, With<FloatingActionButton>>,
        bars: Query<Entity, With<ActionBar>>,
    ) {
        if !state.is_changed() {
            return;
        }
        let Ok(bar) = bars.single() else {
            warn!("Failed to get ActionsBar");
            return;
        };
        for entity in buttons.iter() {
            commands.entity(entity).despawn();
        }
        let actions = state.get_actions();
        spawn_actions(&mut commands, &assets, actions, bar);
    }
}

fn fullscreen_bundle(camera: Entity) -> impl Bundle {
    (
        UiTargetCamera(camera),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::End,
            justify_content: JustifyContent::End,
            ..default()
        },
        ZIndex(ACTION_BAR_Z),
        Pickable::IGNORE,
    )
}

fn bar_bundle() -> impl Bundle {
    (
        ActionBar,
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::End,
            ..default()
        },
        Pickable::IGNORE,
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
