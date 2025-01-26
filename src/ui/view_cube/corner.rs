use crate::geometry::Orientation;
use crate::ui::pan_orbit::Orbit;
use crate::ui::view_cube::materials::ViewCubeMaterials;
use crate::ui::view_cube::meshes::ViewCubeMeshes;
use crate::ui::view_cube::RENDER_LAYER;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

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
            let vector = Orientation::get_vector(&orientation);
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
    event: Trigger<Pointer<Over>>,
    materials: Res<ViewCubeMaterials>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    let Ok(mut material) = query.get_mut(event.entity()) else {
        error!("Failed to get material of ViewCorner");
        return;
    };
    *material = MeshMaterial3d(materials.corner_over.clone());
}

fn on_pointer_out(
    event: Trigger<Pointer<Out>>,
    materials: Res<ViewCubeMaterials>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    let Ok(mut material) = query.get_mut(event.entity()) else {
        error!("Failed to get material of ViewCorner");
        return;
    };
    *material = MeshMaterial3d(materials.corner.clone());
}

fn on_pointer_click(
    event: Trigger<Pointer<Click>>,
    corner: Query<&ViewCubeCorner>,
    mut orbit: Query<&mut Orbit>,
) {
    let Ok(corner) = corner.get(event.entity()) else {
        error!("Failed to get clicked ViewCubeCorner");
        return;
    };
    let Ok(mut orbit) = orbit.get_single_mut() else {
        error!("Failed to get Orbit");
        return;
    };
    orbit.orientate(&corner.orientation);
}
