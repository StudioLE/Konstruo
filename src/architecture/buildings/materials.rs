use bevy::color::palettes::*;
use bevy::prelude::*;

#[allow(clippy::struct_field_names)]
#[derive(Resource)]
pub struct BuildingMaterials {
    /// Material for [`BuildingModule`].
    pub module: Handle<StandardMaterial>,
}

impl BuildingMaterials {
    /// System to insert [`BuildingMaterials`] on startup.
    pub(super) fn startup_system(
        mut commands: Commands,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        commands.insert_resource(BuildingMaterials {
            module: materials.add(StandardMaterial {
                // base_color: tailwind::STONE_800.into(),
                base_color: tailwind::STONE_400.into(),
                perceptual_roughness: 1.0,
                // depth_bias: -1.0,
                // double_sided: true,
                // cull_mode: None,
                // unlit: true,
                ..Default::default()
            }),
        });
    }
}
