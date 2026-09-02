use crate::{IndexedTriangleList, LinearRing};
use bevy::prelude::*;
use earcut::Earcut;
use thiserror::Error;

/// The fewest vertices an open ring needs to enclose an area.
const MINIMUM_RING_VERTICES: usize = 3;

/// A planar surface bounded by one exterior ring and zero or more interior rings.
///
/// - Follows OGC Simple Features 06-103r4 §6.1.11, where each interior ring defines a hole
/// - Winds every interior ring opposite the exterior ring, as §6.1.11.1 requires
/// - Enforces none of the other validity assertions of §6.1.11.1
/// - Assumes interior rings are coplanar with, and inside, the exterior ring
#[derive(Clone, Debug, PartialEq)]
pub struct Polygon {
    /// The exterior boundary of the surface.
    ///
    /// - Keeps the winding it was constructed with, so the surface may face any direction
    exterior: LinearRing,
    /// The interior boundaries of the surface.
    ///
    /// - Cuts one hole per ring
    /// - Winds opposite the exterior ring
    interiors: Vec<LinearRing>,
}

impl Polygon {
    /// Create a [`Polygon`] from an exterior ring and its interior rings.
    ///
    /// - Reverses any interior ring that winds with the exterior ring
    /// - Leaves a degenerate ring untouched, having no plane to orient against
    #[must_use]
    pub fn new(exterior: LinearRing, interiors: Vec<LinearRing>) -> Self {
        let normal = exterior.get_normal();
        let interiors = interiors
            .into_iter()
            .map(|interior| to_opposite_winding(interior, normal))
            .collect();
        Self {
            exterior,
            interiors,
        }
    }

    /// The exterior boundary of the [`Polygon`].
    #[must_use]
    pub fn get_exterior(&self) -> &LinearRing {
        &self.exterior
    }

    /// The interior boundaries of the [`Polygon`].
    #[must_use]
    pub fn get_interiors(&self) -> &Vec<LinearRing> {
        &self.interiors
    }

    /// The area of the [`Polygon`] with its holes subtracted.
    ///
    /// - Is meaningful only when interior rings lie inside the exterior ring
    /// - Assumes interior rings do not overlap each other
    #[must_use]
    pub fn get_area(&self) -> f32 {
        let interiors: f32 = self.interiors.iter().map(LinearRing::get_area).sum();
        self.exterior.get_area() - interiors
    }

    /// Triangulate the [`Polygon`] with its holes cut out.
    ///
    /// - Requires every ring to lie on the XY plane, because earcut is two dimensional
    /// - Drops Z for the earcut input, then restores it from the source vertices
    /// - Always yields `+Z` facing triangles, because earcut normalizes ring winding itself
    ///
    /// # Errors
    ///
    /// - [`TriangulateError::DegenerateExterior`] when the exterior ring encloses no area
    /// - [`TriangulateError::DegenerateInterior`] when an interior ring cuts no hole
    /// - [`TriangulateError::TooManyVertices`] when the vertex count exceeds the `u32` index range
    /// - [`TriangulateError::NoTriangles`] when earcut yields nothing
    pub fn triangulate(&self) -> Result<IndexedTriangleList, TriangulateError> {
        let mut positions = self.exterior.to_open_vertices();
        if positions.len() < MINIMUM_RING_VERTICES {
            return Err(TriangulateError::DegenerateExterior(positions.len()));
        }
        let mut hole_indices: Vec<u32> = Vec::with_capacity(self.interiors.len());
        for (index, interior) in self.interiors.iter().enumerate() {
            let interior = interior.to_open_vertices();
            if interior.len() < MINIMUM_RING_VERTICES {
                return Err(TriangulateError::DegenerateInterior {
                    index,
                    vertices: interior.len(),
                });
            }
            let start = u32::try_from(positions.len())
                .map_err(|_| TriangulateError::TooManyVertices(positions.len()))?;
            hole_indices.push(start);
            positions.extend(interior);
        }
        let mut indices: Vec<u32> = Vec::new();
        Earcut::new().earcut(
            positions.iter().map(|vertex| [vertex.x, vertex.y]),
            &hole_indices,
            &mut indices,
        );
        let triangles = IndexedTriangleList::new(positions, indices);
        if triangles.is_empty() {
            return Err(TriangulateError::NoTriangles);
        }
        Ok(triangles)
    }
}

impl From<LinearRing> for Polygon {
    fn from(exterior: LinearRing) -> Self {
        Self::new(exterior, Vec::new())
    }
}

/// Reverse `ring` when it winds the same way as `exterior`.
fn to_opposite_winding(ring: LinearRing, exterior: Option<Vec3>) -> LinearRing {
    let (Some(exterior), Some(normal)) = (exterior, ring.get_normal()) else {
        return ring;
    };
    if normal.dot(exterior) > 0.0 {
        return ring.to_reversed();
    }
    ring
}

/// Errors returned by [`Polygon::triangulate`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum TriangulateError {
    /// The exterior ring encloses no area.
    #[error("exterior ring opens to {0} vertices, expected at least {MINIMUM_RING_VERTICES}")]
    DegenerateExterior(usize),
    /// An interior ring cuts no hole.
    #[error(
        "interior ring {index} opens to {vertices} vertices, expected at least {MINIMUM_RING_VERTICES}"
    )]
    DegenerateInterior {
        /// Position of the ring in [`Polygon::get_interiors`].
        index: usize,
        /// Vertex count of the ring once opened.
        vertices: usize,
    },
    /// The vertex count exceeds the `u32` index range earcut addresses.
    #[error("polygon holds {0} vertices, exceeding the u32 index range")]
    TooManyVertices(usize),
    /// Earcut produced no triangles from rings that are individually valid.
    #[error("earcut produced no triangles")]
    NoTriangles,
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;
    use konstruo_core::floats::is_almost_equal_to;

    #[test]
    fn polygon_get_area() {
        let exterior = LinearRing::create_square(Vec3::ZERO, 4.0);
        let interior = LinearRing::create_square(Vec3::ZERO, 2.0);
        let polygon = Polygon::new(exterior, vec![interior]);
        assert!(is_almost_equal_to(polygon.get_area(), 12.0));
    }

    #[test]
    fn polygon_triangulate() {
        // Arrange
        let polygon = Polygon::from(LinearRing::create_square(Vec3::ZERO, 2.0));

        // Act
        let triangles = polygon.triangulate().expect("square should triangulate");

        // Assert
        assert_eq!(triangles.get_positions().len(), 4);
        assert_eq!(triangles.get_triangle_count(), 2);
    }

    /// An interior ring given the same winding as the exterior is reversed on construction.
    #[test]
    fn polygon_new_winds_interiors_opposite() {
        // Arrange
        let exterior = LinearRing::create_square(Vec3::ZERO, 4.0);
        let interior = LinearRing::create_square(Vec3::ZERO, 2.0);

        // Act
        let polygon = Polygon::new(exterior, vec![interior]);

        // Assert
        assert_eq!(polygon.get_exterior().get_normal(), Some(Vec3::Z));
        let interior = polygon.get_interiors().first().expect("hole should exist");
        assert_eq!(interior.get_normal(), Some(Vec3::NEG_Z));
    }

    /// The winding rule is relative, so it holds on a plane other than XY.
    #[test]
    fn polygon_new_winds_interiors_opposite_vertical() {
        // Arrange
        let exterior = to_vertical_square(4.0);
        let interior = to_vertical_square(2.0);

        // Act
        let polygon = Polygon::new(exterior, vec![interior]);

        // Assert
        assert_eq!(polygon.get_exterior().get_normal(), Some(Vec3::NEG_Y));
        let interior = polygon.get_interiors().first().expect("hole should exist");
        assert_eq!(interior.get_normal(), Some(Vec3::Y));
    }

    /// A clockwise exterior ring must still produce `+Z` facing triangles.
    #[test]
    fn polygon_triangulate_clockwise() {
        // Arrange
        let polygon = Polygon::from(LinearRing::create_square(Vec3::ZERO, 2.0).to_reversed());

        // Act
        let triangles = polygon.triangulate().expect("square should triangulate");

        // Assert
        for normal in to_normals(&triangles) {
            assert_eq!(normal, Vec3::Z);
        }
    }

    #[test]
    fn polygon_triangulate_with_hole() {
        // Arrange
        let exterior = LinearRing::create_square(Vec3::ZERO, 4.0);
        let interior = LinearRing::create_square(Vec3::ZERO, 2.0);
        let polygon = Polygon::new(exterior, vec![interior]);

        // Act
        let triangles = polygon.triangulate().expect("square should triangulate");

        // Assert
        assert_eq!(triangles.get_positions().len(), 8);
        assert_eq!(triangles.get_triangle_count(), 8);
        for normal in to_normals(&triangles) {
            assert_eq!(normal, Vec3::Z);
        }
        assert_yaml_snapshot!(triangles.get_indices());
    }

    #[test]
    fn polygon_triangulate_degenerate_exterior() {
        // Arrange
        let ring =
            LinearRing::new(vec![Vec3::ZERO, Vec3::X, Vec3::ZERO]).expect("ring should be closed");
        let polygon = Polygon::from(ring);

        // Act
        let output = polygon.triangulate();

        // Assert
        assert_eq!(output, Err(TriangulateError::DegenerateExterior(2)));
    }

    #[test]
    fn polygon_triangulate_degenerate_interior() {
        // Arrange
        let exterior = LinearRing::create_square(Vec3::ZERO, 4.0);
        let interior =
            LinearRing::new(vec![Vec3::ZERO, Vec3::X, Vec3::ZERO]).expect("ring should be closed");
        let polygon = Polygon::new(exterior, vec![interior]);

        // Act
        let output = polygon.triangulate();

        // Assert
        assert_eq!(
            output,
            Err(TriangulateError::DegenerateInterior {
                index: 0,
                vertices: 2
            })
        );
    }

    /// Three collinear vertices form a ring earcut cannot make a triangle from.
    #[test]
    fn polygon_triangulate_no_triangles() {
        // Arrange
        let vertices = vec![Vec3::ZERO, Vec3::X, Vec3::X * 2.0, Vec3::ZERO];
        let ring = LinearRing::new(vertices).expect("ring should be closed");
        let polygon = Polygon::from(ring);

        // Act
        let output = polygon.triangulate();

        // Assert
        assert_eq!(output, Err(TriangulateError::NoTriangles));
    }

    /// Create a square [`LinearRing`] on the XZ plane, facing `-Y`.
    fn to_vertical_square(size: f32) -> LinearRing {
        let half = size / 2.0;
        LinearRing::new(vec![
            Vec3::new(-half, 0.0, -half),
            Vec3::new(half, 0.0, -half),
            Vec3::new(half, 0.0, half),
            Vec3::new(-half, 0.0, half),
            Vec3::new(-half, 0.0, -half),
        ])
        .expect("ring should be closed")
    }

    /// Create the normal of each triangle from its wound vertices.
    fn to_normals(triangles: &IndexedTriangleList) -> Vec<Vec3> {
        let positions = triangles.get_positions();
        triangles
            .get_indices()
            .chunks_exact(3)
            .filter_map(|chunk| {
                let mut vertices = chunk.iter().filter_map(|index| {
                    usize::try_from(*index)
                        .ok()
                        .and_then(|index| positions.get(index))
                });
                let a = *vertices.next()?;
                let b = *vertices.next()?;
                let c = *vertices.next()?;
                Some((b - a).cross(c - a).normalize())
            })
            .collect()
    }
}
