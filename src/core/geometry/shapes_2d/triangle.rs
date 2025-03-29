use bevy::math::Vec3;

/// A [`Triangle`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Triangle {
    /// Vertices
    vertices: [Vec3; 3],
}

impl Triangle {
    /// Create a [`Triangle`].
    #[must_use]
    pub fn new(vertices: [Vec3; 3]) -> Self {
        Self { vertices }
    }

    /// Get the vertices of the [`Triangle`].
    ///
    /// The [`Triangle`] is consumed so no cloning takes place.
    #[must_use]
    pub fn to_vertices(self) -> [Vec3; 3] {
        self.vertices
    }

    /// Get the vertices of the [`Triangle`].
    ///
    /// The vertices are borrowed.
    #[must_use]
    pub fn get_vertices(&self) -> &[Vec3; 3] {
        &self.vertices
    }

    /// Get the normal vector of the [`Triangle`].
    #[must_use]
    pub fn get_normal(&self) -> Vec3 {
        let u = self.vertices[1] - self.vertices[0];
        let v = self.vertices[2] - self.vertices[0];
        let normal = u.cross(v);
        normal.normalize()
    }

    /// Get the normal vector of the [`Triangle`].
    #[must_use]
    pub fn from_rectangle(rectangle: [Vec3; 4]) -> [Triangle; 2] {
        [
            Triangle::new([rectangle[0], rectangle[1], rectangle[2]]),
            Triangle::new([rectangle[0], rectangle[2], rectangle[3]]),
        ]
    }
}
