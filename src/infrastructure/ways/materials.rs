use crate::infrastructure::SurfaceType;
use bevy::color::palettes::*;
use bevy::prelude::*;

#[allow(clippy::struct_field_names)]
#[derive(Resource)]
pub struct WayMaterials {
    /// Material for the center line.
    pub center_line: Handle<StandardMaterial>,

    /// Material for [`WayControlHandle`] and [`WayControlOrigin`].
    pub control_node: Handle<StandardMaterial>,

    /// Material for the over state of [`WayControlHandle`] and [`WayControlOrigin`].
    pub control_node_over: Handle<StandardMaterial>,

    /// Material for the over state of [`WayControlHandle`] and [`WayControlOrigin`].
    pub control_node_drag: Handle<StandardMaterial>,

    /// Material for the line from origin to handle of a way control.
    pub control_line: Handle<StandardMaterial>,

    /// Material for the edge.
    pub edge: Handle<StandardMaterial>,

    /// Material for the surface wireframe.
    pub wireframe: Handle<StandardMaterial>,

    /// Material for a road [`WaySurface`].
    pub carriageway: Handle<StandardMaterial>,

    /// Material for a road [`WaySurface`].
    pub footway: Handle<StandardMaterial>,

    /// Material for a road [`WaySurface`].
    pub verge: Handle<StandardMaterial>,
}

impl WayMaterials {
    /// System to insert [`WayMaterials`] on startup.
    pub(super) fn startup_system(
        mut commands: Commands,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        commands.insert_resource(WayMaterials {
            center_line: materials.add(StandardMaterial {
                base_color: tailwind::SLATE_500.into(),
                alpha_mode: AlphaMode::Opaque,
                depth_bias: 1.0,
                unlit: true,
                ..default()
            }),
            control_node: materials.add(StandardMaterial {
                emissive: tailwind::GRAY_600.into(),
                base_color: tailwind::GRAY_600.into(),
                alpha_mode: AlphaMode::Opaque,
                depth_bias: 1.0,
                ..default()
            }),
            control_node_over: materials.add(StandardMaterial {
                emissive: tailwind::BLUE_700.into(),
                base_color: tailwind::BLUE_700.into(),
                alpha_mode: AlphaMode::Opaque,
                depth_bias: 1.0,
                ..default()
            }),
            control_node_drag: materials.add(StandardMaterial {
                emissive: tailwind::RED_600.into(),
                base_color: tailwind::RED_600.into(),
                alpha_mode: AlphaMode::Opaque,
                depth_bias: 1.0,
                ..default()
            }),
            control_line: materials.add(StandardMaterial {
                base_color: tailwind::SLATE_500.into(),
                alpha_mode: AlphaMode::Opaque,
                depth_bias: 1.0,
                unlit: true,
                ..default()
            }),
            edge: materials.add(StandardMaterial {
                base_color: tailwind::BLUE_500.into(),
                alpha_mode: AlphaMode::Opaque,
                depth_bias: 10.0,
                unlit: true,
                ..default()
            }),
            wireframe: materials.add(StandardMaterial {
                base_color: tailwind::SLATE_500.into(),
                alpha_mode: AlphaMode::Opaque,
                depth_bias: 1.0,
                unlit: true,
                ..default()
            }),
            carriageway: materials.add(StandardMaterial {
                base_color: tailwind::STONE_400.into(),
                perceptual_roughness: 1.0,
                depth_bias: -1.0,
                ..default()
            }),
            footway: materials.add(StandardMaterial {
                base_color: tailwind::STONE_300.into(),
                perceptual_roughness: 1.0,
                depth_bias: -1.0,
                ..default()
            }),
            verge: materials.add(StandardMaterial {
                base_color: tailwind::LIME_900.into(),
                perceptual_roughness: 1.0,
                depth_bias: -1.0,
                ..default()
            }),
        });
    }

    #[must_use]
    pub fn get_surface(&self, surface: &SurfaceType) -> Handle<StandardMaterial> {
        match surface {
            SurfaceType::Carriageway => self.carriageway.clone(),
            SurfaceType::Footway => self.footway.clone(),
            SurfaceType::Verge => self.verge.clone(),
        }
    }
}
