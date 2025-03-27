use crate::architecture::*;
use crate::distribution::{Distributable, Distribution, FlexBuilder};
use bevy::prelude::*;

/// Factory to produce vertically stacked modules
#[derive(Clone, Component, Debug)]
#[require(Distribution(spawn_distribution), BuildingPlot)]
pub struct ModularBuildingFactory {
    pub stacks: Vec<BuildingModuleStack>,
}

impl ModularBuildingFactory {
    /// Spawn the children for [`ModularBuildingFactory`].
    pub fn spawn_children(
        self,
        commands: &mut Commands,
        meshes: &Res<BuildingMeshes>,
        materials: &Res<BuildingMaterials>,
        entity: Entity,
    ) {
        for (index, stack) in self.stacks.into_iter().enumerate() {
            let modules = stack.modules.clone();
            let parent = spawn_stack(commands, index, stack, entity);
            for (order, module) in modules.into_iter().enumerate() {
                spawn_module(commands, meshes, materials, order, module, parent);
            }
        }
    }
}

fn spawn_distribution() -> Distribution {
    Distribution {
        flex: FlexBuilder::new()
            .with_axis(Vec3::X, Vec3::Y)
            .with_align_items_cross(AlignItems::FlexEnd)
            .build(),
        translate_to_ground: true,
        ..default()
    }
}

fn spawn_stack(
    commands: &mut Commands,
    index: usize,
    stack: BuildingModuleStack,
    plot: Entity,
) -> Entity {
    let bundle = (
        Distributable {
            order: index,
            ..default()
        },
        Distribution {
            flex: FlexBuilder::new().with_axis(Vec3::Z, Vec3::X).build(),
            translate_to_ground: true,
            ..default()
        },
        stack,
    );
    commands.spawn(bundle).set_parent(plot).id()
}

fn spawn_module(
    commands: &mut Commands,
    meshes: &Res<BuildingMeshes>,
    materials: &Res<BuildingMaterials>,
    order: usize,
    module: BuildingModule,
    parent: Entity,
) {
    let distributable = Distributable {
        order,
        size: Some(Vec3::new(module.width, module.length, module.height)),
        margin: module.margin,
    };
    let mesh = match module.roof {
        None => meshes.cuboid.clone(),
        Some(RoofStyle::PitchLeftToRight) => meshes.pitched_left_right.clone(),
        Some(RoofStyle::PitchFrontToBack) => meshes.pitched_front_back.clone(),
    };
    let bundle = (
        Transform::from_scale(Vec3::new(module.width, module.length, module.height)),
        distributable,
        Mesh3d(mesh),
        MeshMaterial3d(materials.module.clone()),
        module,
    );
    commands.spawn(bundle).set_parent(parent);
}
