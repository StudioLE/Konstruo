use crate::cursor::Cursor;
use crate::pan_orbit::PrimaryCamera;
use crate::tools::gizmos::{draw_axis_gizmo, Gizmos700};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const GIZMO_LENGTH: f32 = 10.0;

pub struct CursorGizmoPlugin;

impl Plugin for CursorGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, axis_gizmo_system);
    }
}

/// A system to draw the cursor as an axis marker gizmo.
fn axis_gizmo_system(
    gizmos: Gizmos<Gizmos700>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
) {
    if let Ok(position) = Cursor::on_ground(&window, &camera) {
        draw_axis_gizmo(gizmos, position, GIZMO_LENGTH);
    };
}
