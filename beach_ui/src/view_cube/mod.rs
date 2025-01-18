use crate::axis_marker::axis_marker::AxisMarker;
use crate::view_cube::corner::spawn_corners;
use crate::view_cube::edge::spawn_edges;
use crate::view_cube::materials::insert_materials;
use crate::view_cube::meshes::insert_meshes;
use crate::view_cube::orthographic_camera::{on_orbit_changed, spawn_camera};
use crate::view_cube::side::spawn_sides;
use bevy::prelude::*;
use bevy::render::view::{Layer, RenderLayers};
use bevy_mod_picking::DefaultPickingPlugins;

mod camera;
mod corner;
mod edge;
mod materials;
mod meshes;
mod orthographic_camera;
mod perspective_camera;
mod side;

pub const RENDER_LAYER: Layer = 1;

/// Register systems for tools.
pub fn view_cube_plugin(app: &mut App) {
    app.add_systems(Startup, insert_materials)
        .add_systems(Startup, insert_meshes)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_axis_marker)
        .add_systems(PostStartup, spawn_sides)
        .add_systems(PostStartup, spawn_edges)
        .add_systems(PostStartup, spawn_corners)
        .add_systems(Update, on_orbit_changed);
    // .add_plugins(DefaultPickingPlugins);
}

pub fn spawn_axis_marker(mut commands: Commands) {
    let layer = RenderLayers::layer(RENDER_LAYER);
    commands.spawn((
        SpatialBundle::default(),
        layer,
        AxisMarker {
            length: 1.1,
            thickness: 0.2,
        },
    ));
}
