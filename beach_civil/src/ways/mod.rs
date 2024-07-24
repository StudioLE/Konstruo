use crate::ways::control_line::on_way_line_added;
use crate::ways::controls::on_way_control_added;
use crate::ways::edges::on_way_edges_added;
use crate::ways::materials::insert_materials;
use crate::ways::mesh_2d::on_way_mesh_added;
use crate::ways::meshes::insert_meshes;
use crate::ways::way::{on_way_added, spawn_way_example};
use bevy::app::{App, Startup, Update};

pub mod control_line;
pub mod controls;
pub mod edges;
mod materials;
pub mod mesh_2d;
mod meshes;
pub mod way;

/// Register systems for ways.
pub fn ways_plugin(app: &mut App) {
    app.add_systems(Startup, insert_materials)
        .add_systems(Startup, insert_meshes)
        .add_systems(Startup, spawn_way_example)
        .add_systems(Update, on_way_added)
        .add_systems(Update, on_way_control_added)
        .add_systems(Update, on_way_line_added)
        .add_systems(Update, on_way_edges_added)
        .add_systems(Update, on_way_mesh_added);
}
