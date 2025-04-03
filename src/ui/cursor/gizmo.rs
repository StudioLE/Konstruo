use crate::ui::cursor::Cursor;
use crate::ui::gizmos::{AxisGizmo, Bold};
use crate::ui::pan_orbit::PrimaryCamera;
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
    gizmos: Gizmos<Bold>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
) {
    if let Ok(position) = Cursor::from_window(&window, &camera) {
        AxisGizmo::draw_at(gizmos, position, GIZMO_LENGTH);
    }
}
