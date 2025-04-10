use crate::*;
use bevy::prelude::*;
use konstruo_core::Vec3Extensions;

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

    /// Get a vertex.
    #[must_use]
    pub fn get_vertex(&self, corner: [Orientation; 3]) -> Vec3 {
        let facing = Orientation::get_facing_in(&corner);
        let vertex = facing * 0.5;
        self.transform.transform_point(vertex)
    }

    /// Get the start and end of an edge.
    #[must_use]
    pub fn get_edge(&self, edge: [Orientation; 2]) -> Line {
        let facing = Orientation::get_facing_in(&edge);
        let center = 0.5 * facing;
        let cross = facing.invert_0_and_1();
        let start = center - cross * 0.5;
        let end = center + cross * 0.5;
        let start = self.transform.transform_point(start);
        let end = self.transform.transform_point(end);
        Line::new(start, end)
    }

    /// Get the vertices forming a face.
    ///
    /// When looking at the face the winding will be CCW.
    #[must_use]
    pub fn get_face(&self, face: Orientation) -> [Vec3; 4] {
        let facing = face.to_facing_in();
        let center = 0.5 * facing;
        let normal = if facing.abs().is_almost_equal_to(Vec3::X) {
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
        .map(|point| self.transform.transform_point(point))
    }

    /// Get the vertices forming a face.
    ///
    /// When looking at the face the winding will be CW.
    #[must_use]
    pub fn get_face_reversed(&self, face: Orientation) -> [Vec3; 4] {
        let mut face = self.get_face(face);
        face.reverse();
        face
    }

    /// Get the edges as a [`LineList`].
    #[must_use]
    pub fn get_edges(&self) -> LineList {
        let lines = Orientation::get_all_edges()
            .map(|edge| self.get_edge(edge))
            .to_vec();
        LineList::from_lines(lines)
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
        (center - half).assert_almost_equal_to(cuboid.get_vertex([Bottom, Front, Left]));
        (center + half * Vec3::new(1.0, -1.0, -1.0))
            .assert_almost_equal_to(cuboid.get_vertex([Bottom, Front, Right]));
        (center + half * Vec3::new(1.0, -1.0, 1.0))
            .assert_almost_equal_to(cuboid.get_vertex([Top, Front, Right]));
        (center + half).assert_almost_equal_to(cuboid.get_vertex([Top, Back, Right]));
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
        cuboid
            .get_vertex([Bottom, Front, Left])
            .assert_almost_equal_to(bottom_front.start);
        cuboid
            .get_vertex([Bottom, Front, Right])
            .assert_almost_equal_to(bottom_front.end);
        cuboid
            .get_vertex([Top, Front, Left])
            .assert_almost_equal_to(top_front.start);
        cuboid
            .get_vertex([Top, Front, Right])
            .assert_almost_equal_to(top_front.end);
        cuboid
            .get_vertex([Bottom, Front, Left])
            .assert_almost_equal_to(left_front.start);
        cuboid
            .get_vertex([Top, Front, Left])
            .assert_almost_equal_to(left_front.end);
        cuboid
            .get_vertex([Bottom, Front, Left])
            .assert_almost_equal_to(front_left.start);
        cuboid
            .get_vertex([Top, Front, Left])
            .assert_almost_equal_to(front_left.end);
        cuboid
            .get_vertex([Top, Back, Left])
            .assert_almost_equal_to(top_back.start);
        cuboid
            .get_vertex([Top, Back, Right])
            .assert_almost_equal_to(top_back.end);
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
        cuboid
            .get_vertex([Bottom, Front, Left])
            .assert_almost_equal_to(front[0]);
        cuboid
            .get_vertex([Bottom, Front, Right])
            .assert_almost_equal_to(front[1]);
        cuboid
            .get_vertex([Top, Front, Right])
            .assert_almost_equal_to(front[2]);
        cuboid
            .get_vertex([Top, Front, Left])
            .assert_almost_equal_to(front[3]);
        cuboid
            .get_vertex([Top, Back, Left])
            .assert_almost_equal_to(back[0]);
        cuboid
            .get_vertex([Top, Back, Right])
            .assert_almost_equal_to(back[1]);
        cuboid
            .get_vertex([Bottom, Back, Right])
            .assert_almost_equal_to(back[2]);
        cuboid
            .get_vertex([Bottom, Back, Left])
            .assert_almost_equal_to(back[3]);
        cuboid
            .get_vertex([Top, Front, Left])
            .assert_almost_equal_to(left[0]);
        cuboid
            .get_vertex([Top, Back, Left])
            .assert_almost_equal_to(left[1]);
        cuboid
            .get_vertex([Bottom, Back, Left])
            .assert_almost_equal_to(left[2]);
        cuboid
            .get_vertex([Bottom, Front, Left])
            .assert_almost_equal_to(left[3]);
        cuboid
            .get_vertex([Bottom, Front, Right])
            .assert_almost_equal_to(right[0]);
        cuboid
            .get_vertex([Bottom, Back, Right])
            .assert_almost_equal_to(right[1]);
        cuboid
            .get_vertex([Top, Back, Right])
            .assert_almost_equal_to(right[2]);
        cuboid
            .get_vertex([Top, Front, Right])
            .assert_almost_equal_to(right[3]);
    }
}
