use bevy::color::palettes::*;
use bevy::prelude::*;

const COLOR_X: Srgba = tailwind::RED_600;
const COLOR_Y: Srgba = tailwind::GREEN_600;
const COLOR_Z: Srgba = tailwind::SKY_600;

#[allow(clippy::struct_field_names)]
#[derive(Resource)]
pub struct AxisMarkerMaterials {
    pub x: Handle<StandardMaterial>,
    pub y: Handle<StandardMaterial>,
    pub z: Handle<StandardMaterial>,
}

impl AxisMarkerMaterials {
    /// System to insert the [`AxisMarkerMaterials`] resource on startup.
    pub(super) fn startup_system(
        mut commands: Commands,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let resource = AxisMarkerMaterials {
            x: materials.add(StandardMaterial {
                base_color: COLOR_X.into(),
                alpha_mode: AlphaMode::Opaque,
                perceptual_roughness: 1.0,
                unlit: true,
                ..Default::default()
            }),
            y: materials.add(StandardMaterial {
                base_color: COLOR_Y.into(),
                alpha_mode: AlphaMode::Opaque,
                perceptual_roughness: 1.0,
                unlit: true,
                ..Default::default()
            }),
            z: materials.add(StandardMaterial {
                base_color: COLOR_Z.into(),
                alpha_mode: AlphaMode::Opaque,
                perceptual_roughness: 1.0,
                unlit: true,
                ..Default::default()
            }),
        };
        commands.insert_resource(resource);
    }
}
