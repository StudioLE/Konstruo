use crate::architecture::*;
use crate::distribution::{Distribution, FlexBuilder};
use bevy::prelude::*;

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
            ModularBuilding,
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
    /// Spawn the full hierarchy of [`BuildingPlot`] > [`BuildingModuleStack`] > [`BuildingModule`].
    pub fn spawn(&mut self, building: ModularBuildingInfo) -> Entity {
        let bundle = ModularBuilding::bundle();
        let plot = self.commands.spawn(bundle).id();
        for (index, stack) in building.stacks.into_iter().enumerate() {
            self.spawn_stack(stack, index, plot);
        }
        plot
    }
}
