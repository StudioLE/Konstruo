use crate::beziers::CubicBezierSpline;
use crate::geometry::{Polyline, TriangleList, Vec6};
use crate::infrastructure::{FLATTEN_TOLERANCE, OFFSET_ACCURACY};
use bevy::math::Vec3;
use bevy::prelude::Transform;

/// A geometric sweep with parallel edges on the ground plane.
///
/// All edges will have the same number of vertices
pub struct Sweep {
    /// Vertices of the bottom left edge
    bottom_left_edge: Polyline,
    /// Vertices of the bottom right edge
    bottom_right_edge: Polyline,
    /// Vertices of the top left edge
    top_left_edge: Polyline,
    /// Vertices of the top right edge
    top_right_edge: Polyline,
}

impl Sweep {
    /// Create a new [`Sweep`] on the ground plane by offsets from a spline.
    #[must_use]
    pub fn new(spline: &CubicBezierSpline, offsets: Vec6) -> Self {
        let mut left = spline
            .offset(offsets.left, OFFSET_ACCURACY)
            .flatten(FLATTEN_TOLERANCE)
            .into();
        let mut right = spline
            .offset(offsets.right, OFFSET_ACCURACY)
            .flatten(FLATTEN_TOLERANCE)
            .into();
        Polyline::equalize_vertices_count(&mut left, &mut right);
        let bottom_transform = Transform::from_translation(Vec3::new(0.0, 0.0, offsets.bottom));
        let top_transform = Transform::from_translation(Vec3::new(0.0, 0.0, offsets.top));
        Self {
            bottom_left_edge: left.get_transformed(bottom_transform),
            bottom_right_edge: right.get_transformed(bottom_transform),
            top_left_edge: left.get_transformed(top_transform),
            top_right_edge: right.get_transformed(top_transform),
        }
    }

    /// Create a 3D [`TriangleList`] between two parallel polylines.
    ///
    /// If the polylines do not have an equal vertices count then the longest edge will be split.
    ///
    /// The [`Polyline`] are consumed so minimal cloning takes places.
    ///
    /// TODO: Check the polylines do not intersect with each another
    /// TODO: Check the polylines do not self-intersect
    #[must_use]
    #[allow(clippy::indexing_slicing)]
    pub fn to_triangle_list(self) -> TriangleList {
        let bottom_left = self.bottom_left_edge.to_vertices();
        let bottom_right = self.bottom_right_edge.to_vertices();
        let top_left = self.top_left_edge.to_vertices();
        let top_right = self.top_right_edge.to_vertices();
        let start_top = vec![
            *top_left.first().expect("first should exist"),
            *top_right.first().expect("first should exist"),
        ];
        let start_bottom = vec![
            *bottom_left.first().expect("first should exist"),
            *bottom_right.first().expect("first should exist"),
        ];
        let end_top = vec![
            *top_left.last().expect("last should exist"),
            *top_right.last().expect("last should exist"),
        ];
        let end_bottom = vec![
            *bottom_left.last().expect("first should exist"),
            *bottom_right.last().expect("last should exist"),
        ];
        let mut triangles =
            TriangleList::between_polylines(top_left.clone().into(), top_right.clone().into());
        triangles.merge(TriangleList::between_polylines(
            bottom_left.into(),
            top_left.into(),
        ));
        triangles.merge(TriangleList::between_polylines(
            top_right.into(),
            bottom_right.into(),
        ));
        triangles.merge(TriangleList::between_polylines(
            start_top.into(),
            start_bottom.into(),
        ));
        triangles.merge(TriangleList::between_polylines(
            end_bottom.into(),
            end_top.into(),
        ));
        triangles
    }
}
