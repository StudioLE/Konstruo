use crate::architecture::*;
use crate::distribution::{Distributable, Distribution, FlexBuilder};
use bevy::prelude::*;

/// Factory to produce vertically stacked modules
#[derive(Component, Debug)]
pub struct ModularBuildingFactory {
    pub stacks: Vec<BuildingModuleStack>,
}

impl ModularBuildingFactory {
    /// System to spawn a [`BuildingPlot`] containing [`BuildingModule`]
    pub fn added_system(
        mut commands: Commands,
        meshes: Res<BuildingMeshes>,
        materials: Res<BuildingMaterials>,
        query: Query<(Entity, &ModularBuildingFactory), Added<ModularBuildingFactory>>,
    ) {
        for (entity, factory) in query.iter() {
            insert_components(&mut commands, entity);
            for (index, stack) in factory.stacks.iter().enumerate() {
                let modules = create_stacked_modules(index, stack);
                let parent = spawn_stack(&mut commands, index, stack.clone(), entity);
                for (order, module) in modules.into_iter().enumerate() {
                    spawn_module(&mut commands, &meshes, &materials, order, module, parent);
                }
            }
        }
    }
}

#[allow(clippy::as_conversions, clippy::cast_possible_wrap)]
fn create_stacked_modules(index: usize, stack: &BuildingModuleStack) -> Vec<BuildingModule> {
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
            height: stack.roof_height,
            roof: Some(roof),
            ..stack.definition
        });
    };
    modules
}

fn insert_components(commands: &mut Commands, entity: Entity) {
    let bundle = (
        Distribution {
            flex: FlexBuilder::new()
                .with_axis(Vec3::X, Vec3::Y)
                .with_align_items_cross(AlignItems::FlexEnd)
                .build(),
            translate_to_ground: true,
            ..default()
        },
        BuildingPlot,
    );
    commands.entity(entity).insert(bundle);
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
        None => meshes.cuboid_module.clone(),
        Some(RoofStyle::PitchLeftToRight) => meshes.pitched_left_right_module.clone(),
        Some(RoofStyle::PitchFrontToBack) => meshes.pitched_front_back_module.clone(),
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
