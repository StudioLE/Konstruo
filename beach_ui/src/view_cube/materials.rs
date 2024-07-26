use bevy::color::palettes::*;
use bevy::prelude::*;

#[allow(clippy::struct_field_names)]
#[derive(Resource)]
pub struct ViewCubeMaterials {
    pub side: Handle<StandardMaterial>,
    pub edge: Handle<StandardMaterial>,
    pub corner: Handle<StandardMaterial>,
    pub corner_over: Handle<StandardMaterial>,
}

pub fn insert_materials(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(ViewCubeMaterials {
        side: materials.add(StandardMaterial {
            emissive: tailwind::NEUTRAL_700.into(),
            base_color: tailwind::NEUTRAL_700.into(),
            alpha_mode: AlphaMode::Opaque,
            ..Default::default()
        }),
        edge: materials.add(StandardMaterial {
            emissive: tailwind::NEUTRAL_600.into(),
            base_color: tailwind::NEUTRAL_600.into(),
            alpha_mode: AlphaMode::Opaque,
            ..Default::default()
        }),
        corner: materials.add(StandardMaterial {
            emissive: tailwind::NEUTRAL_700.into(),
            base_color: tailwind::NEUTRAL_700.into(),
            alpha_mode: AlphaMode::Opaque,
            ..Default::default()
        }),
        corner_over: materials.add(StandardMaterial {
            emissive: tailwind::BLUE_700.into(),
            base_color: tailwind::BLUE_700.into(),
            alpha_mode: AlphaMode::Opaque,
            ..Default::default()
        }),
    });
}
