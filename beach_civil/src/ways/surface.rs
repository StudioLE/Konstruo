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
    #[allow(dead_code)]
    offsets: [f32; 2],
    /// Cubic bezier splines of the edges.
    splines: [CubicBezierSpline; 2],
}

impl WaySurface {
    /// Create a new [`WaySurface`] offset from [`Way`].
    pub fn from_offsets(way: &Way, offsets: [f32; 2]) -> Self {
        Self {
            offsets,
            splines: [
                way.spline.offset(offsets[0], OFFSET_ACCURACY),
                way.spline.offset(offsets[1], OFFSET_ACCURACY),
            ],
        }
    }

    /// Create a new [`WaySurface`] from the center of [`Way`].
    pub fn from_center(way: &Way, width: f32) -> Self {
        Self::from_offsets(way, [width * -0.5, width * 0.5])
    }

    /// Get the polylines of each edge.
    ///
    /// The polylines will have the same number of vertices.
    pub(super) fn get_polylines(&self) -> [Vec<Vec3>; 2] {
        let mut polylines = [
            self.splines[0].flatten(FLATTEN_TOLERANCE),
            self.splines[1].flatten(FLATTEN_TOLERANCE),
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

    /// System to create mesh geometry for [`WaySurface`].
    pub(super) fn added_system(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        materials: Res<WayMaterials>,
        query: Query<(Entity, &WaySurface), Added<WaySurface>>,
    ) {
        for (entity, surface) in query.iter() {
            let polylines = surface.get_polylines();
            let triangle_strip = create_triangle_strip_between_polylines(&polylines);
            let triangle_strip = create_triangle_strip(triangle_strip);
            let bundle = (
                Mesh3d(meshes.add(triangle_strip)),
                MeshMaterial3d(materials.mesh.clone()),
            );
            commands.spawn(bundle).set_parent(entity);
        }
    }
}
