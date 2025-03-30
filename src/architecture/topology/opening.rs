use crate::architecture::{BuildingMaterials, BuildingMeshes};
use crate::distribution::{Container, Distributable, Distribution, FlexBuilder};
use crate::geometry::{Orientation, Vec6};
use bevy::prelude::*;
use Orientation::*;

const DEPTH: f32 = 0.300;

/// A window or door opening in a building.
#[derive(Clone, Component, Debug, Default)]
pub struct Opening;

/// Dimensions of an [`Opening`].
#[derive(Clone, Debug, Default)]
pub struct OpeningInfo {
    /// Left to right width
    pub width: f32,
    /// Top to bottom height
    pub height: f32,
    /// Minimum margin between openings
    pub margin: Option<Vec6>,
}

/// Factory to create and distribute [`Opening`].
#[derive(Clone, Debug)]
pub struct OpeningFactory {
    /// Side to apply openings.
    pub side: Orientation,
    /// Justify the content.
    pub justify_content: JustifyContent,
    /// Openings to distribute.
    pub openings: Vec<OpeningInfo>,
}

impl OpeningFactory {
    /// Spawn and distribute [`Opening`].
    pub fn spawn(
        &self,
        commands: &mut Commands,
        meshes: &Res<BuildingMeshes>,
        materials: &Res<BuildingMaterials>,
        size: Vec3,
        parent: Entity,
    ) {
        let bounds = match self.side {
            Left | Right => Vec3::new(size.y, 0.0, size.z),
            _ => Vec3::new(size.x, 0.0, size.z),
        };
        let translation = (size - DEPTH) * self.side.to_facing_in() * 0.5;
        let rotation = self.side.get_z_rotation();
        let transform =
            Transform::from_rotation(Quat::from_rotation_z(rotation)).with_translation(translation);
        let bundle = (
            transform,
            Distribution {
                flex: FlexBuilder::new()
                    .with_axis(Vec3::X, Vec3::Z)
                    .with_bounds(bounds)
                    .with_justify_content(self.justify_content)
                    .with_align_items_cross(AlignItems::End)
                    .with_align_items_normal(AlignItems::Center)
                    .build(),
                ..default()
            },
        );
        let distribution = commands.spawn(bundle).set_parent(parent).id();
        for (order, opening) in self.openings.iter().enumerate() {
            let scale = Vec3::new(opening.width, DEPTH, opening.height);
            let distributable = Distributable {
                order,
                size: Some(scale),
                margin: opening.margin,
            };
            let mesh = meshes.cuboid_edges.clone();
            let bundle = (
                Opening,
                Transform::from_scale(scale),
                distributable,
                Mesh3d(mesh),
                MeshMaterial3d(materials.edges.clone()),
            );
            commands.spawn(bundle).set_parent(distribution);
        }
    }

    /// Distribute the openings.
    #[must_use]
    pub fn distribute(&self, bounds: Vec3, right: Vec3, up: Vec3) -> Container {
        let distributables = self
            .openings
            .iter()
            .enumerate()
            .map(|(order, opening)| {
                let scale = Vec3::new(opening.width, DEPTH, opening.height);
                Distributable {
                    order,
                    size: Some(scale),
                    margin: opening.margin,
                }
            })
            .collect();
        let flex = FlexBuilder::new()
            .with_axis(right, up)
            .with_bounds(bounds)
            .with_justify_content(self.justify_content)
            .with_align_items_cross(AlignItems::End)
            .with_align_items_normal(AlignItems::End)
            .build();
        flex.execute(distributables)
    }
}
