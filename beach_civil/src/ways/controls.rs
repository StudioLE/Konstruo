use crate::ways::materials::WayMaterials;
use crate::ways::meshes::WayMeshes;
use beach_core::mathematics::constants::QUARTER_PI;
use bevy::prelude::*;

/// A control point that manipulates a [`Way`].
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct WayControl {
    // TODO: Remove vector and use Transform instead
    vector: Vec3,
}

/// The handle of a [`WayControl`].
#[derive(Component)]
pub struct WayControlHandle;

/// The origin of a [`WayControl`].
#[derive(Component)]
pub struct WayControlOrigin;

/// The line of a [`WayControl`].
#[derive(Component)]
pub struct WayControlLine;

impl WayControl {
    pub fn new(origin: Vec3, control: Vec3) -> (Self, Transform) {
        let vector = control - origin;
        let transform = Transform::from_translation(origin);
        (WayControl { vector }, transform)
    }

    pub fn get_scale(&self) -> f32 {
        self.vector.length()
    }

    pub fn get_rotation(&self) -> Quat {
        let angle = self.vector.angle_between(Vec3::X);
        Quat::from_axis_angle(Vec3::Z, angle)
    }

    /// System to add the visual representation of a [`WayControl`] entity.
    pub(super) fn added_system(
        mut commands: Commands,
        meshes: Res<WayMeshes>,
        materials: Res<WayMaterials>,
        query: Query<(Entity, &WayControl), Added<WayControl>>,
    ) {
        for (entity, way_control) in query.iter() {
            let bundle = (
                WayControlOrigin,
                Mesh3d(meshes.control_origin.clone()),
                MeshMaterial3d(materials.control_node.clone()),
                Transform::from_rotation(Quat::from_rotation_z(QUARTER_PI)),
            );
            commands
                .spawn(bundle)
                .observe(on_pointer_over)
                .observe(on_pointer_out)
                .set_parent(entity);
            let bundle = (
                WayControlLine,
                Mesh3d(meshes.control_line.clone()),
                MeshMaterial3d(materials.control_line.clone()),
                Transform {
                    translation: Vec3::ZERO,
                    rotation: way_control.get_rotation(),
                    scale: Vec3::new(way_control.get_scale(), 1.0, 1.0),
                },
            );
            commands.spawn(bundle).set_parent(entity);
            let bundle = (
                WayControlHandle,
                Mesh3d(meshes.control_handle.clone()),
                MeshMaterial3d(materials.control_node.clone()),
                Transform::from_translation(way_control.vector),
            );
            commands
                .spawn(bundle)
                .observe(on_pointer_over)
                .observe(on_pointer_out)
                .set_parent(entity);
        }
    }
}

fn on_pointer_over(
    event: Trigger<Pointer<Over>>,
    materials: Res<WayMaterials>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    let Ok(mut material) = query.get_mut(event.entity()) else {
        error!("Failed to get material of WayControlHandle or WayControlOrigin");
        return;
    };
    *material = MeshMaterial3d(materials.control_node_over.clone());
}

fn on_pointer_out(
    event: Trigger<Pointer<Out>>,
    materials: Res<WayMaterials>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    let Ok(mut material) = query.get_mut(event.entity()) else {
        error!("Failed to get material of WayControlHandle or WayControlOrigin");
        return;
    };
    *material = MeshMaterial3d(materials.control_node.clone());
}
