use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

/// Create a [`PrimitiveTopology::LineList`] from vertices.
#[must_use]
pub fn create_line_list(lines: &[[Vec3; 2]]) -> Mesh {
    let vertices: Vec<Vec3> = lines.iter().flatten().copied().collect();
    Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
}

/// Create a [`PrimitiveTopology::TriangleList`] from vertices.
#[must_use]
#[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
pub fn create_triangle_list(triangles: &[[Vec3; 3]]) -> Mesh {
    let vertices: Vec<Vec3> = triangles.iter().flatten().copied().collect();
    // let indices: Vec<u32> = (0..vertices.len() as u32).collect();
    let normals: Vec<Vec3> = vertices.clone();
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    // .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    // .with_inserted_indices(Indices::U32(indices))
}

#[must_use]
pub fn create_triangular_prism() -> [[Vec3; 3]; 8] {
    let a1 = Vec3::new(-0.5, 0.5, -0.5);
    let b1 = Vec3::new(-0.5, -0.5, -0.5);
    let c1 = Vec3::new(-0.5, 0.0, 0.5);
    let a2 = Vec3::new(0.5, 0.5, -0.5);
    let b2 = Vec3::new(0.5, -0.5, -0.5);
    let c2 = Vec3::new(0.5, 0.0, 0.5);
    let front = [a1, b1, c1];
    let back = [a2, c2, b2];
    let left_bottom = [a1, c2, a2];
    let left_top = [a1, c1, c2];
    let right_bottom = [b1, b2, c2];
    let right_top = [b1, c2, c1];
    let front_bottom = [a1, a2, b1];
    let back_bottom = [b1, a2, b2];
    [
        front,
        back,
        left_bottom,
        left_top,
        right_bottom,
        right_top,
        front_bottom,
        back_bottom,
    ]
}
