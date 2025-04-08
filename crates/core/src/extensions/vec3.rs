use crate::mathematics::floats::fix_floating;
use bevy::prelude::*;

const EPSILON: f32 = 1e-6;

pub trait Vec3Extensions {
    fn is_almost_zero(self) -> bool;
    fn is_almost_equal_to(self, b: Vec3) -> bool;
    fn assert_almost_equal_to(self, actual: Vec3);
    fn fix_floating_vectors(self) -> Vec3;
    fn angle_between_on_plane(self, v2: Vec3, plane_normal: Vec3) -> f32;
    fn invert_0_and_1(self) -> Vec3;
    fn project_point_to_line(self, line: [Vec3; 2]) -> Vec3;
    fn project_point_to_line_internal(self, point_on_line: Vec3, direction: Vec3) -> Vec3;
}

pub trait VecVec3Extensions {
    fn mean(self) -> Vec3;
    fn is_ccw(self, normal: Vec3) -> Option<bool>;
}

impl Vec3Extensions for Vec3 {
    #[must_use]
    fn is_almost_zero(self) -> bool {
        self.abs_diff_eq(Vec3::ZERO, EPSILON)
    }

    #[must_use]
    fn is_almost_equal_to(self, b: Vec3) -> bool {
        self.abs_diff_eq(b, EPSILON)
    }

    fn assert_almost_equal_to(self, actual: Vec3) {
        let result = self.is_almost_equal_to(actual);
        assert!(result, "Expected: {self}, Actual: {actual}");
    }

    #[must_use]
    fn fix_floating_vectors(self) -> Vec3 {
        let x = fix_floating(self.x);
        let y = fix_floating(self.y);
        let z = fix_floating(self.z);
        Vec3::new(x, y, z)
    }

    /// Calculate the angle between two vectors on a plane.
    /// Assumes `plane_normal` is normalized.
    ///
    /// Positive angle typically means that `v1` counter-clockwise relative to `v2` around the normal of the plane.
    /// Negative Angle generally means that `v1` is clockwise relative to `v2` around the normal of the plane.
    #[must_use]
    fn angle_between_on_plane(self, v2: Vec3, plane_normal: Vec3) -> f32 {
        // Project vectors onto the plane
        let v1_proj = self - plane_normal * self.dot(plane_normal);
        let v2_proj = v2 - plane_normal * v2.dot(plane_normal);

        // Normalize the projected vectors
        let v1_proj_norm = v1_proj.normalize();
        let v2_proj_norm = v2_proj.normalize();

        // Calculate the angle using the dot product
        let cos_theta = v1_proj_norm.dot(v2_proj_norm);
        let angle = cos_theta.acos();

        // Optional: Determine the direction using the cross product
        let cross = v1_proj_norm.cross(v2_proj_norm);
        if cross.dot(plane_normal) < 0.0 {
            // If cross is in the opposite direction of the plane normal, make the angle negative
            -angle
        } else {
            angle
        }
    }

    #[must_use]
    fn invert_0_and_1(self) -> Vec3 {
        let x = if self.x == 0.0 { 1.0 } else { 0.0 };
        let y = if self.y == 0.0 { 1.0 } else { 0.0 };
        let z = if self.z == 0.0 { 1.0 } else { 0.0 };
        Vec3::new(x, y, z)
    }

    #[must_use]
    fn project_point_to_line(self, line: [Vec3; 2]) -> Vec3 {
        let vector = line[1] - line[0];
        self.project_point_to_line_internal(line[0], vector.normalize())
    }

    #[must_use]
    fn project_point_to_line_internal(self, point_on_line: Vec3, direction: Vec3) -> Vec3 {
        // Vector from line point to the point being projected
        let vector = self - point_on_line;
        // Projection of vector on line
        let projection = direction * vector.dot(direction);
        // Add projection to the line's starting point
        point_on_line + projection
    }
}

impl VecVec3Extensions for &[Vec3] {
    #[must_use]
    #[allow(clippy::as_conversions, clippy::cast_precision_loss)]
    fn mean(self) -> Vec3 {
        let total = self.iter().sum::<Vec3>();
        let count = self.len() as f32;
        total / count
    }

    #[must_use]
    #[allow(clippy::indexing_slicing)]
    fn is_ccw(self, normal: Vec3) -> Option<bool> {
        if self.len() < 3 {
            return None;
        }
        let edge1 = self[1] - self[0];
        let edge2 = self[2] - self[0];
        let cross = edge1.cross(edge2);
        let dot = cross.dot(normal);
        Some(dot > 0.0)
    }
}
