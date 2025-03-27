use crate::geometry::*;
use bevy::prelude::*;

/// A cuboid.
/// - <https://en.wikipedia.org/wiki/Cuboid>
#[derive(Clone, Debug)]
pub struct Cuboid {
    bottom_front_left: Vec3,
    bottom_front_right: Vec3,
    bottom_back_left: Vec3,
    bottom_back_right: Vec3,
    top_front_left: Vec3,
    top_front_right: Vec3,
    top_back_left: Vec3,
    top_back_right: Vec3,
}

impl Default for Cuboid {
    fn default() -> Self {
        Self {
            bottom_front_left: Vec3::new(-0.5, -0.5, -0.5),
            bottom_front_right: Vec3::new(0.5, -0.5, -0.5),
            bottom_back_left: Vec3::new(-0.5, 0.5, -0.5),
            bottom_back_right: Vec3::new(0.5, 0.5, -0.5),
            top_front_left: Vec3::new(-0.5, -0.5, 0.5),
            top_front_right: Vec3::new(0.5, -0.5, 0.5),
            top_back_left: Vec3::new(-0.5, 0.5, 0.5),
            top_back_right: Vec3::new(0.5, 0.5, 0.5),
        }
    }
}

impl Cuboid {
    /// Get the triangules as a [`TriangleList`].
    #[must_use]
    pub fn to_triangles(self) -> TriangleList {
        let front_bottom = Triangle::new([
            self.bottom_front_left,
            self.bottom_front_right,
            self.top_front_left,
        ]);
        let front_top = Triangle::new([
            self.top_front_left,
            self.bottom_front_right,
            self.top_front_right,
        ]);
        let right_bottom = Triangle::new([
            self.bottom_front_right,
            self.bottom_back_right,
            self.top_front_right,
        ]);
        let right_top = Triangle::new([
            self.top_front_right,
            self.bottom_back_right,
            self.top_back_right,
        ]);
        let back_bottom = Triangle::new([
            self.bottom_back_right,
            self.bottom_back_left,
            self.top_back_right,
        ]);
        let back_top = Triangle::new([
            self.top_back_right,
            self.bottom_back_left,
            self.top_back_left,
        ]);
        let left_bottom = Triangle::new([
            self.bottom_back_left,
            self.bottom_front_left,
            self.top_back_left,
        ]);
        let left_top = Triangle::new([
            self.top_back_left,
            self.bottom_front_left,
            self.top_front_left,
        ]);
        let bottom_front = Triangle::new([
            self.bottom_front_left,
            self.bottom_back_right,
            self.bottom_front_right,
        ]);
        let bottom_back = Triangle::new([
            self.bottom_back_left,
            self.bottom_back_right,
            self.bottom_front_left,
        ]);
        let top_front = Triangle::new([
            self.top_front_left,
            self.top_front_right,
            self.top_back_right,
        ]);
        let top_back =
            Triangle::new([self.top_back_left, self.top_front_left, self.top_back_right]);
        TriangleList::new(vec![
            front_bottom,
            front_top,
            right_bottom,
            right_top,
            back_bottom,
            back_top,
            left_bottom,
            left_top,
            bottom_front,
            bottom_back,
            top_front,
            top_back,
        ])
    }

    /// Get the edges as a [`LineList`].
    #[must_use]
    pub fn to_edges(self) -> LineList {
        let lines = vec![
            [self.bottom_front_left, self.bottom_front_right],
            [self.bottom_front_right, self.bottom_back_right],
            [self.bottom_back_right, self.bottom_back_left],
            [self.bottom_back_left, self.bottom_front_left],
            [self.top_front_left, self.top_front_right],
            [self.top_front_right, self.top_back_right],
            [self.top_back_right, self.top_back_left],
            [self.top_back_left, self.top_front_left],
            [self.bottom_front_left, self.top_front_left],
            [self.bottom_front_right, self.top_front_right],
            [self.bottom_back_right, self.top_back_right],
            [self.bottom_back_left, self.top_back_left],
        ];
        LineList::new(lines)
    }

    /// Apply a [`Transform`].
    #[must_use]
    pub fn with_transform(self, transform: Transform) -> Self {
        Self {
            bottom_front_left: transform.transform_point(self.bottom_front_left),
            bottom_front_right: transform.transform_point(self.bottom_front_right),
            bottom_back_left: transform.transform_point(self.bottom_back_left),
            bottom_back_right: transform.transform_point(self.bottom_back_right),
            top_front_left: transform.transform_point(self.top_front_left),
            top_front_right: transform.transform_point(self.top_front_right),
            top_back_left: transform.transform_point(self.top_back_left),
            top_back_right: transform.transform_point(self.top_back_right),
        }
    }
}
