use super::*;
use crate::{Orbit, VIEW_CUBE_CAMERA_ORDER};
#[cfg(not(target_arch = "wasm32"))]
use bevy::anti_alias::smaa::{Smaa, SmaaPreset};
use bevy::camera::visibility::RenderLayers;
use bevy::camera::ScalingMode::Fixed;
use bevy::camera::{Hdr, Viewport};
use bevy::prelude::Projection::Orthographic;
use bevy::prelude::*;
use konstruo_core::constants::CAMERA_MSAA;

/// A camera looking at a geoemtric view cube that rotates according to [`Orbit`].
#[derive(Component)]
pub(super) struct ViewCubeCamera;

impl ViewCubeCamera {
    /// System to spawn a [`ViewCubeCamera`] on startup
    /// <https://bevy-cheatbook.github.io/graphics/camera.html?highlight=viewport#viewport>
    pub(super) fn startup_system(mut commands: Commands) {
        let viewport = Some(Viewport {
            physical_position: UVec2::splat(10),
            physical_size: UVec2::splat(100),
            ..default()
        });
        let bundle = (
            ViewCubeCamera,
            Camera3d::default(),
            Camera {
                order: VIEW_CUBE_CAMERA_ORDER,
                viewport,
                clear_color: ClearColorConfig::None,
                ..default()
            },
            // Matches the `Hdr` that `AtmosphereSettings` requires on the primary camera.
            // Two cameras share an intermediate render target only when their format
            // matches, and without this the viewport composites over an empty one, rendering
            // solid black.
            Hdr,
            CAMERA_MSAA,
            #[cfg(not(target_arch = "wasm32"))]
            Smaa {
                preset: SmaaPreset::Ultra,
            },
            Orthographic(OrthographicProjection {
                scaling_mode: Fixed {
                    width: 2.0,
                    height: 2.0,
                },
                ..OrthographicProjection::default_3d()
            }),
            Transform::from_xyz(0.0, 0.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
            RenderLayers::layer(RENDER_LAYER),
        );
        commands.spawn(bundle);
    }

    /// System to rotate the [`ViewCubeCamera`] when [`Orbit`] is changed.
    pub fn update_system(
        orbit: Query<&Orbit, Changed<Orbit>>,
        mut transform: Query<&mut Transform, With<ViewCubeCamera>>,
    ) {
        let Ok(orbit) = orbit.single() else {
            return;
        };
        let Ok(mut transform) = transform.single_mut() else {
            warn!("Failed to get ViewCubeCamera");
            return;
        };
        *transform = Transform::from_translation(orbit.get_cartesian_translation().normalize())
            .with_rotation(orbit.get_orientation());
    }
}
