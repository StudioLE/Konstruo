use crate::geometry::TriangleList;
use bevy::prelude::*;

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
    #[must_use]
    pub fn to_triangle_list(self) -> TriangleList {
        let back = [self.back_right, self.back_left, self.back_top];
        let front = [self.front_right, self.front_top, self.front_left];
        let right_bottom = [self.back_right, self.front_top, self.front_right];
        let right_top = [self.back_right, self.back_top, self.front_top];
        let left_bottom = [self.back_left, self.front_left, self.front_top];
        let left_top = [self.back_left, self.front_top, self.back_top];
        let back_bottom = [self.back_right, self.front_right, self.back_left];
        let front_bottom = [self.back_left, self.front_right, self.front_left];
        TriangleList::new([
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
}
