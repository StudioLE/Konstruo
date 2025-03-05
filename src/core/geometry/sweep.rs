use crate::beziers::CubicBezierSpline;
use crate::geometry::{Polyline, TriangleList, Vec6};
use crate::infrastructure::{FLATTEN_TOLERANCE, OFFSET_ACCURACY};
use bevy::math::Vec3;
use bevy::prelude::Transform;

/// A geometric sweep with parallel edges on the ground plane.
///
/// All edges will have the same number of vertices
#[derive(Clone)]
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

    fn get_front_bottom_edge(&self) -> Polyline {
        let left = *self
            .bottom_left_edge
            .get_vertices()
            .first()
            .expect("sweep edge should have vertices");
        let right = *self
            .bottom_right_edge
            .get_vertices()
            .first()
            .expect("sweep edge should have vertices");
        Polyline::new([left, right])
    }

    fn get_front_top_edge(&self) -> Polyline {
        let left = *self
            .top_left_edge
            .get_vertices()
            .first()
            .expect("sweep edge should have vertices");
        let right = *self
            .top_right_edge
            .get_vertices()
            .first()
            .expect("sweep edge should have vertices");
        Polyline::new([left, right])
    }

    fn get_back_bottom_edge(&self) -> Polyline {
        let left = *self
            .bottom_left_edge
            .get_vertices()
            .last()
            .expect("sweep edge should have vertices");
        let right = *self
            .bottom_right_edge
            .get_vertices()
            .last()
            .expect("sweep edge should have vertices");
        Polyline::new([left, right])
    }

    fn get_back_top_edge(&self) -> Polyline {
        let left = *self
            .top_left_edge
            .get_vertices()
            .last()
            .expect("sweep edge should have vertices");
        let right = *self
            .top_right_edge
            .get_vertices()
            .last()
            .expect("sweep edge should have vertices");
        Polyline::new([left, right])
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
        let mut triangles = TriangleList::between_polylines(
            self.bottom_left_edge.clone(),
            self.top_left_edge.clone(),
        );
        triangles.merge(TriangleList::between_polylines(
            self.top_right_edge.clone(),
            self.bottom_right_edge.clone(),
        ));
        triangles.merge(TriangleList::between_polylines(
            self.get_front_top_edge(),
            self.get_front_bottom_edge(),
        ));
        triangles.merge(TriangleList::between_polylines(
            self.get_back_bottom_edge(),
            self.get_back_top_edge(),
        ));
        triangles.merge(TriangleList::between_polylines(
            self.top_left_edge,
            self.top_right_edge,
        ));
        triangles.merge(TriangleList::between_polylines(
            self.bottom_right_edge,
            self.bottom_left_edge,
        ));
        triangles
    }

    #[must_use]
    pub fn get_edges(&self) -> [Polyline; 8] {
        [
            self.bottom_left_edge.clone(),
            self.bottom_right_edge.clone(),
            self.top_left_edge.clone(),
            self.top_right_edge.clone(),
            self.get_front_bottom_edge(),
            self.get_front_top_edge(),
            self.get_back_bottom_edge(),
            self.get_back_top_edge(),
        ]
    }
}
