use bevy::asset::RenderAssetUsages;
use bevy::math::Vec3;
use bevy::prelude::Mesh;
use bevy::render::mesh::PrimitiveTopology;
use std::fmt::{Display, Formatter};
use TriangleStripError::*;

/// A [`PrimitiveTopology::TriangleStrip`]
#[derive(Clone, Debug, PartialEq)]
pub struct TriangleStip {
    /// Pairs of vertices
    pairs: Vec<[Vec3; 2]>,
}

#[derive(Debug, PartialEq)]
pub enum TriangleStripError {
    InEqual { left: usize, right: usize },
}

impl TriangleStip {
    /// Create a [`TriangleStrip`] between two parallel polylines.
    ///
    /// The polylines must have equal number of vertices.
    pub fn between_polylines(polylines: &[Vec<Vec3>; 2]) -> Result<Self, TriangleStripError> {
        let left = polylines[0].len();
        let right = polylines[1].len();
        if left != right {
            return Err(InEqual { left, right });
        }
        let pairs = polylines[0]
            .iter()
            .zip(polylines[1].iter())
            .map(|(&a, &b)| [a, b])
            .collect();
        Ok(Self { pairs })
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

impl Display for TriangleStripError {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            InEqual { left, right } => {
                format!("Polyline lengths were not equal: {left} != {right}")
            }
        };
        output.fmt(formatter)
    }
}
