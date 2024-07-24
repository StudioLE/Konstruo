mod corner;
mod edge;
mod materials;
mod meshes;
mod orthographic_camera;
mod perspective_camera;
mod side;
mod camera;

use crate::view_cube::corner::spawn_corners;
use crate::view_cube::edge::spawn_edges;
use crate::view_cube::materials::insert_materials;
use crate::view_cube::meshes::insert_meshes;
use crate::view_cube::orthographic_camera::{on_orbit_changed, spawn_camera};
use crate::view_cube::side::spawn_sides;
use bevy::prelude::*;
use bevy::render::view::Layer;

pub const RENDER_LAYER: Layer = 1;

/// Register systems for tools.
pub fn view_cube_plugin(app: &mut App) {
    app.add_systems(Startup, insert_materials)
        .add_systems(Startup, insert_meshes)
        .add_systems(Startup, spawn_camera)
        .add_systems(PostStartup, spawn_sides)
        .add_systems(PostStartup, spawn_edges)
        .add_systems(PostStartup, spawn_corners)
        .add_systems(Update, on_orbit_changed);
}
