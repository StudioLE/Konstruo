use crate::debug::visibility::debug_visibility_hierarchy;
use bevy::app::App;
use bevy::prelude::Update;

mod visibility;

/// Register systems for debug.
pub fn debug_plugin(app: &mut App) {
    app.add_systems(Update, debug_visibility_hierarchy);
}
