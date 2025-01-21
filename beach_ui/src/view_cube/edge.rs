use crate::pan_orbit::Orbit;
use crate::view_cube::materials::ViewCubeMaterials;
use crate::view_cube::meshes::ViewCubeMeshes;
use crate::view_cube::side::Side;
use crate::view_cube::RENDER_LAYER;
use beach_core::mathematics::spherical_coordinate_system::cartesian_to_spherical;
use bevy::log::info;
use bevy::math::Vec3;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

#[derive(Component)]
pub struct ViewEdge {
    #[allow(dead_code)]
    pub sides: [Side; 2],
}

impl ViewEdge {
    fn get_vector(&self) -> Vec3 {
        self.sides
            .iter()
            .fold(Vec3::ZERO, |acc, side| acc + side.get_vector())
            .normalize()
    }
}

pub fn spawn_edges(
    mut commands: Commands,
    meshes: Res<ViewCubeMeshes>,
    materials: Res<ViewCubeMaterials>,
) {
    let edges = [
        [Side::Front, Side::Left],
        [Side::Front, Side::Right],
        [Side::Front, Side::Top],
        [Side::Front, Side::Bottom],
        [Side::Back, Side::Left],
        [Side::Back, Side::Right],
        [Side::Back, Side::Top],
        [Side::Back, Side::Bottom],
        [Side::Left, Side::Top],
        [Side::Left, Side::Bottom],
        [Side::Right, Side::Top],
        [Side::Right, Side::Bottom],
    ];
    for edge in edges {
        let vector = edge
            .iter()
            .fold(Vec3::ZERO, |acc, side| acc + side.get_vector());
        let mut transform = Transform::from_translation(vector * 0.4);
        transform.scale = Vec3::splat(0.6) - vector.abs() * 0.4;
        let bundle = (
            ViewEdge { sides: edge },
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
    edge: Query<&ViewEdge>,
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
    let vector = edge.get_vector();
    let mut spherical = cartesian_to_spherical(vector);
    spherical.x = orbit.translation.current.x;
    orbit.translation.set_target(spherical);
    info!("Side {:?} {:?}", edge.sides[0], edge.sides[1]);
    info!("Cartesian {:?}", vector);
    info!("Spherical {:?}", spherical);
}
