use bevy::color::palettes::*;
use bevy::prelude::*;

#[allow(clippy::struct_field_names)]
#[derive(Resource)]
pub struct WayMaterials {
    /// Material for the origin of a way control.
    pub control_origin: Handle<StandardMaterial>,

    /// Material for the line from origin to handle of a way control.
    pub control_line: Handle<StandardMaterial>,

    /// Material for the handle of a way control.
    pub control_handle: Handle<StandardMaterial>,

    /// Material for the mesh of a way.
    pub mesh: Handle<StandardMaterial>,
}

pub fn insert_materials(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    let control_origin_handle = materials.add(StandardMaterial {
        emissive: tailwind::SKY_800.into(),
        base_color: tailwind::SKY_800.into(),
        alpha_mode: AlphaMode::Opaque,
        depth_bias: 1.0,
        ..Default::default()
    });
    commands.insert_resource(WayMaterials {
        control_origin: control_origin_handle.clone(),
        control_line: materials.add(StandardMaterial {
            emissive: tailwind::SLATE_500.into(),
            base_color: tailwind::SLATE_500.into(),
            alpha_mode: AlphaMode::Opaque,
            depth_bias: 1.0,
            ..Default::default()
        }),
        control_handle: control_origin_handle,
        mesh: materials.add(StandardMaterial {
            emissive: basic::WHITE.into(),
            base_color: basic::WHITE.into(),
            alpha_mode: AlphaMode::Opaque,
            depth_bias: -1.0,
            ..Default::default()
        }),
    });
}
