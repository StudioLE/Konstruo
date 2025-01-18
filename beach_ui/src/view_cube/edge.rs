use crate::cameras::orbit::Orbit;
use crate::view_cube::materials::ViewCubeMaterials;
use crate::view_cube::meshes::ViewCubeMeshes;
use crate::view_cube::side::Side;
use crate::view_cube::RENDER_LAYER;
use beach_core::mathematics::spherical_coordinate_system::cartesian_to_spherical;
use bevy::asset::Handle;
use bevy::log::info;
use bevy::math::Vec3;
use bevy::pbr::{MeshMaterial3d, PbrBundle, StandardMaterial};
use bevy::prelude::{Commands, Component, Mesh, Mesh3d, Query, Res, Transform};
use bevy::render::view::RenderLayers;
use bevy_mod_picking::events::{Click, Out, Over, Pointer};
use bevy_mod_picking::prelude::{Listener, On};

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
        let bundle = create_edge(meshes.edge.clone(), materials.edge.clone(), &edge);
        let layer = RenderLayers::layer(RENDER_LAYER);
        // TODO: Implement bevy 0.15 picking
        // let over = On::<Pointer<Over>>::run(on_pointer_over);
        // let out = On::<Pointer<Out>>::run(on_pointer_out);
        // let click = On::<Pointer<Click>>::run(on_pointer_click);
        commands.spawn((bundle, layer, /*over, out, click,*/ ViewEdge { sides: edge }));
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
    edge: Query<&ViewEdge>,
    mut orbit: Query<&mut Orbit>,
) {
    let edge = edge.get(event.target).expect("entity should exist");
    let Ok(mut orbit) = orbit.get_single_mut() else {
        return;
    };
    let vector = edge.get_vector();
    let mut spherical = cartesian_to_spherical(vector);
    spherical.x = orbit.movement.current.x;
    orbit.movement.set_target(spherical);
    info!("Side {:?} {:?}", edge.sides[0], edge.sides[1]);
    info!("Cartesian {:?}", vector);
    info!("Spherical {:?}", spherical);
}
