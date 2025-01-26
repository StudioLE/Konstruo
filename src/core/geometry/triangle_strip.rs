use bevy::asset::RenderAssetUsages;
use bevy::math::Vec3;
use bevy::prelude::Mesh;
use bevy::render::mesh::PrimitiveTopology;
use std::cmp::Ordering;

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
    #[must_use]
    #[allow(
        clippy::as_conversions,
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss
    )]
    pub fn between_polylines(polylines: &[Vec<Vec3>; 2]) -> Self {
        let mut polylines = [polylines[0].clone(), polylines[1].clone()];
        let difference = polylines[0].len() as isize - polylines[1].len() as isize;
        match difference.cmp(&0) {
            Ordering::Less => {
                add_vertices_by_splitting_longest_edge(
                    &mut polylines[0],
                    difference.unsigned_abs(),
                );
            }
            Ordering::Greater => {
                add_vertices_by_splitting_longest_edge(&mut polylines[1], difference as usize);
            }
            Ordering::Equal => {}
        }
        let pairs = polylines[0]
            .iter()
            .zip(polylines[1].iter())
            .map(|(&a, &b)| [a, b])
            .collect();
        Self { pairs }
    }

    /// Create a [`PrimitiveTopology::TriangleStrip`] mesh.
    #[must_use]
    pub fn to_mesh(self) -> Mesh {
        let vertices = self.pairs.into_flattened();
        Mesh::new(
            PrimitiveTopology::TriangleStrip,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    }
}

#[allow(clippy::indexing_slicing)]
/// Add vertices to a polyline by splitting the longest edge.
fn add_vertices_by_splitting_longest_edge(polyline: &mut Vec<Vec3>, count: usize) {
    for _ in 0..count {
        let starts = polyline.iter().take(polyline.len() - 1);
        let ends = polyline.iter().skip(1);
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
        let mid_point = (polyline[max_index] + polyline[max_index + 1]) / 2.0;
        polyline.insert(max_index + 1, mid_point);
    }
}
