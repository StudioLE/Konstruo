use crate::geometry::*;
use bevy::prelude::*;

/// A triangular prism where the bottom is a rectangle and the front and back faces are triangles.
/// - <https://en.wikipedia.org/wiki/Triangular_prism>
#[derive(Clone, Debug)]
pub struct TriangularPrism {
    front_left: Vec3,
    front_right: Vec3,
    front_top: Vec3,
    back_left: Vec3,
    back_right: Vec3,
    back_top: Vec3,
}

impl Default for TriangularPrism {
    fn default() -> Self {
        Self {
            back_right: Vec3::new(-0.5, 0.5, -0.5),
            back_left: Vec3::new(-0.5, -0.5, -0.5),
            back_top: Vec3::new(-0.5, 0.0, 0.5),
            front_right: Vec3::new(0.5, 0.5, -0.5),
            front_left: Vec3::new(0.5, -0.5, -0.5),
            front_top: Vec3::new(0.5, 0.0, 0.5),
        }
    }
}

impl TriangularPrism {
    /// Get the triangules as a [`TriangleList`].
    #[must_use]
    pub fn to_triangles(self) -> TriangleList {
        let back = Triangle::new([self.back_right, self.back_left, self.back_top]);
        let front = Triangle::new([self.front_right, self.front_top, self.front_left]);
        let right_bottom = Triangle::new([self.back_right, self.front_top, self.front_right]);
        let right_top = Triangle::new([self.back_right, self.back_top, self.front_top]);
        let left_bottom = Triangle::new([self.back_left, self.front_left, self.front_top]);
        let left_top = Triangle::new([self.back_left, self.front_top, self.back_top]);
        let back_bottom = Triangle::new([self.back_right, self.front_right, self.back_left]);
        let front_bottom = Triangle::new([self.back_left, self.front_right, self.front_left]);
        TriangleList::new(vec![
            front,
            back,
            left_bottom,
            left_top,
            right_bottom,
            right_top,
            front_bottom,
            back_bottom,
        ])
    }

    /// Get the edges as a [`LineList`].
    #[must_use]
    pub fn to_edges(self) -> LineList {
        let lines = vec![
            [self.front_right, self.front_left],
            [self.front_left, self.front_top],
            [self.front_top, self.front_right],
            [self.back_right, self.back_left],
            [self.back_left, self.back_top],
            [self.back_top, self.back_right],
            [self.back_right, self.front_right],
            [self.back_left, self.front_left],
            [self.back_top, self.front_top],
        ];
        LineList::new(lines)
    }

    /// Apply a [`Transform`].
    #[must_use]
    pub fn with_transform(self, transform: Transform) -> Self {
        Self {
            back_right: transform.transform_point(self.back_right),
            back_left: transform.transform_point(self.back_left),
            back_top: transform.transform_point(self.back_top),
            front_right: transform.transform_point(self.front_right),
            front_left: transform.transform_point(self.front_left),
            front_top: transform.transform_point(self.front_top),
        }
    }
}
