use bevy::color::palettes::*;
use bevy::prelude::*;

#[allow(clippy::struct_field_names)]
#[derive(Resource)]
pub(super) struct GridMaterials {
    pub minor: Handle<StandardMaterial>,
    pub medium: Handle<StandardMaterial>,
    pub major: Handle<StandardMaterial>,
}

impl GridMaterials {
    /// System to insert the [`GridMaterials`] resource on startup.
    pub(super) fn startup_system(
        mut commands: Commands,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        commands.insert_resource(GridMaterials {
            minor: materials.add(StandardMaterial {
                emissive: basic::WHITE.into(),
                base_color: basic::WHITE.with_alpha(0.05).into(),
                alpha_mode: AlphaMode::Blend,
                perceptual_roughness: 1.0,
                ..Default::default()
            }),
            medium: materials.add(StandardMaterial {
                emissive: basic::WHITE.into(),
                base_color: basic::WHITE.with_alpha(0.2).into(),
                alpha_mode: AlphaMode::Blend,
                perceptual_roughness: 1.0,
                ..Default::default()
            }),
            major: materials.add(StandardMaterial {
                emissive: basic::WHITE.into(),
                base_color: basic::WHITE.with_alpha(0.35).into(),
                alpha_mode: AlphaMode::Blend,
                perceptual_roughness: 1.0,
                ..Default::default()
            }),
        });
    }
}
