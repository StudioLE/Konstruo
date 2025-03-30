use bevy::prelude::*;

/// A closed polygon where all vertices are on the same plane.
#[derive(Clone, Debug, PartialEq)]
pub struct Polygon {
    /// Vertices
    ///
    /// The first and last vertices will be the same for a closed polygon.
    vertices: Vec<Vec3>,
}

impl Polygon {
    /// Create a [`Polygon`].
    ///
    /// TODO: Add a check to ensure the first and last vertices are the same.
    #[must_use]
    pub fn new(vertices: Vec<Vec3>) -> Self {
        Self { vertices }
    }

    /// Create a [`Polygon`] by closing an open polyline.
    #[must_use]
    pub fn from_open(mut vertices: Vec<Vec3>) -> Option<Self> {
        vertices.push(*vertices.first()?);
        Some(Self::new(vertices))
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
    /// This assumes the vertices are on the same plane and that the polygon is closed
    /// so the first and last vertices must be the same.
    #[must_use]
    #[allow(clippy::indexing_slicing)]
    pub fn get_area(&self) -> f32 {
        self.vertices
            .windows(2)
            .map(|pair| pair[0].cross(pair[1]))
            .fold(Vec3::ZERO, |sum, cross| sum + cross)
            .length()
            / 2.0
    }

    /// Get the individual edge lines of the [`Polygon`].
    #[must_use]
    #[allow(clippy::indexing_slicing)]
    pub fn get_edges(self) -> Vec<[Vec3; 2]> {
        self.vertices.windows(2).map(|x| [x[0], x[1]]).collect()
    }
}
