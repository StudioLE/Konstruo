use super::*;
use crate::Orbit;
use bevy::math::Vec3;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use konstruo_geometry::Orientation;

#[derive(Component)]
pub(super) struct ViewCubeEdge {
    orientation: [Orientation; 2],
}

impl ViewCubeEdge {
    /// System to spawn [`ViewCubeEdge`] on startup.
    pub(super) fn startup_system(
        mut commands: Commands,
        meshes: Res<ViewCubeMeshes>,
        materials: Res<ViewCubeMaterials>,
    ) {
        for orientation in Orientation::get_all_edges() {
            let vector = Orientation::get_facing_in(&orientation);
            let mut transform = Transform::from_translation(vector * 0.4);
            transform.scale = Vec3::splat(0.6) - vector.abs() * 0.4;
            let bundle = (
                ViewCubeEdge { orientation },
                Mesh3d(meshes.edge.clone()),
                MeshMaterial3d(materials.edge.clone()),
                transform,
                RenderLayers::layer(RENDER_LAYER),
            );
            commands
                .spawn(bundle)
                .observe(on_pointer_over)
                .observe(on_pointer_out)
                .observe(on_pointer_click);
        }
    }
}

fn on_pointer_over(
    trigger: Trigger<Pointer<Over>>,
    materials: Res<ViewCubeMaterials>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    let Ok(mut material) = query.get_mut(trigger.target()) else {
        error!("Failed to get material of ViewCubeEdge");
        return;
    };
    *material = MeshMaterial3d(materials.edge_over.clone());
}

fn on_pointer_out(
    trigger: Trigger<Pointer<Out>>,
    materials: Res<ViewCubeMaterials>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    let Ok(mut material) = query.get_mut(trigger.target()) else {
        error!("Failed to get material of ViewCubeEdge");
        return;
    };
    *material = MeshMaterial3d(materials.edge.clone());
}

fn on_pointer_click(
    trigger: Trigger<Pointer<Click>>,
    edge: Query<&ViewCubeEdge>,
    mut orbit: Query<&mut Orbit>,
) {
    if trigger.button != PointerButton::Primary {
        return;
    }
    let Ok(edge) = edge.get(trigger.target()) else {
        error!("Failed to get clicked ViewCubeEdge");
        return;
    };
    let Ok(mut orbit) = orbit.single_mut() else {
        error!("Failed to get Orbit");
        return;
    };
    debug!("Clicked ViewCubeEdge: {:?}", edge.orientation);
    orbit.orientate(&edge.orientation);
}
