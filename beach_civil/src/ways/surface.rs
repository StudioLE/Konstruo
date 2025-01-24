use crate::ways::way::Way;
use crate::ways::{WayMaterials, FLATTEN_TOLERANCE, OFFSET_ACCURACY};
use beach_core::beziers::CubicBezierSpline;
use beach_core::geometry::meshes::create_triangle_strip;
use beach_core::geometry::primitives::create_triangle_strip_between_polylines;
use beach_core::geometry::triangles::add_vertices_by_spliting_longest_edge;
use bevy::prelude::*;
use std::cmp::Ordering;

/// A surface formed by two lines from a [Way].
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct WaySurface {
    /// Offsets from the way.
    offsets: [f32; 2],
}

impl WaySurface {
    /// Create a new [`WaySurface`] offset from [`Way`].
    pub fn new(offsets: [f32; 2]) -> Self {
        Self { offsets }
    }

    /// Create a new [`WaySurface`] centered at [`Way`].
    pub fn centered(width: f32) -> Self {
        Self::new([width * -0.5, width * 0.5])
    }

    /// Regenerate the mesh geometry.
    pub fn regenerate(&self, meshes: &mut ResMut<Assets<Mesh>>, mut mesh: Mut<Mesh3d>, way: &Way) {
        *mesh = Mesh3d(meshes.add(self.get_mesh(way)));
    }

    /// Spawn a [`WaySurface`] with its mesh geometry.
    pub fn spawn(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &Res<WayMaterials>,
        way: &Way,
        parent: Entity,
    ) {
        let mesh = self.get_mesh(way);
        let bundle = (
            self,
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.mesh.clone()),
        );
        commands.spawn(bundle).set_parent(parent);
    }

    /// Get the splines of each edge.
    fn get_splines(&self, way: &Way) -> [CubicBezierSpline; 2] {
        [
            way.spline.offset(self.offsets[0], OFFSET_ACCURACY),
            way.spline.offset(self.offsets[1], OFFSET_ACCURACY),
        ]
    }

    /// Get the polylines of each edge.
    ///
    /// The polylines will have the same number of vertices.
    fn get_polylines(&self, way: &Way) -> [Vec<Vec3>; 2] {
        let splines = self.get_splines(way);
        let mut polylines = [
            splines[0].flatten(FLATTEN_TOLERANCE),
            splines[1].flatten(FLATTEN_TOLERANCE),
        ];
        #[allow(clippy::cast_possible_wrap)]
        let difference = polylines[0].len() as isize - polylines[1].len() as isize;
        match difference.cmp(&0) {
            Ordering::Less => {
                add_vertices_by_spliting_longest_edge(&mut polylines[0], difference.unsigned_abs());
            }
            Ordering::Greater => {
                #[allow(clippy::cast_sign_loss)]
                add_vertices_by_spliting_longest_edge(&mut polylines[1], difference as usize);
            }
            Ordering::Equal => {}
        }
        polylines
    }

    /// Get the [`Mesh`].
    fn get_mesh(&self, way: &Way) -> Mesh {
        let polylines = self.get_polylines(way);
        let triangle_strip = create_triangle_strip_between_polylines(&polylines);
        create_triangle_strip(triangle_strip)
    }
}
