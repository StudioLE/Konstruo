use crate::view_cube::materials::ViewCubeMaterials;
use crate::view_cube::meshes::ViewCubeMeshes;
use crate::view_cube::side::Side;
use crate::view_cube::RENDER_LAYER;
use bevy::asset::Handle;
use bevy::math::Vec3;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{Commands, Component, Mesh, Res, Transform};
use bevy::render::view::RenderLayers;

#[derive(Component)]
pub struct ViewEdge {
    #[allow(dead_code)]
    pub sides: [Side; 2],
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
        let bundle = create_edge(meshes.edge.clone(), materials.edge.clone(), &edge);
        let layer = RenderLayers::layer(RENDER_LAYER);
        commands.spawn((bundle, layer, ViewEdge { sides: edge }));
    }
}

fn create_edge(
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    sides: &[Side; 2],
) -> PbrBundle {
    let vector = sides
        .iter()
        .fold(Vec3::ZERO, |acc, side| acc + side.get_vector());
    let mut transform = Transform::from_translation(vector * 0.4);
    transform.scale = Vec3::splat(0.6) - vector.abs() * 0.4;
    PbrBundle {
        mesh,
        material,
        transform,
        ..Default::default()
    }
}
