use bevy::prelude::*;
use konstruo_distribution::{Container, Distributable, FlexBuilder};
use konstruo_geometry::{Orientation, Vec6};

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

/// A set of [`Opening`] and the logic to distribute them.
#[derive(Clone, Debug)]
pub struct OpeningDistribution {
    /// Side to apply openings.
    pub side: Orientation,
    /// Justify the content.
    pub justify_content: JustifyContent,
    /// Openings to distribute.
    pub openings: Vec<OpeningInfo>,
}

impl OpeningDistribution {
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
