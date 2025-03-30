use super::*;
use crate::distribution::{Distributable, Distribution, FlexBuilder};
use bevy::prelude::*;

/// A vertical stack of [`BuildingModuleFactory`].
#[derive(Clone, Component, Debug)]
#[require(InheritedVisibility, Transform)]
pub struct BuildingModuleStack;

/// A factory to create a [`BuildingModuleStack`].
pub struct BuildingModuleStackFactory {
    pub modules: Vec<BuildingModuleFactory>,
}

impl BuildingModuleStack {
    /// Create a bundle for [`BuildingModuleStack`].
    pub(super) fn bundle(index: usize) -> (BuildingModuleStack, Distributable, Distribution) {
        (
            BuildingModuleStack,
            Distributable {
                order: index,
                ..default()
            },
            Distribution {
                flex: FlexBuilder::new().with_axis(Vec3::Z, Vec3::X).build(),
                translate_to_ground: true,
                ..default()
            },
        )
    }
}

impl BuildingModuleStackFactory {
    /// Spawn the hierarchy of [`BuildingModuleStack`] > [`BuildingModuleFactory`].
    pub(super) fn spawn(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        building_meshes: &Res<BuildingMeshes>,
        materials: &Res<BuildingMaterials>,
        index: usize,
        plot: Entity,
    ) -> Entity {
        let bundle = BuildingModuleStack::bundle(index);
        let stack = commands.spawn(bundle).set_parent(plot).id();
        for (order, module) in self.modules.into_iter().enumerate() {
            module.spawn(commands, meshes, building_meshes, materials, order, stack);
        }
        stack
    }
}
