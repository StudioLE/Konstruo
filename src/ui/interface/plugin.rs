use super::*;
use bevy::prelude::*;

/// Plugin to spawn a bottom app bar with floating action buttons.
/// - <https://m3.material.io/components/bottom-app-bar/overview>
/// - <https://m3.material.io/components/floating-action-button/overview>
pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InterfaceState>()
            .add_systems(PostStartup, ActionBar::startup_system)
            .add_systems(Startup, InterfaceState::startup_system)
            .add_systems(Update, InterfaceState::event_system);
    }
}
