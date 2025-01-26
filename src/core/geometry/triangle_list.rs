use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;

/// A [`PrimitiveTopology::TriangleList`]
#[derive(Clone, Debug, PartialEq)]
pub struct TriangleList {
    /// Vertices
    triangles: Vec<[Vec3; 3]>,
}

impl TriangleList {
    /// Create a [`TriangleList`].
    #[must_use]
    pub fn new(triangles: impl Into<Vec<[Vec3; 3]>>) -> Self {
        Self {
            triangles: triangles.into(),
        }
    }

    /// Create a [`PrimitiveTopology::TriangleList`].
    #[must_use]
    pub fn to_mesh(self) -> Mesh {
        let normals: Vec<Vec3> = self
            .triangles
            .iter()
            .map(calculate_normal)
            .flat_map(|x| [x, x, x])
            .collect();
        let vertices = self.triangles.into_flattened();
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    }
}

fn calculate_normal(vertices: &[Vec3; 3]) -> Vec3 {
    let u = vertices[1] - vertices[0];
    let v = vertices[2] - vertices[0];
    let normal = u.cross(v);
    normal.normalize()
}
