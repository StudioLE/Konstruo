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
                base_color: tailwind::LIME_900.with_alpha(0.3).into(),
                alpha_mode: AlphaMode::Blend,
                perceptual_roughness: 1.0,
                depth_bias: -2.0,
                unlit: true,
                ..Default::default()
            }),
            medium: materials.add(StandardMaterial {
                base_color: tailwind::LIME_900.with_alpha(0.6).into(),
                alpha_mode: AlphaMode::Blend,
                perceptual_roughness: 1.0,
                depth_bias: -2.0,
                unlit: true,
                ..Default::default()
            }),
            major: materials.add(StandardMaterial {
                base_color: tailwind::LIME_700.with_alpha(0.4).into(),
                alpha_mode: AlphaMode::Blend,
                perceptual_roughness: 1.0,
                depth_bias: 1.0,
                unlit: true,
                ..Default::default()
            }),
        });
    }
}
