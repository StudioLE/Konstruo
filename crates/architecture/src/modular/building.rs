use crate::*;
use bevy::prelude::*;
use konstruo_distribution::{Distribution, FlexBuilder};
use konstruo_ui::EntityState;

/// A modular building formed of [`BuildingModuleStack`] and [`BuildingModule`].
#[derive(Component, Default)]
#[require(InheritedVisibility, Transform)]
pub struct ModularBuilding;

/// A definition to horizontally array vertical stacks of [`BuildingModule`].
pub struct ModularBuildingInfo {
    pub stacks: Vec<BuildingModuleStackInfo>,
}

impl ModularBuilding {
    /// Create a bundle for [`ModularBuildingInfo`].
    fn bundle() -> impl Bundle {
        (
            Name::new("Modular Building"),
            ModularBuilding,
            EntityState::Default,
            Distribution {
                flex: FlexBuilder::new()
                    .with_axis(Vec3::X, Vec3::Y)
                    .with_align_items_cross(AlignItems::FlexEnd)
                    .build(),
                translate_to_ground: true,
                ..default()
            },
        )
    }
}

impl ModularBuildingFactory<'_> {
    /// Spawn the full hierarchy of [`ModularBuilding`] > [`BuildingModuleStack`] > [`BuildingModule`].
    pub fn spawn(&mut self, building: ModularBuildingInfo) -> Entity {
        let bundle = ModularBuilding::bundle();
        let plot = self.commands.spawn(bundle).id();
        for (index, stack) in building.stacks.into_iter().enumerate() {
            self.spawn_stack(stack, index, plot);
        }
        plot
    }
}
