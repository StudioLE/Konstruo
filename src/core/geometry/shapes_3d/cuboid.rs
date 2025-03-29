use crate::geometry::*;
use bevy::prelude::*;

/// A cuboid.
/// - <https://en.wikipedia.org/wiki/Cuboid>
#[derive(Clone, Debug, Default)]
pub struct Cuboid {
    transform: Transform,
}

impl Cuboid {
    /// Create a new [`Cuboid`].
    #[must_use]
    pub fn new(transform: Transform) -> Self {
        Self { transform }
    }

    #[must_use]
    pub fn get_vertex(&self, corner: [Orientation; 3]) -> Vec3 {
        let vector = Orientation::get_vector(&corner);
        self.transform.translation + vector * 0.5
    }

    #[must_use]
    pub fn get_edge(&self, edge: [Orientation; 2]) -> [Vec3; 2] {
        let facing = Orientation::get_vector(&edge);
        let center = self.transform.translation + 0.5 * facing;
        let cross = Vec3Helpers::invert_0_and_1(facing);
        let start = center - cross * 0.5;
        let end = center + cross * 0.5;
        [start, end]
    }

    #[must_use]
    pub fn get_face(&self, face: Orientation) -> [Vec3; 4] {
        let facing = face.to_vector();
        let center = self.transform.translation + 0.5 * facing;
        let normal = if Vec3Helpers::is_almost_equal_to(facing.abs(), Vec3::X) {
            Vec3::Y
        } else {
            Vec3::X
        };
        let cross = facing.cross(normal).normalize();
        [
            center - normal * 0.5 - cross * 0.5,
            center + normal * 0.5 - cross * 0.5,
            center + normal * 0.5 + cross * 0.5,
            center - normal * 0.5 + cross * 0.5,
        ]
    }

    /// Get the edges as a [`LineList`].
    #[must_use]
    pub fn get_edges(&self) -> LineList {
        let lines = Orientation::get_all_edges()
            .map(|edge| self.get_edge(edge))
            .to_vec();
        LineList::new(lines)
    }

    /// Get the triangles as a [`TriangleList`].
    #[must_use]
    pub fn get_triangles(&self) -> TriangleList {
        let triangles = Orientation::get_all()
            .map(|face| self.get_face(face))
            .map(Triangle::from_rectangle)
            .into_iter()
            .flatten()
            .collect();
        TriangleList::new(triangles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Orientation::*;

    #[test]
    fn get_vertex() {
        // Arrange
        let center = Vec3::new(1.0, 2.0, 3.0);
        let half = Vec3::new(0.5, 0.5, 0.5);
        let cuboid = Cuboid::new(Transform::from_translation(center));

        // Act
        // Assert
        Vec3Helpers::assert_almost_equal_to(
            center - half,
            cuboid.get_vertex([Bottom, Front, Left]),
        );
        Vec3Helpers::assert_almost_equal_to(
            center + half * Vec3::new(1.0, -1.0, -1.0),
            cuboid.get_vertex([Bottom, Front, Right]),
        );
        Vec3Helpers::assert_almost_equal_to(
            center + half * Vec3::new(1.0, -1.0, 1.0),
            cuboid.get_vertex([Top, Front, Right]),
        );
        Vec3Helpers::assert_almost_equal_to(center + half, cuboid.get_vertex([Top, Back, Right]));
    }

    #[test]
    fn get_edge() {
        // Arrange
        let center = Vec3::new(1.0, 2.0, 3.0);
        let cuboid = Cuboid::new(Transform::from_translation(center));

        // Act
        let bottom_front = cuboid.get_edge([Bottom, Front]);
        let top_front = cuboid.get_edge([Top, Front]);
        let front_left = cuboid.get_edge([Front, Left]);
        let left_front = cuboid.get_edge([Left, Front]);
        let top_back = cuboid.get_edge([Top, Back]);

        // Assert
        Vec3Helpers::assert_almost_equal_to(
            cuboid.get_vertex([Bottom, Front, Left]),
            bottom_front[0],
        );
        Vec3Helpers::assert_almost_equal_to(
            cuboid.get_vertex([Bottom, Front, Right]),
            bottom_front[1],
        );
        Vec3Helpers::assert_almost_equal_to(cuboid.get_vertex([Top, Front, Left]), top_front[0]);
        Vec3Helpers::assert_almost_equal_to(cuboid.get_vertex([Top, Front, Right]), top_front[1]);
        Vec3Helpers::assert_almost_equal_to(
            cuboid.get_vertex([Bottom, Front, Left]),
            left_front[0],
        );
        Vec3Helpers::assert_almost_equal_to(cuboid.get_vertex([Top, Front, Left]), left_front[1]);
        Vec3Helpers::assert_almost_equal_to(
            cuboid.get_vertex([Bottom, Front, Left]),
            front_left[0],
        );
        Vec3Helpers::assert_almost_equal_to(cuboid.get_vertex([Top, Front, Left]), front_left[1]);
        Vec3Helpers::assert_almost_equal_to(cuboid.get_vertex([Top, Back, Left]), top_back[0]);
        Vec3Helpers::assert_almost_equal_to(cuboid.get_vertex([Top, Back, Right]), top_back[1]);
    }

    #[test]
    fn get_face() {
        // Arrange
        let center = Vec3::new(1.0, 2.0, 3.0);
        let cuboid = Cuboid::new(Transform::from_translation(center));

        // Act
        let front = cuboid.get_face(Front);
        let back = cuboid.get_face(Back);
        let left = cuboid.get_face(Left);
        let right = cuboid.get_face(Right);

        // Assert
        Vec3Helpers::assert_almost_equal_to(cuboid.get_vertex([Bottom, Front, Left]), front[0]);
        Vec3Helpers::assert_almost_equal_to(cuboid.get_vertex([Bottom, Front, Right]), front[1]);
        Vec3Helpers::assert_almost_equal_to(cuboid.get_vertex([Top, Front, Right]), front[2]);
        Vec3Helpers::assert_almost_equal_to(cuboid.get_vertex([Top, Front, Left]), front[3]);
        Vec3Helpers::assert_almost_equal_to(cuboid.get_vertex([Top, Back, Left]), back[0]);
        Vec3Helpers::assert_almost_equal_to(cuboid.get_vertex([Top, Back, Right]), back[1]);
        Vec3Helpers::assert_almost_equal_to(cuboid.get_vertex([Bottom, Back, Right]), back[2]);
        Vec3Helpers::assert_almost_equal_to(cuboid.get_vertex([Bottom, Back, Left]), back[3]);
        Vec3Helpers::assert_almost_equal_to(cuboid.get_vertex([Top, Front, Left]), left[0]);
        Vec3Helpers::assert_almost_equal_to(cuboid.get_vertex([Top, Back, Left]), left[1]);
        Vec3Helpers::assert_almost_equal_to(cuboid.get_vertex([Bottom, Back, Left]), left[2]);
        Vec3Helpers::assert_almost_equal_to(cuboid.get_vertex([Bottom, Front, Left]), left[3]);
        Vec3Helpers::assert_almost_equal_to(cuboid.get_vertex([Bottom, Front, Right]), right[0]);
        Vec3Helpers::assert_almost_equal_to(cuboid.get_vertex([Bottom, Back, Right]), right[1]);
        Vec3Helpers::assert_almost_equal_to(cuboid.get_vertex([Top, Back, Right]), right[2]);
        Vec3Helpers::assert_almost_equal_to(cuboid.get_vertex([Top, Front, Right]), right[3]);
    }
}
