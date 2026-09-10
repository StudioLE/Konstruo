use bevy::prelude::*;
use konstruo_geography_core::{F32Extensions, HeightChunk, CHUNK_SIZE};
use konstruo_geometry::TriangleList;

/// Faceted mesh of one [`HeightChunk`], at true elevation in world space.
pub struct HeightChunkMesh;

impl HeightChunkMesh {
    /// Create a [`TriangleList`] from the interior grid of a [`HeightChunk`].
    ///
    /// - Discards the border ring, which exists to average normals across a
    ///   seam, and faceted shading never does
    /// - Discards any quad with an unsurveyed corner, leaving a hole
    #[must_use]
    pub fn from_chunk(chunk: &HeightChunk) -> TriangleList {
        let across = chunk.spacing.vertices_across();
        let mut rectangles = Vec::new();
        for row in 1..across - 2 {
            for column in 1..across - 2 {
                if let Some(rectangle) = to_rectangle(chunk, row, column) {
                    rectangles.push(rectangle);
                }
            }
        }
        TriangleList::from_rectangles(rectangles)
    }
}

/// Create the four corners of one quad, south-west first, winding anti-clockwise.
///
/// - `row` and `column` are the north-west corner of the quad
/// - Winds so both triangles of the quad take a normal with a positive Z
/// - Returns [`None`] IF any corner is beyond the chunk or unsurveyed
fn to_rectangle(chunk: &HeightChunk, row: usize, column: usize) -> Option<[Vec3; 4]> {
    Some([
        to_vertex(chunk, row + 1, column)?,
        to_vertex(chunk, row + 1, column + 1)?,
        to_vertex(chunk, row, column + 1)?,
        to_vertex(chunk, row, column)?,
    ])
}

/// Create the world position of one vertex, at true elevation.
///
/// - Returns [`None`] IF the vertex is beyond the chunk or unsurveyed
fn to_vertex(chunk: &HeightChunk, row: usize, column: usize) -> Option<Vec3> {
    let height = chunk.get_height(row, column)?;
    if height.is_no_data() {
        return None;
    }
    let row = i32::try_from(row).expect("row should fit i32");
    let column = i32::try_from(column).expect("column should fit i32");
    let meters = chunk.spacing.meters();
    let x = chunk.index.x * CHUNK_SIZE + (column - 1) * meters;
    let y = chunk.index.y * CHUNK_SIZE + CHUNK_SIZE - (row - 1) * meters;
    Some(Vec3::new(to_meters(x), to_meters(y), height))
}

/// Convert a world coordinate to the `f32` a mesh vertex holds.
#[expect(
    clippy::as_conversions,
    clippy::cast_precision_loss,
    reason = "exact within f32"
)]
fn to_meters(value: i32) -> f32 {
    value as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use konstruo_geography_core::{ChunkIndex, HeightSpacing, NO_DATA};

    /// Spacing with the fewest vertices, so tests stay small.
    const SPACING: HeightSpacing = HeightSpacing::Sixteen;

    /// Triangles of a fully surveyed chunk at [`SPACING`], being 32 by 32 quads.
    const TRIANGLES: usize = 2048;

    #[test]
    fn height_chunk_mesh_from_chunk() {
        // Arrange
        let chunk = chunk(ChunkIndex::new(0, 0), None);

        // Act
        let triangles = HeightChunkMesh::from_chunk(&chunk);

        // Assert
        assert_eq!(triangles.get_triangles().len(), TRIANGLES);
    }

    /// Every face points away from the ground, so the surface is lit from above.
    #[test]
    fn height_chunk_mesh_from_chunk_normals() {
        // Arrange
        let chunk = chunk(ChunkIndex::new(0, 0), None);

        // Act
        let triangles = HeightChunkMesh::from_chunk(&chunk);

        // Assert
        let downward = triangles
            .get_triangles()
            .iter()
            .filter(|triangle| triangle.get_normal().z <= 0.0)
            .count();
        assert_eq!(downward, 0);
    }

    /// One unsurveyed vertex removes the four quads that share it.
    #[test]
    fn height_chunk_mesh_from_chunk_no_data() {
        // Arrange
        let chunk = chunk(ChunkIndex::new(0, 0), Some((17, 17)));

        // Act
        let triangles = HeightChunkMesh::from_chunk(&chunk);

        // Assert
        assert_eq!(triangles.get_triangles().len(), TRIANGLES - 8);
    }

    /// The north-west interior vertex sits on the north-west corner of the chunk.
    #[test]
    fn height_chunk_mesh_from_chunk_position() {
        // Arrange
        let chunk = chunk(ChunkIndex::new(1, 2), None);

        // Act
        let triangles = HeightChunkMesh::from_chunk(&chunk);

        // Assert
        let triangles = triangles.get_triangles();
        let south_west = triangles
            .first()
            .expect("should hold triangles")
            .get_vertices()
            .first()
            .copied();
        let north_west = triangles
            .get(1)
            .expect("should hold triangles")
            .get_vertices()
            .get(2)
            .copied();
        assert_eq!(south_west, Some(Vec3::new(512.0, 1520.0, 71.0)));
        assert_eq!(north_west, Some(Vec3::new(512.0, 1536.0, 36.0)));
    }

    /// Create a chunk with each height set to its own offset.
    ///
    /// - Sets the height at `no_data`, a row and column, to [`NO_DATA`]
    fn chunk(index: ChunkIndex, no_data: Option<(usize, usize)>) -> HeightChunk {
        let across = SPACING.vertices_across();
        let mut heights: Vec<f32> = (0..across.pow(2))
            .map(|offset| f32::from(u16::try_from(offset).expect("offset should fit u16")))
            .collect();
        if let Some((row, column)) = no_data {
            *heights
                .get_mut(row * across + column)
                .expect("offset should be within the chunk") = NO_DATA;
        }
        HeightChunk::new(SPACING, index, heights).expect("heights should be valid")
    }
}
