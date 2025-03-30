use bevy::color::palettes::*;
use bevy::prelude::*;

#[allow(clippy::struct_field_names)]
#[derive(Resource)]
pub struct BuildingMaterials {
    /// Material for the face of a [`BuildingModule`].
    pub face: Handle<StandardMaterial>,
    /// Material for the edges of a [`BuildingModule`].
    pub edges: Handle<StandardMaterial>,
}

impl BuildingMaterials {
    /// System to insert [`BuildingMaterials`] on startup.
    pub(super) fn startup_system(
        mut commands: Commands,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        commands.insert_resource(BuildingMaterials {
            face: materials.add(StandardMaterial {
                base_color: tailwind::STONE_300.into(),
                perceptual_roughness: 0.8,
                ..Default::default()
            }),
            edges: materials.add(StandardMaterial {
                base_color: tailwind::STONE_400.into(),
                alpha_mode: AlphaMode::Opaque,
                depth_bias: 10.0,
                unlit: true,
                ..default()
            }),
        });
    }
}
