use crate::Cuboid;
use crate::*;
use bevy::prelude::*;
use Orientation::*;

/// A triangular prism where the bottom is a rectangle and the front and back faces are triangles.
/// - <https://en.wikipedia.org/wiki/Triangular_prism>
#[derive(Clone, Debug, Default)]
pub struct TriangularPrism {
    bounds: Cuboid,
}

impl TriangularPrism {
    /// Create a new [`TriangularPrism`].
    #[must_use]
    pub fn new(transform: Transform) -> Self {
        Self {
            bounds: Cuboid::new(transform),
        }
    }

    #[must_use]
    fn get_front_left(&self) -> Vec3 {
        self.bounds.get_vertex([Bottom, Front, Left])
    }

    #[must_use]
    fn get_front_right(&self) -> Vec3 {
        self.bounds.get_vertex([Bottom, Front, Right])
    }

    #[must_use]
    fn get_front_top(&self) -> Vec3 {
        let edge = self.bounds.get_edge([Top, Front]);
        edge.get_midpoint()
    }

    #[must_use]
    fn get_back_left(&self) -> Vec3 {
        self.bounds.get_vertex([Bottom, Back, Left])
    }

    #[must_use]
    fn get_back_right(&self) -> Vec3 {
        self.bounds.get_vertex([Bottom, Back, Right])
    }

    #[must_use]
    fn get_back_top(&self) -> Vec3 {
        let edge = self.bounds.get_edge([Top, Back]);
        edge.get_midpoint()
    }

    /// Get the triangules as a [`TriangleList`].
    #[must_use]
    pub fn get_triangles(&self) -> TriangleList {
        let back = Triangle::new([
            self.get_back_right(),
            self.get_back_left(),
            self.get_back_top(),
        ]);
        let front = Triangle::new([
            self.get_front_left(),
            self.get_front_right(),
            self.get_front_top(),
        ]);
        let right_bottom = Triangle::new([
            self.get_back_right(),
            self.get_front_top(),
            self.get_front_right(),
        ]);
        let right_top = Triangle::new([
            self.get_back_right(),
            self.get_back_top(),
            self.get_front_top(),
        ]);
        let left_bottom = Triangle::new([
            self.get_back_left(),
            self.get_front_left(),
            self.get_front_top(),
        ]);
        let left_top = Triangle::new([
            self.get_back_left(),
            self.get_front_top(),
            self.get_back_top(),
        ]);
        let back_bottom = Triangle::new([
            self.get_back_right(),
            self.get_front_right(),
            self.get_back_left(),
        ]);
        let front_bottom = Triangle::new([
            self.get_back_left(),
            self.get_front_right(),
            self.get_front_left(),
        ]);
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
    pub fn get_edges(self) -> LineList {
        let lines = vec![
            [self.get_front_right(), self.get_front_left()],
            [self.get_front_left(), self.get_front_top()],
            [self.get_front_top(), self.get_front_right()],
            [self.get_back_right(), self.get_back_left()],
            [self.get_back_left(), self.get_back_top()],
            [self.get_back_top(), self.get_back_right()],
            [self.get_back_right(), self.get_front_right()],
            [self.get_back_left(), self.get_back_left()],
            [self.get_back_top(), self.get_front_top()],
        ];
        LineList::new(lines)
    }
}
