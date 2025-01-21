use bevy::color::palettes::*;
use bevy::prelude::*;

#[allow(clippy::struct_field_names)]
#[derive(Resource)]
pub(super) struct ViewCubeMaterials {
    pub face: Handle<StandardMaterial>,
    pub face_over: Handle<StandardMaterial>,
    pub edge: Handle<StandardMaterial>,
    pub edge_over: Handle<StandardMaterial>,
    pub corner: Handle<StandardMaterial>,
    pub corner_over: Handle<StandardMaterial>,
}

impl ViewCubeMaterials {
    /// System to insert the [`ViewCubeMaterials`] resource on startup.
    pub(super) fn startup_system(
        mut commands: Commands,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        commands.insert_resource(ViewCubeMaterials {
            face: materials.add(StandardMaterial {
                emissive: tailwind::SLATE_600.with_alpha(0.6).into(),
                base_color: tailwind::SLATE_600.with_alpha(0.6).into(),
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            }),
            face_over: materials.add(StandardMaterial {
                emissive: tailwind::BLUE_700.into(),
                base_color: tailwind::BLUE_700.into(),
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            }),
            edge: materials.add(StandardMaterial {
                emissive: tailwind::GRAY_600.with_alpha(0.8).into(),
                base_color: tailwind::GRAY_600.with_alpha(0.8).into(),
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            }),
            edge_over: materials.add(StandardMaterial {
                emissive: tailwind::BLUE_700.into(),
                base_color: tailwind::BLUE_700.into(),
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            }),
            corner: materials.add(StandardMaterial {
                emissive: tailwind::GRAY_600.with_alpha(0.6).into(),
                base_color: tailwind::GRAY_600.with_alpha(0.6).into(),
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            }),
            corner_over: materials.add(StandardMaterial {
                emissive: tailwind::BLUE_700.into(),
                base_color: tailwind::BLUE_700.into(),
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            }),
        });
    }
}
