use bevy::color::palettes::*;
use bevy::prelude::*;
use bevy::render::render_resource::Face;

/// A graphical representation of the terrain as a faceted mesh.
#[derive(Component)]
pub struct Terrain;

impl Terrain {
    /// Create a bundle for [`Terrain`].
    ///
    /// - Holds an identity [`Transform`], the vertices being in world space at
    ///   true elevation
    pub(crate) fn bundle(mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> impl Bundle {
        (
            Terrain,
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::IDENTITY,
        )
    }

    /// Create the [`Terrain`] material.
    ///
    /// - Reads as stone against the green ground plane
    /// - Culls back faces, the surface being closed and facing up
    pub(crate) fn material() -> StandardMaterial {
        StandardMaterial {
            base_color: tailwind::STONE_600.into(),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Back),
            ..default()
        }
    }
}
