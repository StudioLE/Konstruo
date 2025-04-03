use super::*;
use crate::distribution::{Distributable, Distribution, FlexBuilder};
use bevy::prelude::*;

/// A vertical stack of [`BuildingModuleInfo`].
#[derive(Clone, Component, Debug)]
#[require(InheritedVisibility, Transform)]
pub struct BuildingModuleStack;

/// A defintion to create a [`BuildingModuleStack`].
pub struct BuildingModuleStackInfo {
    pub modules: Vec<BuildingModuleInfo>,
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

impl ModularBuildingFactory<'_> {
    /// Spawn the hierarchy of [`BuildingModuleStack`] > [`BuildingModuleInfo`].
    pub(super) fn spawn_stack(
        &mut self,
        stack: BuildingModuleStackInfo,
        index: usize,
        plot: Entity,
    ) -> Entity {
        let bundle = BuildingModuleStack::bundle(index);
        let stack_entity = self.commands.spawn(bundle).set_parent(plot).id();
        for (order, module) in stack.modules.into_iter().enumerate() {
            self.spawn_module(&module, order, stack_entity);
        }
        stack_entity
    }
}
