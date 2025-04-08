use super::*;
use bevy::prelude::*;

/// Plugin to spawn a bottom app bar with floating action buttons.
/// - <https://m3.material.io/components/bottom-app-bar/overview>
/// - <https://m3.material.io/components/floating-action-button/overview>
pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InterfaceState::Default)
            .add_systems(PostStartup, ActionBar::startup_system)
            .add_systems(PostStartup, Interceptor::startup_system)
            .add_systems(Update, InterfaceState::on_entity_state_changed)
            .add_systems(Update, Interceptor::update_system)
            .add_systems(Update, InterfaceState::on_entity_state_changed)
            .add_systems(Update, ActionBar::update_system);
    }
}
