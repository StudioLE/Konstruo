use crate::infrastructure::SurfaceType;
use bevy::color::palettes::*;
use bevy::prelude::*;

#[allow(clippy::struct_field_names)]
#[derive(Resource)]
pub struct PathMaterials {
    /// Material for the center line.
    pub center_line: Handle<StandardMaterial>,

    /// Material for the default state of [`PathControl`].
    pub control_node: Handle<StandardMaterial>,

    /// Material for the over state of [`PathControl`].
    pub control_node_over: Handle<StandardMaterial>,

    /// Material for the drag state of [`PathControl`].
    pub control_node_drag: Handle<StandardMaterial>,

    /// Material for a [`PathControlLine`].
    pub control_line: Handle<StandardMaterial>,

    /// Material for the edge.
    pub edge: Handle<StandardMaterial>,

    /// Material for the surface wireframe.
    pub wireframe: Handle<StandardMaterial>,

    /// Material for a road [`PathSurface`].
    pub carriageway: Handle<StandardMaterial>,

    /// Material for a road [`PathSurface`].
    pub footway: Handle<StandardMaterial>,

    /// Material for a road [`PathSurface`].
    pub verge: Handle<StandardMaterial>,
}

impl PathMaterials {
    /// System to insert [`PathMaterials`] on startup.
    pub(super) fn startup_system(
        mut commands: Commands,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        commands.insert_resource(PathMaterials {
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
                perceptual_roughness: 0.8,
                depth_bias: -1.0,
                ..default()
            }),
            footway: materials.add(StandardMaterial {
                base_color: tailwind::STONE_300.into(),
                perceptual_roughness: 0.8,
                depth_bias: -1.0,
                ..default()
            }),
            verge: materials.add(StandardMaterial {
                base_color: tailwind::LIME_900.into(),
                perceptual_roughness: 0.8,
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
