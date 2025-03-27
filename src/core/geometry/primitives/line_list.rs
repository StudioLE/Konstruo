use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;

/// A [`PrimitiveTopology::LineList`]
#[derive(Clone, Debug, PartialEq)]
pub struct LineList {
    /// Lines as pairs of vertices
    lines: Vec<[Vec3; 2]>,
}

impl LineList {
    /// Create a [`LineList`].
    #[must_use]
    pub fn new(lines: Vec<[Vec3; 2]>) -> Self {
        Self { lines }
    }

    /// Create a [`PrimitiveTopology::LineList`].
    ///
    /// Picking requires [`RenderAssetUsages::default()`].
    #[must_use]
    pub fn to_mesh(self) -> Mesh {
        let vertices = self.lines.into_flattened();
        Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    }
}
