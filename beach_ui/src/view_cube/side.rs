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
        // let over = On::<Pointer<Over>>::run(on_pointer_over);
        // let out = On::<Pointer<Out>>::run(on_pointer_out);
        // let click = On::<Pointer<Click>>::run(on_pointer_click);
        commands.spawn((bundle, layer, /*over, out, click,*/ ViewSide { side }));
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
        mesh: Mesh3d(mesh),
        material: MeshMaterial3d(material),
        transform,
        ..Default::default()
    }
}

#[cfg(ignore)]
fn on_pointer_over(
    mut commands: Commands,
    event: Listener<Pointer<Over>>,
    materials: Res<ViewCubeMaterials>,
) {
    commands
        .entity(event.target)
        .insert(MeshMaterial3d(materials.corner_over.clone()));
}

#[cfg(ignore)]
fn on_pointer_out(
    mut commands: Commands,
    event: Listener<Pointer<Out>>,
    materials: Res<ViewCubeMaterials>,
) {
    commands
        .entity(event.target)
        .insert(materials.corner.clone());
}

#[cfg(ignore)]
fn on_pointer_click(
    event: Listener<Pointer<Click>>,
    side: Query<&ViewSide>,
    mut orbit: Query<&mut Orbit>,
) {
    let side = side.get(event.target).expect("entity should exist");
    let Ok(mut orbit) = orbit.get_single_mut() else {
        return;
    };
    let vector = side.side.get_vector();
    let mut spherical = cartesian_to_spherical(vector);
    spherical.x = orbit.movement.current.x;
    orbit.movement.set_target(spherical);
    info!("Side {:?}", side.side);
    info!("Cartesian {:?}", vector);
    info!("Spherical {:?}", spherical);
}
