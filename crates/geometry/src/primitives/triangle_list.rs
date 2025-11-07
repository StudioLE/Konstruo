use crate::*;
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::mesh::PrimitiveTopology;

/// A [`PrimitiveTopology::TriangleList`]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TriangleList {
    /// Vertices
    triangles: Vec<Triangle>,
}

impl TriangleList {
    /// Create a [`TriangleList`].
    #[must_use]
    pub fn new(triangles: Vec<Triangle>) -> Self {
        Self { triangles }
    }

    /// Create a [`TriangleList`] from rectangles.
    #[must_use]
    pub fn from_rectangles(rectangles: Vec<[Vec3; 4]>) -> Self {
        let triangles = rectangles
            .into_iter()
            .flat_map(Triangle::from_rectangle)
            .collect();
        Self { triangles }
    }

    /// Get the [`Triangle`].
    #[must_use]
    pub fn get_triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }

    /// Create a [`TriangleList`] between two parallel polylines.
    ///
    /// If the polylines do not have an equal vertices count then the longest edge will be split.
    ///
    /// The [`Polyline`] are consumed so no cloning takes places.
    ///
    /// The [`Polyline`] MUST have equal numbers of vertices.
    ///
    /// TODO: Check the polylines do not intersect with each another
    /// TODO: Check the polylines do not self-intersect
    #[must_use]
    #[allow(clippy::indexing_slicing)]
    pub fn between_polylines(left: Polyline, right: Polyline) -> Self {
        let triangles = left
            .to_vertices()
            .windows(2)
            .zip(right.to_vertices().windows(2))
            .flat_map(|(a, b)| {
                [
                    Triangle::new([a[0], a[1], b[0]]),
                    Triangle::new([a[1], b[1], b[0]]),
                ]
            })
            .collect();
        Self { triangles }
    }

    /// Create a [`PrimitiveTopology::TriangleList`].
    pub fn merge(&mut self, mut other: TriangleList) {
        let triangles = &mut other.triangles;
        self.triangles.append(triangles);
    }

    /// Create a [`PrimitiveTopology::TriangleList`].
    ///
    /// Picking requires [`RenderAssetUsages::default()`].
    #[must_use]
    pub fn to_mesh(self) -> Mesh {
        let normals: Vec<Vec3> = self
            .triangles
            .iter()
            .map(Triangle::get_normal)
            .flat_map(|x| [x, x, x])
            .collect();
        let vertices: Vec<Vec3> = self
            .triangles
            .into_iter()
            .flat_map(Triangle::to_vertices)
            .collect();
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    }

    /// Create a [`TriangleList`].
    #[must_use]
    pub fn with_transform(self, transform: Transform) -> Self {
        let triangles = self
            .triangles
            .into_iter()
            .map(|triangle| {
                let vertices = triangle.to_vertices();
                let vertices = [
                    transform.transform_point(vertices[0]),
                    transform.transform_point(vertices[1]),
                    transform.transform_point(vertices[2]),
                ];
                Triangle::new(vertices)
            })
            .collect();
        Self { triangles }
    }
}
