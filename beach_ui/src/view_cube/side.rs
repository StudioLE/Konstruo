use crate::view_cube::materials::ViewCubeMaterials;
use crate::view_cube::meshes::ViewCubeMeshes;
use crate::view_cube::RENDER_LAYER;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

#[derive(Component)]
pub struct ViewSide {
    #[allow(dead_code)]
    pub side: Side,
}

#[derive(Debug)]
pub enum Side {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
}

impl Side {
    pub fn get_vector(&self) -> Vec3 {
        match self {
            Side::Front => Vec3::NEG_Y,
            Side::Back => Vec3::Y,
            Side::Left => Vec3::NEG_X,
            Side::Right => Vec3::X,
            Side::Top => Vec3::Z,
            Side::Bottom => Vec3::NEG_Z,
        }
    }
}

pub fn spawn_sides(
    mut commands: Commands,
    meshes: Res<ViewCubeMeshes>,
    materials: Res<ViewCubeMaterials>,
) {
    let sides = [
        Side::Front,
        Side::Back,
        Side::Left,
        Side::Right,
        Side::Top,
        Side::Bottom,
    ];
    for side in sides {
        let bundle = create_view_side(meshes.side.clone(), materials.side.clone(), &side);
        let layer = RenderLayers::layer(RENDER_LAYER);
        commands.spawn((bundle, layer, ViewSide { side }));
    }
}

fn create_view_side(
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    side: &Side,
) -> PbrBundle {
    let vector = side.get_vector();
    let mut transform = Transform::from_translation(side.get_vector() * 0.4);
    transform.look_at(Vec3::ZERO, vector.normalize());
    PbrBundle {
        mesh,
        material,
        transform,
        ..Default::default()
    }
}
