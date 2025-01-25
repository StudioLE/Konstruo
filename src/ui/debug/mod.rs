use crate::ui::debug::visibility::debug_visibility_hierarchy;
use crate::ui::debug::world::debug_world;
use bevy::prelude::*;

mod visibility;
mod world;

/// Register systems for debug.
pub fn debug_plugin(app: &mut App) {
    app.add_systems(Update, debug_visibility_hierarchy)
        .add_systems(PostUpdate, debug_world);
}
