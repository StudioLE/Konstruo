use super::*;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

/// A mesh representation of the axes.
///
/// Each axis has represented with its standard color:
/// X: Red
/// Y: Green
/// Z: Blue
#[derive(Component, Debug)]
#[require(InheritedVisibility, Transform)]
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

impl AxisMarker {
    /// System to create mesh geometry when an [`AxisMarker`] is added.
    pub(super) fn added_system(
        mut commands: Commands,
        meshes: Res<AxisMarkerMeshes>,
        materials: Res<AxisMarkerMaterials>,
        query: Query<(Entity, &AxisMarker, Option<&RenderLayers>), Added<AxisMarker>>,
    ) {
        for (entity, marker, layer) in query.iter() {
            let x = (
                Mesh3d(meshes.cuboid.clone()),
                MeshMaterial3d(materials.x.clone()),
                Transform::from_scale(Vec3::new(marker.length, marker.thickness, marker.thickness)),
            );
            let y = (
                Mesh3d(meshes.cuboid.clone()),
                MeshMaterial3d(materials.y.clone()),
                Transform::from_scale(Vec3::new(marker.thickness, marker.length, marker.thickness)),
            );
            let z = (
                Mesh3d(meshes.cuboid.clone()),
                MeshMaterial3d(materials.z.clone()),
                Transform::from_scale(Vec3::new(marker.thickness, marker.thickness, marker.length)),
            );
            if let Some(layer) = layer {
                commands.spawn((x, layer.clone())).set_parent(entity);
                commands.spawn((y, layer.clone())).set_parent(entity);
                commands.spawn((z, layer.clone())).set_parent(entity);
            } else {
                commands.spawn(x).set_parent(entity);
                commands.spawn(y).set_parent(entity);
                commands.spawn(z).set_parent(entity);
            }
        }
    }
}
