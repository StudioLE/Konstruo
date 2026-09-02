use crate::Line;
use bevy::prelude::*;
use std::iter::once;
use thiserror::Error;

/// A closed ring of vertices on a single plane.
///
/// - Deviates from OGC Simple Features 06-103r4 §6.1.7, where a `LinearRing` is a curve with no area
/// - Answers the area and winding queries that the spec places on the surface
/// - Enforces closure, but not simplicity
#[derive(Clone, Debug, PartialEq)]
pub struct LinearRing {
    /// The vertices of the ring.
    ///
    /// - Repeats the first vertex as the last when closed
    vertices: Vec<Vec3>,
}

impl LinearRing {
    /// Create a [`LinearRing`] from closed vertices.
    ///
    /// - See [`LinearRing::from_open`] to close an open polyline instead
    ///
    /// # Errors
    ///
    /// - [`LinearRingError::NotClosed`] when the first and last vertices differ
    pub fn new(vertices: Vec<Vec3>) -> Result<Self, LinearRingError> {
        if vertices.first() != vertices.last() {
            return Err(LinearRingError::NotClosed);
        }
        Ok(Self { vertices })
    }

    /// Create a [`LinearRing`] by closing an open polyline.
    ///
    /// - Returns [`None`] when `vertices` is empty
    #[must_use]
    pub fn from_open(mut vertices: Vec<Vec3>) -> Option<Self> {
        vertices.push(*vertices.first()?);
        Some(Self { vertices })
    }

    /// Create a square [`LinearRing`] about `origin`.
    ///
    /// - Measures `size` as the edge length
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

    /// Create a diamond [`LinearRing`] about `origin`.
    ///
    /// - Measures `size` as the edge length, matching [`LinearRing::create_square`]
    /// - Places each vertex `size * 2.0.sqrt() / 2.0` from `origin`
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

    /// Convert the [`LinearRing`] into its vertices.
    ///
    /// - Repeats the first vertex as the last when closed
    /// - Consumes the [`LinearRing`] so no cloning takes place
    #[must_use]
    pub fn to_vertices(self) -> Vec<Vec3> {
        self.vertices
    }

    /// The borrowed vertices of the [`LinearRing`].
    ///
    /// - Repeats the first vertex as the last when closed
    #[must_use]
    pub fn get_vertices(&self) -> &Vec<Vec3> {
        &self.vertices
    }

    /// The area enclosed by the [`LinearRing`].
    ///
    /// - Is always positive, on any plane
    /// - Assumes the vertices are coplanar
    /// - See [`LinearRing::get_normal`] for winding order
    #[must_use]
    pub fn get_area(&self) -> f32 {
        self.to_newell_vector().length() / 2.0
    }

    /// The normal of the plane the [`LinearRing`] lies on.
    ///
    /// - Points in the direction from which the vertices appear counter clockwise
    /// - Handles concave rings, unlike a normal taken from three vertices
    /// - Returns [`None`] when the ring is degenerate and defines no plane
    #[must_use]
    pub fn get_normal(&self) -> Option<Vec3> {
        self.to_newell_vector().try_normalize()
    }

    /// Reverse the winding of the [`LinearRing`].
    #[must_use]
    pub fn to_reversed(self) -> Self {
        let mut vertices = self.vertices;
        vertices.reverse();
        Self { vertices }
    }

    /// Sum the cross product of each edge pair by the Newell method.
    ///
    /// - Has a length of twice the enclosed area and a direction of the plane normal
    /// - Treats the ring as cyclic, so open and closed rings both work
    fn to_newell_vector(&self) -> Vec3 {
        let Some(first) = self.vertices.first() else {
            return Vec3::ZERO;
        };
        self.vertices
            .iter()
            .zip(self.vertices.iter().skip(1).chain(once(first)))
            .map(|(a, b)| a.cross(*b))
            .fold(Vec3::ZERO, |sum, cross| sum + cross)
    }

    /// Get the individual lines that form the [`LinearRing`].
    #[must_use]
    #[expect(clippy::indexing_slicing, reason = "windows(2) yields pairs")]
    pub fn to_lines(self) -> Vec<Line> {
        self.vertices
            .windows(2)
            .map(|x| Line::new(x[0], x[1]))
            .collect()
    }

    /// Convert the [`LinearRing`] into vertices with any repeated final vertex dropped.
    ///
    /// - Treats a repeated vertex as degenerate input, which tessellation rejects
    pub(crate) fn to_open_vertices(&self) -> Vec<Vec3> {
        let mut vertices = self.vertices.clone();
        if vertices.len() > 1 && vertices.first() == vertices.last() {
            vertices.pop();
        }
        vertices
    }
}

/// Errors returned by [`LinearRing`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum LinearRingError {
    /// The vertices do not repeat the first as the last.
    #[error("ring is not closed")]
    NotClosed,
}

#[cfg(test)]
mod tests {
    use super::*;
    use konstruo_core::floats::is_almost_equal_to;

    #[test]
    fn linear_ring_get_area() {
        let ring = LinearRing::create_square(Vec3::ZERO, 2.0);
        assert!(is_almost_equal_to(ring.get_area(), 4.0));
    }

    /// The area of a ring is plane agnostic so a clockwise ring is still positive.
    #[test]
    fn linear_ring_get_area_clockwise() {
        let ring = LinearRing::create_square(Vec3::ZERO, 2.0).to_reversed();
        assert!(is_almost_equal_to(ring.get_area(), 4.0));
    }

    #[test]
    fn linear_ring_get_normal_counter_clockwise() {
        let ring = LinearRing::create_square(Vec3::ZERO, 2.0);
        assert_eq!(ring.get_normal(), Some(Vec3::Z));
    }

    #[test]
    fn linear_ring_get_normal_clockwise() {
        let ring = LinearRing::create_square(Vec3::ZERO, 2.0).to_reversed();
        assert_eq!(ring.get_normal(), Some(Vec3::NEG_Z));
    }

    /// A ring on a vertical plane has a normal on that plane, not on `+Z`.
    #[test]
    fn linear_ring_get_normal_vertical() {
        // Arrange
        let ring = LinearRing::new(vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(2.0, 0.0, 0.0),
            Vec3::new(2.0, 0.0, 2.0),
            Vec3::new(0.0, 0.0, 2.0),
            Vec3::new(0.0, 0.0, 0.0),
        ])
        .expect("ring should be closed");

        // Act
        let normal = ring.get_normal();

        // Assert
        assert_eq!(normal, Some(Vec3::NEG_Y));
        assert!(is_almost_equal_to(ring.get_area(), 4.0));
    }

    #[test]
    fn linear_ring_get_normal_degenerate() {
        let ring =
            LinearRing::new(vec![Vec3::ZERO, Vec3::X, Vec3::ZERO]).expect("ring should be closed");
        assert_eq!(ring.get_normal(), None);
    }

    #[test]
    fn linear_ring_new_not_closed() {
        let output = LinearRing::new(vec![Vec3::ZERO, Vec3::X, Vec3::Y]);
        assert_eq!(output, Err(LinearRingError::NotClosed));
    }

    #[test]
    fn linear_ring_to_open_vertices() {
        // Arrange
        let ring = LinearRing::create_square(Vec3::ZERO, 2.0);

        // Act
        let vertices = ring.to_open_vertices();

        // Assert
        assert_eq!(ring.get_vertices().len(), 5);
        assert_eq!(vertices.len(), 4);
    }
}
