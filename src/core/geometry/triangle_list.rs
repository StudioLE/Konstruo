use crate::geometry::{Polyline, Triangle};
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;

/// A [`PrimitiveTopology::TriangleList`]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TriangleList {
    /// Vertices
    triangles: Vec<Triangle>,
}

impl From<Vec<Triangle>> for TriangleList {
    fn from(triangles: Vec<Triangle>) -> Self {
        Self { triangles }
    }
}

impl TriangleList {
    /// Create a [`TriangleList`].
    #[must_use]
    pub fn new(triangles: impl Into<Vec<Triangle>>) -> Self {
        Self {
            triangles: triangles.into(),
        }
    }

    /// Create a [`TriangleList`] between two parallel polylines.
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
            .flat_map(|(a, b)| {
                [
                    Triangle::new([a[0], a[1], b[0]]),
                    Triangle::new([a[1], b[1], b[0]]),
                ]
            })
            .collect();
        Self { triangles }
    }

    /// Create a 3D [`TriangleList`] between two parallel polylines.
    ///
    /// If the polylines do not have an equal vertices count then the longest edge will be split.
    ///
    /// The [`Polyline`] are consumed so minimal cloning takes places.
    ///
    /// TODO: Check the polylines do not intersect with each another
    /// TODO: Check the polylines do not self-intersect
    #[must_use]
    #[allow(clippy::indexing_slicing)]
    pub fn between_polylines_3d(mut polylines: [Polyline; 2], depth: f32) -> Self {
        Polyline::equalize_vertices_count(&mut polylines);
        let [bottom_0, bottom_1] = polylines.map(Polyline::to_vertices);
        let top_0: Vec<Vec3> = bottom_0
            .iter()
            .map(|vertex| vertex.with_z(vertex.z + depth))
            .collect();
        let top_1: Vec<Vec3> = bottom_1
            .iter()
            .map(|vertex| vertex.with_z(vertex.z + depth))
            .collect();
        let start_top = vec![
            *top_0.first().expect("first should exist"),
            *top_1.first().expect("first should exist"),
        ];
        let start_bottom = vec![
            *bottom_0.first().expect("first should exist"),
            *bottom_1.first().expect("first should exist"),
        ];
        let end_top = vec![
            *top_0.last().expect("last should exist"),
            *top_1.last().expect("last should exist"),
        ];
        let end_bottom = vec![
            *bottom_0.last().expect("first should exist"),
            *bottom_1.last().expect("last should exist"),
        ];
        let mut triangles =
            TriangleList::between_polylines([top_0.clone().into(), top_1.clone().into()]);
        triangles.merge(TriangleList::between_polylines([
            bottom_1.clone().into(),
            bottom_0.clone().into(),
        ]));
        triangles.merge(TriangleList::between_polylines([
            bottom_0.into(),
            top_0.into(),
        ]));
        triangles.merge(TriangleList::between_polylines([
            top_1.into(),
            bottom_1.into(),
        ]));
        triangles.merge(TriangleList::between_polylines([
            start_top.into(),
            start_bottom.into(),
        ]));
        triangles.merge(TriangleList::between_polylines([
            end_bottom.into(),
            end_top.into(),
        ]));
        triangles
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
                Triangle::from(vertices)
            })
            .collect();
        Self { triangles }
    }
}
