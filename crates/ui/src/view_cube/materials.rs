use bevy::color::palettes::*;
use bevy::prelude::*;

#[allow(clippy::struct_field_names)]
#[derive(Resource)]
pub(super) struct ViewCubeMaterials {
    pub face_x: Handle<StandardMaterial>,
    pub face_y: Handle<StandardMaterial>,
    pub face_z: Handle<StandardMaterial>,
    pub edge: Handle<StandardMaterial>,
    pub corner: Handle<StandardMaterial>,
    pub face_over: Handle<StandardMaterial>,
    pub edge_over: Handle<StandardMaterial>,
    pub corner_over: Handle<StandardMaterial>,
}

impl ViewCubeMaterials {
    /// System to insert the [`ViewCubeMaterials`] resource on startup.
    pub(super) fn startup_system(
        mut commands: Commands,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let over = materials.add(StandardMaterial {
            base_color: tailwind::BLUE_700.into(),
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 1.0,
            unlit: true,
            ..Default::default()
        });
        commands.insert_resource(ViewCubeMaterials {
            face_x: materials.add(StandardMaterial {
                base_color: tailwind::RED_900.into(),
                alpha_mode: AlphaMode::Opaque,
                perceptual_roughness: 1.0,
                unlit: true,
                ..Default::default()
            }),
            face_y: materials.add(StandardMaterial {
                base_color: tailwind::GREEN_900.into(),
                alpha_mode: AlphaMode::Opaque,
                perceptual_roughness: 1.0,
                unlit: true,
                ..Default::default()
            }),
            face_z: materials.add(StandardMaterial {
                base_color: tailwind::BLUE_900.into(),
                alpha_mode: AlphaMode::Opaque,
                perceptual_roughness: 1.0,
                unlit: true,
                ..Default::default()
            }),
            edge: materials.add(StandardMaterial {
                base_color: tailwind::ZINC_800.into(),
                alpha_mode: AlphaMode::Opaque,
                perceptual_roughness: 1.0,
                unlit: true,
                ..Default::default()
            }),
            corner: materials.add(StandardMaterial {
                base_color: tailwind::ZINC_900.into(),
                alpha_mode: AlphaMode::Opaque,
                perceptual_roughness: 1.0,
                unlit: true,
                ..Default::default()
            }),
            face_over: over.clone(),
            edge_over: over.clone(),
            corner_over: over,
        });
    }
}
