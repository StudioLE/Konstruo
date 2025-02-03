use crate::architecture::*;
use crate::distribution::{Container, FlexBuilder, SourceItem};
use bevy::prelude::*;

/// Factory to produce vertically stacked modules
#[derive(Debug)]
pub struct ModularBuildingFactory;

#[derive(Debug)]
pub enum ModularBuildingFactoryError {
    InEqualLengths { lengths: Vec<f32> },
}

impl ModularBuildingFactory {
    /// Factory method to spawn a [`BuildingPlot`] containing [`BuildingModule`]
    pub fn spawn(
        commands: &mut Commands,
        meshes: &Res<BuildingMeshes>,
        materials: &Res<BuildingMaterials>,
        stacks: Vec<BuildingModuleStack>,
    ) -> Result<(), ModularBuildingFactoryError> {
        let stacked_modules: Vec<_> = stacks
            .iter()
            .enumerate()
            .map(|(index, stack)| create_stacked_modules(index, stack))
            .collect();
        let sizes = stacked_modules
            .iter()
            .map(|(container, _)| SourceItem {
                size: container.size,
                margin: Vec3::ZERO,
            })
            .collect();
        let container = FlexBuilder::new()
            .with_axis(Vec3::X, Vec3::Y)
            .with_items(sizes)
            .execute();
        let bundle = (
            Transform::from_translation(Vec3::new(0.0, 0.0, container.size.z * 0.5)),
            BuildingPlot {
                width: container.size.x,
                length: container.size.y,
                height: container.size.z,
            },
        );
        let plot = commands.spawn(bundle).id();
        for ((stack_item, stack), (stack_container, modules)) in
            container.items.into_iter().zip(stacks).zip(stacked_modules)
        {
            let bundle = (stack, Transform::from_translation(stack_item.translation));
            let stack = commands.spawn(bundle).set_parent(plot).id();
            spawn_stacked_modules(commands, meshes, materials, stack_container, modules, stack);
        }
        Ok(())
    }
}

#[allow(clippy::as_conversions, clippy::cast_possible_wrap)]
fn create_stacked_modules(
    index: usize,
    stack: &BuildingModuleStack,
) -> (Container, Vec<BuildingModule>) {
    let levels = 0..stack.levels;
    let mut modules: Vec<BuildingModule> = levels
        .map(|level| BuildingModule {
            index,
            level: level as isize,
            height: stack.level_height,
            roof: None,
            ..stack.definition
        })
        .collect();
    if let Some(roof) = stack.roof_style.clone() {
        modules.push(BuildingModule {
            index,
            level: stack.levels as isize,
            height: stack.level_height, // TODO: roof height
            roof: Some(roof),
            ..stack.definition
        });
    };
    let items = modules
        .iter()
        .map(|module| {
            let offset = if module.front_offset < module.back_offset {
                module.front_offset
            } else {
                module.back_offset
            };
            SourceItem {
                size: Vec3::new(module.width, module.length, module.height),
                margin: Vec3::new(0.0, offset, 0.0),
            }
        })
        .collect();
    let container = FlexBuilder::new()
        .with_axis(Vec3::Z, Vec3::X)
        .with_items(items)
        .execute();
    (container, modules)
}

fn spawn_stacked_modules(
    commands: &mut Commands,
    meshes: &Res<BuildingMeshes>,
    materials: &Res<BuildingMaterials>,
    container: Container,
    modules: Vec<BuildingModule>,
    parent: Entity,
) {
    let items = modules.into_iter().zip(container.items);
    for (module, item) in items {
        let transform = Transform::from_translation(item.translation).with_scale(Vec3::new(
            module.width,
            module.length,
            module.height,
        ));
        let mesh = if module.roof.is_some() {
            meshes.pitched_module.clone()
        } else {
            meshes.cuboid_module.clone()
        };
        let bundle = (
            transform,
            Mesh3d(mesh),
            MeshMaterial3d(materials.module.clone()),
            module,
        );
        commands.spawn(bundle).set_parent(parent);
    }
}
