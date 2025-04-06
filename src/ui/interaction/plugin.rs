use super::*;
use bevy::prelude::*;

/// Plugin to handle different UI interaction modes including:
/// - [`Drawing`]
/// - [`Selection`]
pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Drawing::update_system);
    }
}
