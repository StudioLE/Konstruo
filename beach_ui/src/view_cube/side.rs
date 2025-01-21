use crate::pan_orbit::Orbit;
use crate::view_cube::materials::ViewCubeMaterials;
use crate::view_cube::meshes::ViewCubeMeshes;
use crate::view_cube::RENDER_LAYER;
use beach_core::mathematics::spherical_coordinate_system::cartesian_to_spherical;
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
        let vector = side.get_vector();
        let mut transform = Transform::from_translation(side.get_vector() * 0.4);
        transform.look_at(Vec3::ZERO, vector.normalize());
        let bundle = (
            ViewSide { side },
            RenderLayers::layer(RENDER_LAYER),
            Mesh3d(meshes.side.clone()),
            MeshMaterial3d(materials.side.clone()),
            transform,
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
        error!("Failed to get material of ViewSide");
        return;
    };
    *material = MeshMaterial3d(materials.side_over.clone());
}

fn on_pointer_out(
    event: Trigger<Pointer<Out>>,
    materials: Res<ViewCubeMaterials>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    let Ok(mut material) = query.get_mut(event.entity()) else {
        error!("Failed to get material of ViewSide");
        return;
    };
    *material = MeshMaterial3d(materials.side.clone());
}

fn on_pointer_click(
    event: Trigger<Pointer<Click>>,
    side: Query<&ViewSide>,
    mut orbit: Query<&mut Orbit>,
) {
    let Ok(side) = side.get(event.entity()) else {
        error!("Failed to get clicked ViewSide");
        return;
    };
    let Ok(mut orbit) = orbit.get_single_mut() else {
        error!("Failed to get Orbit");
        return;
    };
    let vector = side.side.get_vector();
    let mut spherical = cartesian_to_spherical(vector);
    spherical.x = orbit.translation.current.x;
    orbit.translation.set_target(spherical);
    info!("Side {:?}", side.side);
    info!("Cartesian {:?}", vector);
    info!("Spherical {:?}", spherical);
}
