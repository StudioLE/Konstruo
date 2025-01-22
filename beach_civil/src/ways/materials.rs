use bevy::color::palettes::*;
use bevy::prelude::*;

#[allow(clippy::struct_field_names)]
#[derive(Resource)]
pub struct WayMaterials {
    /// Material for [`WayControlHandle`] and [`WayControlOrigin`].
    pub control_node: Handle<StandardMaterial>,

    /// Material for the over state of [`WayControlHandle`] and [`WayControlOrigin`].
    pub control_node_over: Handle<StandardMaterial>,

    /// Material for the over state of [`WayControlHandle`] and [`WayControlOrigin`].
    pub control_node_drag: Handle<StandardMaterial>,

    /// Material for the line from origin to handle of a way control.
    pub control_line: Handle<StandardMaterial>,

    /// Material for the mesh of a way.
    pub mesh: Handle<StandardMaterial>,
}

impl WayMaterials {
    /// System to insert [`WayMaterials`] on startup.
    pub(super) fn startup_system(
        mut commands: Commands,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        commands.insert_resource(WayMaterials {
            control_node: materials.add(StandardMaterial {
                emissive: tailwind::GRAY_600.into(),
                base_color: tailwind::GRAY_600.into(),
                alpha_mode: AlphaMode::Opaque,
                depth_bias: 1.0,
                ..Default::default()
            }),
            control_line: materials.add(StandardMaterial {
                emissive: tailwind::SLATE_500.into(),
                base_color: tailwind::SLATE_500.into(),
                alpha_mode: AlphaMode::Opaque,
                depth_bias: 1.0,
                ..Default::default()
            }),
            mesh: materials.add(StandardMaterial {
                emissive: basic::WHITE.into(),
                base_color: basic::WHITE.into(),
                alpha_mode: AlphaMode::Opaque,
                depth_bias: -1.0,
                double_sided: true,
                cull_mode: None,
                ..Default::default()
            }),
            control_node_over: materials.add(StandardMaterial {
                emissive: tailwind::BLUE_700.into(),
                base_color: tailwind::BLUE_700.into(),
                alpha_mode: AlphaMode::Opaque,
                depth_bias: 1.0,
                ..Default::default()
            }),
            control_node_drag: materials.add(StandardMaterial {
                emissive: tailwind::RED_600.into(),
                base_color: tailwind::RED_600.into(),
                alpha_mode: AlphaMode::Opaque,
                depth_bias: 1.0,
                ..Default::default()
            }),
        });
    }
}
