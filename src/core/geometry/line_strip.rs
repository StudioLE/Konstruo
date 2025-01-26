use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;

/// A polyline or [`PrimitiveTopology::LineStrip`]
#[derive(Clone, Debug, PartialEq)]
pub struct LineStrip {
    /// Vertices
    vertices: Vec<Vec3>,
}

impl LineStrip {
    /// Create a [`LineStrip`].
    #[must_use]
    pub fn new(vertices: impl Into<Vec<Vec3>>) -> Self {
        Self {
            vertices: vertices.into(),
        }
    }

    /// Create a [`PrimitiveTopology::LineStrip`].
    #[must_use]
    pub fn to_mesh(self) -> Mesh {
        Mesh::new(
            PrimitiveTopology::LineStrip,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices)
    }
}
