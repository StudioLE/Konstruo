use crate::ways::edges::WayEdges2d;
use crate::ways::materials::WayMaterials;
use beach_core::geometry::meshes::create_triangle_strip;
use beach_core::geometry::primitives::create_triangle_strip_between_polylines;
use bevy::asset::Assets;
use bevy::prelude::*;

/// Line representation of the edge of a [`Way`].
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct WayMesh2d {
    polylines: [Vec<Vec3>; 2],
}

impl WayMesh2d {
    /// Create a new instance of [`WayMesh2d`].
    pub fn new(polylines: [Vec<Vec3>; 2]) -> Self {
        Self { polylines }
    }

    /// Create a new instance of [`WayMesh2d`] from [`WayEdges2d`].
    pub fn from_way_edges(way_edges2d: &WayEdges2d) -> Self {
        Self {
            polylines: way_edges2d.get_polylines(),
        }
    }
}

/// System to create mesh geometry when a [`WayMesh2d`] is added.
pub fn on_way_mesh_added(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<WayMaterials>,
    query: Query<(Entity, &WayMesh2d), Added<WayMesh2d>>,
) {
    for (entity, way_mesh) in query.iter() {
        let triangle_strip = create_triangle_strip_between_polylines(&way_mesh.polylines);
        let triangle_strip = create_triangle_strip(triangle_strip);
        let bundle = (
            Mesh3d(meshes.add(triangle_strip)),
            MeshMaterial3d(materials.mesh.clone()),
        );
        commands.spawn(bundle).set_parent(entity);
    }
}
