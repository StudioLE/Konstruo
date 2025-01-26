use crate::architecture::*;
use bevy::prelude::*;
use ModularBuildingFactoryError::*;

/// Factory to produce vertically stacked modules
#[derive(Debug)]
pub struct ModularBuildingFactory;

#[derive(Debug)]
pub struct BuildingModuleStack {
    pub definition: BuildingModule,
    pub levels: usize,
    pub level_height: f32,
}

#[derive(Debug)]
pub enum ModularBuildingFactoryError {
    InEqualLengths { lengths: Vec<f32> },
}

impl ModularBuildingFactory {
    /// Factory method to spawn a [`BuildingPlot`] containing [`BuildingModule`]
    #[allow(
        clippy::as_conversions,
        clippy::cast_precision_loss,
        clippy::cast_possible_wrap
    )]
    pub fn spawn(
        commands: &mut Commands,
        meshes: &Res<BuildingMeshes>,
        materials: &Res<BuildingMaterials>,
        stacks: Vec<BuildingModuleStack>,
    ) -> Result<(), ModularBuildingFactoryError> {
        let plot = match create_building(&stacks) {
            Ok(plot) => plot,
            Err(value) => return value,
        };
        let mut front_left = Vec3::new(plot.length * -0.5, plot.width * -0.5, plot.height * -0.5);
        let bundle = (
            Transform::from_translation(Vec3::new(0.0, 0.0, plot.height * 0.5)),
            plot,
        );
        let parent = commands.spawn(bundle).id();
        for (index, stack) in stacks.iter().enumerate() {
            for level in 0..stack.levels {
                if level == stack.levels {
                    // TODO: Create a roof
                    debug!("TODO: Create a roof");
                } else {
                    let module = BuildingModule {
                        index,
                        level: level as isize,
                        height: stack.level_height,
                        ..stack.definition
                    };
                    let translation = Vec3::new(
                        module.front_offset + module.length * 0.5,
                        module.width * 0.5,
                        module.height * (level as f32 + 0.5),
                    );
                    let bundle = (
                        Transform::from_translation(front_left + translation)
                            .with_scale(Vec3::new(module.length, module.width, module.height)),
                        Mesh3d(meshes.module.clone()),
                        MeshMaterial3d(materials.module.clone()),
                        module,
                    );
                    commands.spawn(bundle).set_parent(parent);
                }
            }
            front_left.y += stack.definition.width;
        }
        Ok(())
    }
}

#[allow(clippy::cast_precision_loss, clippy::as_conversions)]
fn create_building(
    stacks: &[BuildingModuleStack],
) -> Result<BuildingPlot, Result<(), ModularBuildingFactoryError>> {
    let mut lengths: Vec<f32> = stacks
        .iter()
        .map(|stack| {
            stack.definition.front_offset + stack.definition.length + stack.definition.back_offset
        })
        .collect();
    lengths.sort_by(|a, b| a.partial_cmp(b).expect("partial compare should not fail"));
    lengths.dedup();
    if lengths.len() > 1 {
        return Err(Err(InEqualLengths { lengths }));
    }
    let width = stacks
        .iter()
        .fold(0.0, |mut width, stack| width + stack.definition.width);
    let mut heights: Vec<f32> = stacks
        .iter()
        .map(|stack| stack.level_height * stack.levels as f32)
        .collect();
    let building = BuildingPlot {
        width,
        length: *lengths.first().expect("should be at least one length"),
        height: heights
            .into_iter()
            .reduce(f32::max)
            .expect("should not fail"),
    };
    Ok(building)
}
