#[allow(clippy::module_inception)]
pub mod axis_marker;
mod materials;
mod meshes;

use crate::axis_marker::axis_marker::{on_axis_marker_added, AxisMarker};
use crate::axis_marker::materials::insert_materials;
use crate::axis_marker::meshes::insert_meshes;
use bevy::prelude::*;

/// Register systems for axis marker.
pub fn axis_marker_plugin(app: &mut App) {
    app.add_systems(Startup, insert_materials)
        .add_systems(Startup, insert_meshes)
        .add_systems(Startup, spawn_positive_marker)
        .add_systems(Update, on_axis_marker_added);
}

fn spawn_positive_marker(mut commands: Commands) {
    let bundle = (
        AxisMarker {
            thickness: 1.0,
            length: 10.0,
        },
        Transform::from_translation(Vec3::splat(10.0)),
    );
    commands.spawn(bundle);
}
