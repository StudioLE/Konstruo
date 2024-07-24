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
pub struct ViewCorner {
    #[allow(dead_code)]
    pub sides: [Side; 3],
}

pub fn spawn_corners(
    mut commands: Commands,
    meshes: Res<ViewCubeMeshes>,
    materials: Res<ViewCubeMaterials>,
) {
    let corners = [
        [Side::Front, Side::Left, Side::Top],
        [Side::Front, Side::Right, Side::Top],
        [Side::Front, Side::Left, Side::Bottom],
        [Side::Front, Side::Right, Side::Bottom],
        [Side::Back, Side::Left, Side::Top],
        [Side::Back, Side::Right, Side::Top],
        [Side::Back, Side::Left, Side::Bottom],
        [Side::Back, Side::Right, Side::Bottom],
    ];
    for corner in corners {
        let bundle = create_corner(meshes.corner.clone(), materials.corner.clone(), &corner);
        let layer = RenderLayers::layer(RENDER_LAYER);
        commands.spawn((bundle, layer, ViewCorner { sides: corner }));
    }
}
fn create_corner(
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    sides: &[Side; 3],
) -> PbrBundle {
    let vector = sides
        .iter()
        .fold(Vec3::ZERO, |acc, side| acc + side.get_vector());
    PbrBundle {
        mesh,
        material,
        transform: Transform::from_translation(vector * 0.4),
        ..Default::default()
    }
}
