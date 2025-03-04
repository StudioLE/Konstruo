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
            .add_systems(Update, InterfaceState::event_system)
            .add_event::<InterfaceState>();
    }
}

fn startup_system(
    mut commands: Commands,
    mut interface_state: EventWriter<InterfaceState>,
    query: Query<Entity, With<PrimaryCamera>>,
) {
    let Ok(camera) = query.get_single() else {
        warn!("Failed to get PrimaryCamera");
        return;
    };
    interface_state.send(InterfaceState::Default);
    let parent = commands
        .spawn((TargetCamera(camera), ActionsBarParent))
        .id();
    commands.spawn(ActionsBar).set_parent(parent);
}
