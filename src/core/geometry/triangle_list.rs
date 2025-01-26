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

    #[must_use]
    pub fn triangular_prism() -> Self {
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
        Self::new([
            front,
            back,
            left_bottom,
            left_top,
            right_bottom,
            right_top,
            front_bottom,
            back_bottom,
        ])
    }

    /// Create a [`PrimitiveTopology::TriangleList`].
    #[must_use]
    pub fn to_mesh(self) -> Mesh {
        let vertices = self.triangles.into_flattened();
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    }
}
