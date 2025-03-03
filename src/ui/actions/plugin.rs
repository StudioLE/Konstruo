use super::*;
use crate::ui::{InterfaceState, PrimaryCamera};
use bevy::prelude::*;

/// Plugin to spawn a bottom app bar with floating action buttons.
/// - <https://m3.material.io/components/bottom-app-bar/overview>
/// - <https://m3.material.io/components/floating-action-button/overview>
pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, startup_system)
            .add_systems(Update, interface_state_changed_system);
    }
}

fn startup_system(mut commands: Commands, query: Query<Entity, With<PrimaryCamera>>) {
    let Ok(camera) = query.get_single() else {
        warn!("Failed to get PrimaryCamera");
        return;
    };
    commands.spawn(InterfaceState::Default);
    let parent = commands
        .spawn((TargetCamera(camera), ActionsBarParent))
        .id();
    commands.spawn(ActionsBar).set_parent(parent);
}

/// System to update the [`ActionsBar`] when the [`InterfaceState`] changes.
fn interface_state_changed_system(
    mut commands: Commands,
    assets: Res<AssetServer>,
    states: Query<&InterfaceState, Changed<InterfaceState>>,
    buttons: Query<Entity, With<FloatingActionButton>>,
    bars: Query<Entity, (With<ActionsBar>, Without<InterfaceState>)>,
) {
    let Ok(state) = states.get_single() else {
        return;
    };
    trace!("InterfaceState changed: {state:?}");
    let Ok(bar) = bars.get_single() else {
        warn!("Failed to get ActionsBar");
        return;
    };
    for entity in buttons.iter() {
        commands.entity(entity).despawn_recursive();
    }
    state.spawn_actions(&mut commands, &assets, bar);
}
