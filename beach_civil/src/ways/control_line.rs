use crate::ways::materials::WayMaterials;
use crate::ways::way::Way;
use beach_core::geometry::meshes::create_linestrip;
use bevy::prelude::*;

/// Line representation of a [`Way`].
#[derive(Component)]
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
}

pub fn on_way_line_added(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<WayMaterials>,
    query: Query<(Entity, &WayLine), Added<WayLine>>,
) {
    for (entity, way_line) in query.iter() {
        let mesh = meshes.add(create_linestrip(way_line.polyline.clone()));
        let bundle = MaterialMeshBundle {
            mesh,
            material: materials.control_line.clone(),
            ..Default::default()
        };
        commands.spawn(bundle).set_parent(entity);
    }
}
