use super::*;
use bevy::app::{App, Startup};
use bevy::gizmos::AppGizmoBuilder;
use bevy::prelude::Plugin;

pub struct GizmoPlugin;

impl Plugin for GizmoPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<Thin>()
            .init_gizmo_group::<Light>()
            .init_gizmo_group::<Medium>()
            .init_gizmo_group::<Bold>()
            .init_gizmo_group::<Heavy>()
            .add_systems(Startup, GizmoWeights::startup_system);
        // .add_systems(Update, AxisGizmo::draw_at_origin);
    }
}
