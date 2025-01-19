use crate::ways::mesh_2d::WayMesh2d;
use crate::ways::way::Way;
use beach_core::beziers::flatten::flatten_bezier;
use beach_core::beziers::offset::offset_bezier;
use beach_core::geometry::triangles::add_vertices_by_spliting_longest_edge;
use bevy::prelude::*;
use std::cmp::Ordering;

/// Line representation of the edge of a [Way].
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct WayEdges2d {
    /// Cubic bezier curves of the edges.
    curves: [Vec<[Vec3; 4]>; 2],

    /// Tolerance for flattening the bezier curves into polylines.
    flatten_tolerance: f32,
}

impl WayEdges2d {
    /// Create a new instance of [`WayEdges2d`].
    pub fn new(curves: [Vec<[Vec3; 4]>; 2], flatten_tolerance: f32) -> Self {
        Self {
            curves,
            flatten_tolerance,
        }
    }

    /// Create a new instance of [`WayEdges2d`] from a [`Way`].
    pub fn from_way(way: &Way) -> Self {
        let curve = way.get_curve();
        let half_width = way.width / 2.0;
        Self {
            curves: [
                offset_bezier(&curve, half_width * -1.0, way.offset_accuracy).control_points,
                offset_bezier(&curve, half_width, way.offset_accuracy).control_points,
            ],
            flatten_tolerance: way.flatten_tolerance,
        }
    }

    /// Get the curve geometry of each edge.
    pub fn get_curves(&self) -> [CubicBezier<Vec3>; 2] {
        [
            CubicBezier::new(self.curves[0].clone()),
            CubicBezier::new(self.curves[1].clone()),
        ]
    }

    /// Get the polylines of each edge.
    ///
    /// The polylines will have the same number of vertices.
    pub fn get_polylines(&self) -> [Vec<Vec3>; 2] {
        let mut polylines = [
            flatten_bezier(&self.get_curves()[0], self.flatten_tolerance),
            flatten_bezier(&self.get_curves()[1], self.flatten_tolerance),
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
}

/// System to create [`WayMesh2d`] when a [`WayEdges2d`] is added.
pub fn on_way_edges_added(
    mut commands: Commands,
    query: Query<(Entity, &WayEdges2d), Added<WayEdges2d>>,
) {
    for (entity, way_edges) in query.iter() {
        let mesh = WayMesh2d::from_way_edges(way_edges);
        commands.spawn(mesh).set_parent(entity);
    }
}
