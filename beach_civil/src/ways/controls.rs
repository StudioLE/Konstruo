use crate::ways::materials::WayMaterials;
use crate::ways::meshes::WayMeshes;
use bevy::prelude::*;

/// A control point that manipulates a [`Way`].
#[derive(Component)]
pub struct WayControl {
    vector: Vec3,
}

#[derive(Bundle)]
pub struct WayControlBundle {
    way_control: WayControl,
    spatial: SpatialBundle,
}

impl WayControl {
    pub fn get_scale(&self) -> f32 {
        self.vector.length()
    }

    pub fn get_rotation(&self) -> Quat {
        let angle = self.vector.angle_between(Vec3::X);
        Quat::from_axis_angle(Vec3::Z, angle)
    }
}

impl WayControlBundle {
    pub fn new(origin: Vec3, control: Vec3) -> Self {
        let vector = control - origin;
        let transform = Transform::from_translation(origin);
        WayControlBundle {
            way_control: WayControl { vector },
            spatial: SpatialBundle::from_transform(transform),
        }
    }
}

/// System to add the visual representation of a [`WayControl`] entity.
pub fn on_way_control_added(
    mut commands: Commands,
    meshes: Res<WayMeshes>,
    materials: Res<WayMaterials>,
    query: Query<(Entity, &WayControl), Added<WayControl>>,
) {
    for (entity, way_control) in query.iter() {
        let bundle = (
            Mesh3d(meshes.control_origin.clone()),
            MeshMaterial3d(materials.control_origin.clone()),
        );
        commands.spawn(bundle).set_parent(entity);
        let bundle = (
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
            Mesh3d(meshes.control_handle.clone()),
            MeshMaterial3d(materials.control_handle.clone()),
            Transform::from_translation(way_control.vector),
        );
        commands.spawn(bundle).set_parent(entity);
    }
}
