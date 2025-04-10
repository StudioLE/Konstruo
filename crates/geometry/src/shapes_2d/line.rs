use bevy::prelude::*;
use konstruo_core::Vec3Extensions;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line {
    pub start: Vec3,
    pub end: Vec3,
}

impl Line {
    /// Create a new [`Line`].
    pub fn new(start: Vec3, end: Vec3) -> Self {
        Self { start, end }
    }

    /// Get the vector from start to end.
    pub fn get_vector(&self) -> Vec3 {
        self.end - self.start
    }

    /// Get the length.
    pub fn get_length(&self) -> f32 {
        self.get_vector().length()
    }

    /// Get the direction.
    pub fn get_direction(&self) -> Vec3 {
        self.get_vector().normalize()
    }

    /// Get the center.
    pub fn get_midpoint(&self) -> Vec3 {
        (self.start + self.end) / 2.0
    }

    /// Get the point at param along the line.
    ///
    /// Param should typically be in the range `0` to `1`.
    /// Values outside the range will be extensions of the line.
    #[must_use]
    pub fn get_point_at_param(&self, param: f32) -> Vec3 {
        self.start + self.get_vector() * param
    }

    /// Get the point of intersection with another [`Line`].
    #[allow(clippy::manual_range_contains)]
    pub fn get_intersection(&self, other: &Line) -> Option<Vec3> {
        // Direction vectors for both segments
        let dir_a = self.end - self.start;
        let dir_b = other.end - other.start;

        // Vector between the starting points of the two segments
        let between = self.start - other.start;

        // Dot products used in the computation
        let length_squared_a = dir_a.length_squared();
        let length_squared_b = dir_b.length_squared();
        let dot_dirs = dir_a.dot(dir_b);
        let dot_a_to_between = dir_a.dot(between);
        let dot_b_to_between = dir_b.dot(between);

        // Denominator of the parametric solution (used to detect parallelism)
        let denominator = length_squared_a * length_squared_b - dot_dirs * dot_dirs;
        if denominator.abs() < f32::EPSILON {
            return None; // Segments are parallel or nearly so
        }

        // Solve for parameters t and u that define the closest approach
        let t = (dot_dirs * dot_b_to_between - length_squared_b * dot_a_to_between) / denominator;
        let u = (length_squared_a * dot_b_to_between - dot_dirs * dot_a_to_between) / denominator;

        // Ensure both parameters are within [0, 1] so the points lie on the segments
        if t < 0.0 || t > 1.0 || u < 0.0 || u > 1.0 {
            return None; // Closest points fall outside the segments
        }

        // Compute the closest points on each segment
        let point_on_a = self.start + dir_a * t;
        let point_on_b = other.start + dir_b * u;

        // If the points are very close, we treat them as intersecting
        if (point_on_a - point_on_b).length_squared() < 1e-6 {
            Some((point_on_a + point_on_b) * 0.5) // Average for stability
        } else {
            None
        }
    }

    #[must_use]
    pub fn project(self, point: Vec3) -> Vec3 {
        let point_on_line = if point.is_almost_equal_to(self.start) {
            self.end
        } else {
            self.start
        };
        point.project_point_to_line(point_on_line, self.get_direction())
    }
}

impl From<[Vec3; 2]> for Line {
    fn from(value: [Vec3; 2]) -> Self {
        Self {
            start: value[0],
            end: value[1],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_intersection() {
        // Arrange
        let a = Line::new(Vec3::new(-0.5, -0.5, -0.5), Vec3::new(0.5, 0.5, 0.5));
        let b = Line::new(Vec3::new(-0.5, 0.0, 0.0), Vec3::new(0.5, 0.0, 0.0));

        // Act
        let result = a.get_intersection(&b);

        // Assert
        assert_eq!(result, Some(Vec3::new(0.0, 0.0, 0.0)));
    }

    #[test]
    fn get_intersection_parallel() {
        // Arrange
        let a = Line::new(Vec3::new(-0.5, -0.5, -0.5), Vec3::new(0.5, 0.5, 0.5));
        let b = Line::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));

        // Act
        let result = a.get_intersection(&b);

        // Assert
        assert_eq!(result, None);
    }

    #[test]
    fn get_intersection_alt() {
        // Arrange
        let a = Line::new(
            Vec3::new(35.065994, -31.019102, 0.0),
            Vec3::new(33.01777, -33.01777, 0.0),
        );
        let b = Line::new(
            Vec3::new(33.01777, -29.482233, 0.0),
            Vec3::new(101.76777, -98.23223, 0.0),
        );

        // Act
        let result = a.get_intersection(&b);

        // Assert
        assert_eq!(result, Some(Vec3::new(34.807186, -31.271648, 0.0)));
    }
}
