use super::*;
use bevy::prelude::*;

/// Plugin to handle entity selection.
pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EntityStateChanged>()
            .add_systems(Update, Selectable::added_system)
            .add_systems(Update, OnEntityState::on_state_changed);
    }
}
