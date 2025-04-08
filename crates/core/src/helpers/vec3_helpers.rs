use crate::mathematics::floats::fix_floating;
use bevy::prelude::*;

const EPSILON: f32 = 1e-6;

pub struct Vec3Helpers;

impl Vec3Helpers {
    #[must_use]
    pub fn is_almost_zero(vector: Vec3) -> bool {
        vector.abs_diff_eq(Vec3::ZERO, EPSILON)
    }

    #[must_use]
    pub fn is_almost_equal_to(a: Vec3, b: Vec3) -> bool {
        a.abs_diff_eq(b, EPSILON)
    }

    pub fn assert_almost_equal_to(expected: Vec3, actual: Vec3) {
        let result = Vec3Helpers::is_almost_equal_to(expected, actual);
        assert!(result, "Expected: {expected}, Actual: {actual}");
    }

    #[must_use]
    pub fn fix_floating_vectors(vector: Vec3) -> Vec3 {
        let x = fix_floating(vector.x);
        let y = fix_floating(vector.y);
        let z = fix_floating(vector.z);
        Vec3::new(x, y, z)
    }

    /// Calculate the angle between two vectors on a plane.
    /// Assumes `plane_normal` is normalized.
    ///
    /// Positive angle typically means that `v1` counter-clockwise relative to `v2` around the normal of the plane.
    /// Negative Angle generally means that `v1` is clockwise relative to `v2` around the normal of the plane.
    #[must_use]
    pub fn angle_between_on_plane(v1: Vec3, v2: Vec3, plane_normal: Vec3) -> f32 {
        // Project vectors onto the plane
        let v1_proj = v1 - plane_normal * v1.dot(plane_normal);
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
    pub fn invert_0_and_1(vector: Vec3) -> Vec3 {
        let x = if vector.x == 0.0 { 1.0 } else { 0.0 };
        let y = if vector.y == 0.0 { 1.0 } else { 0.0 };
        let z = if vector.z == 0.0 { 1.0 } else { 0.0 };
        Vec3::new(x, y, z)
    }

    #[must_use]
    #[allow(clippy::as_conversions, clippy::cast_precision_loss)]
    pub fn mean(vectors: &[Vec3]) -> Vec3 {
        let total = vectors.iter().sum::<Vec3>();
        let count = vectors.len() as f32;
        total / count
    }

    #[must_use]
    #[allow(clippy::indexing_slicing)]
    pub fn is_ccw(vertices: &[Vec3], normal: Vec3) -> Option<bool> {
        if vertices.len() < 3 {
            return None;
        }
        let edge1 = vertices[1] - vertices[0];
        let edge2 = vertices[2] - vertices[0];
        let cross = edge1.cross(edge2);
        let dot = cross.dot(normal);
        Some(dot > 0.0)
    }

    #[must_use]
    pub fn project_point_to_line(point: Vec3, line: [Vec3; 2]) -> Vec3 {
        let vector = line[1] - line[0];
        Vec3Helpers::project_point_to_line_internal(point, line[0], vector.normalize())
    }

    #[must_use]
    fn project_point_to_line_internal(point: Vec3, point_on_line: Vec3, direction: Vec3) -> Vec3 {
        // Vector from line point to the point being projected
        let vector = point - point_on_line;
        // Projection of vector on line
        let projection = direction * vector.dot(direction);
        // Add projection to the line's starting point
        point_on_line + projection
    }
}
