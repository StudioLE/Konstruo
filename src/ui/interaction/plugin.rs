use super::*;
use bevy::prelude::*;

/// Plugin to handle interaction including:
/// - [`EntityState`] components
/// - [`Action`] events
pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EntityStateChanged>()
            .add_systems(Update, Drawing::update_system);
    }
}
