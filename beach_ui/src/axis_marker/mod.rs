pub mod axis_marker;
mod materials;
mod meshes;

use crate::axis_marker::axis_marker::on_axis_marker_added;
use crate::axis_marker::materials::insert_materials;
use crate::axis_marker::meshes::insert_meshes;
use bevy::prelude::*;

/// Register systems for axis marker.
pub fn axis_marker_plugin(app: &mut App) {
    app.add_systems(Startup, insert_materials)
        .add_systems(Startup, insert_meshes)
        .add_systems(Update, on_axis_marker_added);
}
