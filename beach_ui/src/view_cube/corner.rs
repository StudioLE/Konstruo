use crate::cameras::orbit::Orbit;
use crate::view_cube::materials::ViewCubeMaterials;
use crate::view_cube::meshes::ViewCubeMeshes;
use crate::view_cube::side::Side;
use crate::view_cube::RENDER_LAYER;
use beach_core::mathematics::spherical_coordinate_system::cartesian_to_spherical;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

#[derive(Component)]
pub struct ViewCorner {
    #[allow(dead_code)]
    pub sides: [Side; 3],
}

impl ViewCorner {
    fn get_vector(&self) -> Vec3 {
        self.sides
            .iter()
            .fold(Vec3::ZERO, |acc, side| acc + side.get_vector())
            .normalize()
    }
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
        let vector = corner
            .iter()
            .fold(Vec3::ZERO, |acc, side| acc + side.get_vector());
        let bundle = (
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
    corner: Query<&ViewCorner>,
    mut orbit: Query<&mut Orbit>,
) {
    let Ok(corner) = corner.get(event.entity()) else {
        error!("Failed to get clicked ViewCorner");
        return;
    };
    let Ok(mut orbit) = orbit.get_single_mut() else {
        error!("Failed to get Orbit");
        return;
    };
    let vector = corner.get_vector();
    let mut spherical = cartesian_to_spherical(vector);
    spherical.x = orbit.movement.current.x;
    orbit.movement.set_target(spherical);
}
