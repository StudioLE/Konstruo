use bevy::color::palettes::*;
use bevy::prelude::*;

const COLOR_X: Srgba = tailwind::RED_700;
const COLOR_Y: Srgba = tailwind::GREEN_700;
const COLOR_Z: Srgba = tailwind::SKY_700;

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
                emissive: COLOR_X.into(),
                base_color: COLOR_X.into(),
                alpha_mode: AlphaMode::Opaque,
                ..Default::default()
            }),
            y: materials.add(StandardMaterial {
                emissive: COLOR_Y.into(),
                base_color: COLOR_Y.into(),
                alpha_mode: AlphaMode::Opaque,
                ..Default::default()
            }),
            z: materials.add(StandardMaterial {
                emissive: COLOR_Z.into(),
                base_color: COLOR_Z.into(),
                alpha_mode: AlphaMode::Opaque,
                ..Default::default()
            }),
        };
        commands.insert_resource(resource);
    }
}
