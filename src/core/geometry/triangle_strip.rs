use crate::geometry::{Polyline, Triangle};
use bevy::asset::RenderAssetUsages;
use bevy::math::Vec3;
use bevy::prelude::Mesh;
use bevy::render::mesh::PrimitiveTopology;

/// A [`PrimitiveTopology::TriangleStrip`]
#[derive(Clone, Debug, PartialEq)]
pub struct TriangleStrip {
    /// Pairs of vertices
    pairs: Vec<[Vec3; 2]>,
}

impl TriangleStrip {
    /// Create a [`TriangleStrip`] between two parallel polylines.
    ///
    /// If the polylines do not have an equal vertices count then the longest edge will be split.
    ///
    /// The [`Polyline`] are consumed so no cloning takes places.
    ///
    /// TODO: Check the polylines do not intersect with each another
    /// TODO: Check the polylines do not self-intersect
    #[must_use]
    pub fn new(mut polylines: [Polyline; 2]) -> Self {
        Polyline::equalize_vertices_count(&mut polylines);
        let vertices = polylines.map(Polyline::to_vertices);
        let pairs = vertices[0]
            .iter()
            .zip(vertices[1].iter())
            .map(|(&a, &b)| [a, b])
            .collect();
        Self { pairs }
    }

    /// Create a [`PrimitiveTopology::TriangleStrip`] mesh.
    ///
    /// Picking requires [`RenderAssetUsages::default()`].
    #[must_use]
    #[allow(clippy::indexing_slicing)]
    pub fn to_mesh(self) -> Mesh {
        let vertices = self.pairs.into_flattened();
        let normal = if vertices.len() >= 3 {
            Triangle::new([vertices[0], vertices[1], vertices[2]]).get_normal()
        } else {
            Vec3::Z
        };
        let normals = vec![normal; vertices.len()];
        Mesh::new(
            PrimitiveTopology::TriangleStrip,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    }
}
