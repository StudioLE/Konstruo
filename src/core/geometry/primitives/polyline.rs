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
    pub fn new(vertices: Vec<Vec3>) -> Self {
        Self { vertices }
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

    /// Apply a [`Transform`] to the vertices of the [`Polyline`].
    #[must_use]
    pub fn get_transformed(&self, transform: Transform) -> Polyline {
        let vertices = self
            .vertices
            .iter()
            .copied()
            .map(|vertex| transform.transform_point(vertex))
            .collect();
        Self { vertices }
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
    pub(crate) fn equalize_vertices_count(left: &mut Polyline, right: &mut Polyline) {
        let difference = left.vertices.len() as isize - right.vertices.len() as isize;
        match difference.cmp(&0) {
            Ordering::Less => {
                add_vertices(left, difference.unsigned_abs());
            }
            Ordering::Greater => {
                add_vertices(right, difference as usize);
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
