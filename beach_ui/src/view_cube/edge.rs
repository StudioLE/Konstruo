use crate::pan_orbit::Orbit;
use crate::view_cube::materials::ViewCubeMaterials;
use crate::view_cube::meshes::ViewCubeMeshes;
use crate::view_cube::RENDER_LAYER;
use beach_core::geometry::Orientation;
use beach_core::geometry::Orientation::*;
use beach_core::mathematics::spherical_coordinate_system::cartesian_to_spherical;
use bevy::math::Vec3;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

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
        let orientations = [
            [Front, Left],
            [Front, Right],
            [Front, Top],
            [Front, Bottom],
            [Back, Left],
            [Back, Right],
            [Back, Top],
            [Back, Bottom],
            [Left, Top],
            [Left, Bottom],
            [Right, Top],
            [Right, Bottom],
        ];
        for orientation in orientations {
            let vector = Orientation::get_vector(&orientation);
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
    event: Trigger<Pointer<Over>>,
    materials: Res<ViewCubeMaterials>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    let Ok(mut material) = query.get_mut(event.entity()) else {
        error!("Failed to get material of ViewEdge");
        return;
    };
    *material = MeshMaterial3d(materials.edge_over.clone());
}

fn on_pointer_out(
    event: Trigger<Pointer<Out>>,
    materials: Res<ViewCubeMaterials>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    let Ok(mut material) = query.get_mut(event.entity()) else {
        error!("Failed to get material of ViewEdge");
        return;
    };
    *material = MeshMaterial3d(materials.edge.clone());
}

fn on_pointer_click(
    event: Trigger<Pointer<Click>>,
    edge: Query<&ViewCubeEdge>,
    mut orbit: Query<&mut Orbit>,
) {
    let Ok(edge) = edge.get(event.entity()) else {
        error!("Failed to get clicked ViewEdge");
        return;
    };
    let Ok(mut orbit) = orbit.get_single_mut() else {
        error!("Failed to get Orbit");
        return;
    };
    let vector = Orientation::get_vector(&edge.orientation).normalize();
    let mut spherical = cartesian_to_spherical(vector);
    spherical.x = orbit.translation.current.x;
    orbit.translation.set_target(spherical);
}
