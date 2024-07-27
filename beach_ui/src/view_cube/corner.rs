use crate::cameras::orbit::Orbit;
use crate::view_cube::materials::ViewCubeMaterials;
use crate::view_cube::meshes::ViewCubeMeshes;
use crate::view_cube::side::Side;
use crate::view_cube::RENDER_LAYER;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_mod_picking::prelude::*;
use beach_core::mathematics::spherical_coordinate_system::cartesian_to_spherical;

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
        let bundle = create_corner(meshes.corner.clone(), materials.corner.clone(), &corner);
        let layer = RenderLayers::layer(RENDER_LAYER);
        let over = On::<Pointer<Over>>::run(on_pointer_over);
        let out = On::<Pointer<Out>>::run(on_pointer_out);
        let click = On::<Pointer<Click>>::run(on_pointer_click);
        commands.spawn((
            bundle,
            layer,
            over,
            out,
            click,
            ViewCorner { sides: corner },
        ));
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

fn on_pointer_over(
    mut commands: Commands,
    event: Listener<Pointer<Over>>,
    materials: Res<ViewCubeMaterials>,
) {
    commands
        .entity(event.target)
        .insert(materials.corner_over.clone());
}

fn on_pointer_out(
    mut commands: Commands,
    event: Listener<Pointer<Out>>,
    materials: Res<ViewCubeMaterials>,
) {
    commands
        .entity(event.target)
        .insert(materials.corner.clone());
}

fn on_pointer_click(
    event: Listener<Pointer<Click>>,
    corner: Query<&ViewCorner>,
    mut orbit: Query<&mut Orbit>,
) {
    let corner = corner.get(event.target).expect("entity should exist");
    let Ok(mut orbit) = orbit.get_single_mut() else {
        return;
    };
    let vector = corner.get_vector();
    let mut spherical = cartesian_to_spherical(vector);
    spherical.x = orbit.movement.current.x;
    orbit.movement.set_target(spherical);
    info!(
        "Sides {:?} {:?} {:?}",
        corner.sides[0], corner.sides[1], corner.sides[2]
    );
    info!("Cartesian {:?}", vector);
    info!("Spherical {:?}", spherical);
}
