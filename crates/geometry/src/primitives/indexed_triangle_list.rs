use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

/// An indexed [`PrimitiveTopology::TriangleList`].
///
/// - Shares vertices between triangles, suiting planar surfaces where every vertex has one normal
/// - See [`crate::TriangleList`] for faceted solids where a shared vertex needs several normals
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IndexedTriangleList {
    /// The vertex positions.
    positions: Vec<Vec3>,
    /// The vertex indices.
    ///
    /// - Holds three indices per triangle
    indices: Vec<u32>,
}

impl IndexedTriangleList {
    /// Create an [`IndexedTriangleList`].
    #[must_use]
    pub fn new(positions: Vec<Vec3>, indices: Vec<u32>) -> Self {
        Self { positions, indices }
    }

    /// The vertex positions.
    #[must_use]
    pub fn get_positions(&self) -> &Vec<Vec3> {
        &self.positions
    }

    /// The vertex indices, three per triangle.
    ///
    /// - Indexes into [`IndexedTriangleList::get_positions`]
    #[must_use]
    pub fn get_indices(&self) -> &Vec<u32> {
        &self.indices
    }

    /// The triangle count.
    #[must_use]
    pub fn get_triangle_count(&self) -> usize {
        self.indices.chunks_exact(3).count()
    }

    /// Is the [`IndexedTriangleList`] free of triangles?
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }

    /// Create a [`Mesh`] from the [`IndexedTriangleList`].
    ///
    /// - Builds a [`PrimitiveTopology::TriangleList`], keeping the shared vertices indexed
    /// - Applies the same `normal` to every vertex
    /// - Sets [`RenderAssetUsages::default()`], which picking requires
    #[must_use]
    pub fn to_mesh(self, normal: Vec3) -> Mesh {
        let normals = vec![normal; self.positions.len()];
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_indices(Indices::U32(self.indices))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexed_triangle_list_to_mesh() {
        // Arrange
        let positions = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ];
        let indices = vec![0, 1, 2, 0, 2, 3];
        let list = IndexedTriangleList::new(positions, indices);

        // Act
        let mesh = list.to_mesh(Vec3::Z);

        // Assert
        assert_eq!(mesh.count_vertices(), 4);
        let normals = mesh
            .attribute(Mesh::ATTRIBUTE_NORMAL)
            .expect("normals should be inserted");
        assert_eq!(normals.len(), 4);
        let indices = mesh.indices().expect("indices should be inserted");
        assert_eq!(indices.len(), 6);
    }
}
