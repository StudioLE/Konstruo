use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;

/// Create a [`PrimitiveTopology::LineStrip`] from vertices.
#[must_use]
pub fn create_linestrip(vertices: Vec<Vec3>) -> Mesh {
    Mesh::new(
        PrimitiveTopology::LineStrip,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
}

/// Create a [`PrimitiveTopology::TriangleStrip`] from vertices.
#[must_use]
pub fn create_triangle_strip(vertices: Vec<Vec3>) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleStrip,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
}
