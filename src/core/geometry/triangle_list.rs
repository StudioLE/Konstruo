use crate::geometry::{Polyline, TriangleStrip};
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;

/// A [`PrimitiveTopology::TriangleList`]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TriangleList {
    /// Vertices
    triangles: Vec<[Vec3; 3]>,
}

impl TriangleList {
    /// Create a [`TriangleList`].
    #[must_use]
    pub fn new(triangles: impl Into<Vec<[Vec3; 3]>>) -> Self {
        Self {
            triangles: triangles.into(),
        }
    }

    /// Create a [`TriangleStrip`] between two parallel polylines.
    ///
    /// If the polylines do not have an equal vertices count then the longest edge will be split.
    ///
    /// The [`Polyline`] are consumed so no cloning takes places.
    ///
    /// TODO: Check the polylines do not intersect with each another
    /// TODO: Check the polylines do not self-intersect
    #[must_use]
    #[allow(clippy::indexing_slicing)]
    pub fn between_polylines(mut polylines: [Polyline; 2]) -> Self {
        Polyline::equalize_vertices_count(&mut polylines);
        let vertices = polylines.map(Polyline::to_vertices);
        let triangles = vertices[0]
            .windows(2)
            .zip(vertices[1].windows(2))
            .flat_map(|(a, b)| [[a[0], b[0], a[1]], [a[1], b[0], b[1]]])
            .collect();
        Self { triangles }
    }

    /// Create a [`PrimitiveTopology::TriangleList`].
    pub fn merge(&mut self, mut other: TriangleList) {
        let triangles = &mut other.triangles;
        self.triangles.append(triangles);
    }

    /// Create a [`PrimitiveTopology::TriangleList`].
    #[must_use]
    pub fn to_mesh(self) -> Mesh {
        let normals: Vec<Vec3> = self
            .triangles
            .iter()
            .map(calculate_normal)
            .flat_map(|x| [x, x, x])
            .collect();
        let vertices = self.triangles.into_flattened();
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    }
}

pub(super) fn calculate_normal(vertices: &[Vec3; 3]) -> Vec3 {
    let u = vertices[1] - vertices[0];
    let v = vertices[2] - vertices[0];
    let normal = u.cross(v);
    normal.normalize()
}
