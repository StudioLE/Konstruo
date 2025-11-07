use super::*;
use crate::Orbit;
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use konstruo_geometry::Orientation;

#[derive(Component)]
pub(super) struct ViewCubeCorner {
    orientation: [Orientation; 3],
}

impl ViewCubeCorner {
    /// System to spawn [`ViewCubeCorner`] on startup.
    pub(super) fn startup_system(
        mut commands: Commands,
        meshes: Res<ViewCubeMeshes>,
        materials: Res<ViewCubeMaterials>,
    ) {
        for orientation in Orientation::get_all_corners() {
            let vector = Orientation::get_facing_in(&orientation);
            let bundle = (
                ViewCubeCorner { orientation },
                Mesh3d(meshes.corner.clone()),
                MeshMaterial3d(materials.corner.clone()),
                Transform::from_translation(vector * 0.4),
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
    trigger: On<Pointer<Over>>,
    materials: Res<ViewCubeMaterials>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    let Ok(mut material) = query.get_mut(trigger.original_event_target()) else {
        error!("Failed to get material of ViewCorner");
        return;
    };
    *material = MeshMaterial3d(materials.corner_over.clone());
}

fn on_pointer_out(
    trigger: On<Pointer<Out>>,
    materials: Res<ViewCubeMaterials>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    let Ok(mut material) = query.get_mut(trigger.original_event_target()) else {
        error!("Failed to get material of ViewCorner");
        return;
    };
    *material = MeshMaterial3d(materials.corner.clone());
}

fn on_pointer_click(
    trigger: On<Pointer<Click>>,
    corner: Query<&ViewCubeCorner>,
    mut orbit: Query<&mut Orbit>,
) {
    if trigger.button != PointerButton::Primary {
        return;
    }
    let Ok(corner) = corner.get(trigger.original_event_target()) else {
        error!("Failed to get clicked ViewCubeCorner");
        return;
    };
    let Ok(mut orbit) = orbit.single_mut() else {
        error!("Failed to get Orbit");
        return;
    };
    debug!("Clicked ViewCubeCorner: {:?}", corner.orientation);
    orbit.orientate(&corner.orientation);
}
