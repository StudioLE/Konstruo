use crate::architecture::*;
use crate::distribution::{Distribution, FlexBuilder};
use bevy::prelude::*;

/// A modular building formed of [`BuildingModuleStack`] and [`BuildingModule`].
#[derive(Component, Default)]
#[require(InheritedVisibility, Transform)]
pub struct ModularBuilding;

/// Factory to horizontally array vertical stacks of [`BuildingModule`].
pub struct ModularBuildingFactory {
    pub stacks: Vec<BuildingModuleStackFactory>,
}

impl ModularBuilding {
    /// Create a bundle for [`ModularBuildingFactory`].
    fn bundle() -> (ModularBuilding, Distribution) {
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

impl ModularBuildingFactory {
    /// Spawn the full hierarchy of [`BuildingPlot`] > [`BuildingModuleStack`] > [`BuildingModule`].
    pub fn spawn(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        building_meshes: &Res<BuildingMeshes>,
        materials: &Res<BuildingMaterials>,
    ) -> Entity {
        let bundle = ModularBuilding::bundle();
        let plot = commands.spawn(bundle).id();
        for (index, stack) in self.stacks.into_iter().enumerate() {
            stack.spawn(commands, meshes, building_meshes, materials, index, plot);
        }
        plot
    }
}
