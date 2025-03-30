use super::*;
use crate::geometry::Orientation;
use crate::geometry::Orientation::*;
use crate::ui::Orbit;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

#[derive(Component)]
pub(super) struct ViewCubeFace {
    orientation: Orientation,
}

impl ViewCubeFace {
    /// System to spawn [`ViewCubeFace`] on startup.
    pub(super) fn startup_system(
        mut commands: Commands,
        meshes: Res<ViewCubeMeshes>,
        materials: Res<ViewCubeMaterials>,
    ) {
        for orientation in Orientation::get_all() {
            let material = match_material(&materials, &orientation);
            let vector = orientation.to_facing_in();
            let mut transform = Transform::from_translation(vector * 0.5);
            transform.look_at(Vec3::ZERO, vector.normalize());
            let bundle = (
                ViewCubeFace { orientation },
                RenderLayers::layer(RENDER_LAYER),
                Mesh3d(meshes.face.clone()),
                MeshMaterial3d(material),
                transform,
            );
            commands
                .spawn(bundle)
                .observe(on_pointer_over)
                .observe(on_pointer_out)
                .observe(on_pointer_click);
        }
    }
}

fn match_material(
    materials: &Res<ViewCubeMaterials>,
    orientation: &Orientation,
) -> Handle<StandardMaterial> {
    match orientation {
        Front | Back => materials.face_y.clone(),
        Left | Right => materials.face_x.clone(),
        Top | Bottom => materials.face_z.clone(),
    }
}

fn on_pointer_over(
    event: Trigger<Pointer<Over>>,
    materials: Res<ViewCubeMaterials>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    let Ok(mut material) = query.get_mut(event.entity()) else {
        error!("Failed to get material of ViewCubeFace");
        return;
    };
    *material = MeshMaterial3d(materials.face_over.clone());
}

fn on_pointer_out(
    event: Trigger<Pointer<Out>>,
    materials: Res<ViewCubeMaterials>,
    mut query: Query<(&ViewCubeFace, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    let Ok((face, mut material)) = query.get_mut(event.entity()) else {
        error!("Failed to get material of ViewCubeFace");
        return;
    };
    *material = MeshMaterial3d(match_material(&materials, &face.orientation));
}

fn on_pointer_click(
    event: Trigger<Pointer<Click>>,
    side: Query<&ViewCubeFace>,
    mut orbit: Query<&mut Orbit>,
) {
    let Ok(face) = side.get(event.entity()) else {
        error!("Failed to get clicked ViewCubeFace");
        return;
    };
    let Ok(mut orbit) = orbit.get_single_mut() else {
        error!("Failed to get Orbit");
        return;
    };
    debug!("Clicked ViewCubeFace: {:?}", face.orientation);
    orbit.orientate(&[face.orientation.clone()]);
}
