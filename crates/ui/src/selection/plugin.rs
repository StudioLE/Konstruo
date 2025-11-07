use super::*;
use bevy::prelude::*;

/// Plugin to handle entity selection.
pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<EntityStateChanged>()
            .add_systems(PostStartup, Selectable::startup_system)
            .add_systems(Update, Selectable::added_system)
            .add_systems(Update, OnEntityState::on_state_changed);
    }
}
