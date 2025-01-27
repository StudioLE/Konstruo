use bevy::prelude::*;

/// A closed polygon where all vertices are on the same plane.
#[derive(Clone, Debug, PartialEq)]
pub struct Polygon {
    /// Vertices
    ///
    /// The first and last vertices will be the same for a closed polygon.
    vertices: Vec<Vec3>,
}

impl From<Vec<Vec3>> for Polygon {
    fn from(vertices: Vec<Vec3>) -> Self {
        Self { vertices }
    }
}

impl Polygon {
    /// Create a [`Polygon`].
    ///
    /// TODO: Add a check to ensure the first and last vertices are the same.
    #[must_use]
    pub fn new(vertices: impl Into<Vec<Vec3>>) -> Self {
        Self {
            vertices: vertices.into(),
        }
    }
    /// Create a square
    #[must_use]
    pub fn create_square(origin: Vec3, size: f32) -> Self {
        let half_size = size / 2.0;
        let vertices = vec![
            origin + Vec3::new(-half_size, -half_size, 0.0),
            origin + Vec3::new(half_size, -half_size, 0.0),
            origin + Vec3::new(half_size, half_size, 0.0),
            origin + Vec3::new(-half_size, half_size, 0.0),
            origin + Vec3::new(-half_size, -half_size, 0.0),
        ];
        Self { vertices }
    }

    /// Create a diamond
    #[must_use]
    pub fn create_diamond(origin: Vec3, size: f32) -> Self {
        let hypotenuse = (2.0 * size.powi(2)).sqrt();
        let half = hypotenuse / 2.0;
        let vertices = vec![
            origin + Vec3::new(0.0, -half, 0.0),
            origin + Vec3::new(half, 0.0, 0.0),
            origin + Vec3::new(0.0, half, 0.0),
            origin + Vec3::new(-half, 0.0, 0.0),
            origin + Vec3::new(0.0, -half, 0.0),
        ];
        Self { vertices }
    }

    /// Get the vertices of the [`Polygon`].
    ///
    /// The first and last vertices will be the same for a closed polygon.
    ///
    /// The [`Polygon`] is consumed so no cloning takes place.
    #[must_use]
    pub fn to_vertices(self) -> Vec<Vec3> {
        self.vertices
    }

    /// Get the vertices of the [`Polygon`].
    ///
    /// The first and last vertices will be the same for a closed polygon.
    ///
    /// The vertices are borrowed.
    #[must_use]
    pub fn get_vertices(&self) -> &Vec<Vec3> {
        &self.vertices
    }

    /// Calculate the area of a polygon.
    ///
    /// If the vertices are in clockwise order the area will be negative.
    ///
    /// <https://stackoverflow.com/a/18472899/247218>
    #[must_use]
    #[allow(clippy::indexing_slicing)]
    pub(super) fn get_area_signed(&self) -> f32 {
        let count = self.vertices.len();
        let mut sum = 0.0;
        let mut previous = self.vertices[count - 1];
        for current in &self.vertices {
            sum += (current.x - previous.x) * (current.z + previous.z);
            previous = *current;
        }
        sum / 2.0
    }

    /// Calculate the area of a polygon.
    ///
    /// The winding order of the vertices does not matter.
    #[must_use]
    pub fn get_area(&self) -> f32 {
        self.get_area_signed().abs()
    }

    /// Check if the vertices of a polygon are in counter-clockwise order.
    #[must_use]
    pub fn is_ccw(&self) -> bool {
        self.get_area_signed() > 0.0
    }
}
