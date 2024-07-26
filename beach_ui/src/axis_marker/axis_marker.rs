use crate::axis_marker::materials::AxisMarkerMaterials;
use crate::axis_marker::meshes::AxisMarkerMeshes;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct AxisMarker {
    pub thickness: f32,
    pub length: f32,
}

impl Default for AxisMarker {
    fn default() -> Self {
        Self {
            thickness: 0.1,
            length: 1.0,
        }
    }
}

/// System to create mesh geometry when an [`AxisMarker`] is added.
pub fn on_axis_marker_added(
    mut commands: Commands,
    meshes: Res<AxisMarkerMeshes>,
    materials: Res<AxisMarkerMaterials>,
    query: Query<(Entity, &AxisMarker), Added<AxisMarker>>,
) {
    for (entity, marker) in query.iter() {
        let x = PbrBundle {
            mesh: meshes.cuboid.clone(),
            material: materials.x.clone(),
            transform: Transform::from_scale(Vec3::new(
                marker.length,
                marker.thickness,
                marker.thickness,
            )),
            ..default()
        };
        let y = PbrBundle {
            mesh: meshes.cuboid.clone(),
            material: materials.y.clone(),
            transform: Transform::from_scale(Vec3::new(
                marker.thickness,
                marker.length,
                marker.thickness,
            )),
            ..default()
        };
        let z = PbrBundle {
            mesh: meshes.cuboid.clone(),
            material: materials.z.clone(),
            transform: Transform::from_scale(Vec3::new(
                marker.thickness,
                marker.thickness,
                marker.length,
            )),
            ..default()
        };
        info!("Spawning axis marker geometry for entity {:?}", entity);
        commands.spawn(x).set_parent(entity);
        commands.spawn(y).set_parent(entity);
        commands.spawn(z).set_parent(entity);
    }
}
