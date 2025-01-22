use crate::ways::Way;
use crate::ways::WayMaterials;
use beach_core::geometry::meshes::create_linestrip;
use bevy::prelude::*;

/// Line representation of a [`Way`].
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct WayLine {
    pub polyline: Vec<Vec3>,
}

impl WayLine {
    /// Create a new instance of [`WayLine`].
    pub fn new(polyline: Vec<Vec3>) -> Self {
        Self { polyline }
    }

    /// Create a new instance of [`WayLine`] from a [`Way`].
    pub fn from_way(way: &Way) -> Self {
        Self {
            polyline: way.get_polyline(),
        }
    }

    /// System to spawn mesh geometry when a [`WayLine`] is added
    pub(super) fn added_system(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        materials: Res<WayMaterials>,
        query: Query<(Entity, &WayLine), Added<WayLine>>,
    ) {
        for (entity, way_line) in query.iter() {
            let bundle = (
                Mesh3d(meshes.add(create_linestrip(way_line.polyline.clone()))),
                MeshMaterial3d(materials.control_line.clone()),
            );
            commands.spawn(bundle).set_parent(entity);
        }
    }
}
