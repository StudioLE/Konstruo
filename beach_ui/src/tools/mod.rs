use crate::tools::cursor::{update_cursor_position, Cursor};
use crate::tools::gizmos::{
    configure_gizmos, draw_cursor_gizmo, draw_origin_gizmo, Gizmos100,
    Gizmos300, Gizmos500, Gizmos700, Gizmos900,
};
use crate::tools::grid::spawn_grid;
use bevy::app::{App, Startup, Update};
use bevy::gizmos::AppGizmoBuilder;

pub mod cursor;
pub mod gizmos;
pub mod grid;

/// Register systems for tools.
pub fn tools_plugin(app: &mut App) {
    app.init_resource::<Cursor>()
        .init_gizmo_group::<Gizmos100>()
        .init_gizmo_group::<Gizmos300>()
        .init_gizmo_group::<Gizmos500>()
        .init_gizmo_group::<Gizmos700>()
        .init_gizmo_group::<Gizmos900>()
        .add_systems(Startup, configure_gizmos)
        // .add_systems(Startup, spawn_origin_gizmo)
        .add_systems(Startup, spawn_grid)
        .add_systems(Update, draw_origin_gizmo)
        // .add_systems(Update, draw_grid)
        // .add_systems(Update, draw_curve)
        .add_systems(Update, draw_cursor_gizmo)
        .add_systems(Update, update_cursor_position);
}
