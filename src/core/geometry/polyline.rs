use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use std::cmp::Ordering;

/// A polyline or [`PrimitiveTopology::LineStrip`]
#[derive(Clone, Debug, PartialEq)]
pub struct Polyline {
    /// Vertices
    vertices: Vec<Vec3>,
}

impl From<Vec<Vec3>> for Polyline {
    fn from(vertices: Vec<Vec3>) -> Self {
        Self { vertices }
    }
}

impl Polyline {
    /// Create a [`Polyline`].
    #[must_use]
    pub fn new(vertices: impl Into<Vec<Vec3>>) -> Self {
        Self::from(vertices.into())
    }

    /// Get the vertices of the [`Polyline`].
    ///
    /// The [`Polyline`] is consumed so no cloning takes place.
    #[must_use]
    pub fn to_vertices(self) -> Vec<Vec3> {
        self.vertices
    }

    /// Get the vertices of the [`Polyline`].
    ///
    /// The vertices are borrowed.
    #[must_use]
    pub fn get_vertices(&self) -> &Vec<Vec3> {
        &self.vertices
    }

    /// Create a [`PrimitiveTopology::LineStrip`].
    ///
    /// Picking requires [`RenderAssetUsages::default()`].
    #[must_use]
    pub fn to_mesh(self) -> Mesh {
        Mesh::new(PrimitiveTopology::LineStrip, RenderAssetUsages::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices)
    }

    /// Equalize the number of vertices by splitting the longest edge of the shortest [`Polyline`].
    #[allow(
        clippy::as_conversions,
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss
    )]
    pub(super) fn equalize_vertices_count(polylines: &mut [Polyline; 2]) {
        let difference =
            polylines[0].vertices.len() as isize - polylines[1].vertices.len() as isize;
        match difference.cmp(&0) {
            Ordering::Less => {
                add_vertices(&mut polylines[0], difference.unsigned_abs());
            }
            Ordering::Greater => {
                add_vertices(&mut polylines[1], difference as usize);
            }
            Ordering::Equal => {}
        }
    }
}

#[allow(clippy::indexing_slicing)]
/// Add vertices to a [`Polyline`] by splitting the longest edge.
fn add_vertices(polyline: &mut Polyline, count: usize) {
    for _ in 0..count {
        let starts = polyline.vertices.iter().take(polyline.vertices.len() - 1);
        let ends = polyline.vertices.iter().skip(1);
        let distances = starts
            .zip(ends)
            .map(|(start, end)| start.distance(*end))
            .collect::<Vec<f32>>();
        let max_distance = distances
            .iter()
            .max_by(|a, b| a.partial_cmp(b).expect("Floats are comparable"))
            .expect("Should be at least one distance");
        #[allow(clippy::float_cmp)]
        let max_index = distances
            .iter()
            .position(|&x| x == *max_distance)
            .expect("Item should exist");
        let mid_point = (polyline.vertices[max_index] + polyline.vertices[max_index + 1]) / 2.0;
        polyline.vertices.insert(max_index + 1, mid_point);
    }
}
