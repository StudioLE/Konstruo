#![allow(dead_code)]
use crate::cameras::orbit::Orbit;
use crate::view_cube::camera::ViewCubeCamera;
use crate::view_cube::RENDER_LAYER;
use beach_core::mathematics::spherical_coordinate_system::spherical_to_cartesian;
use bevy::prelude::Projection::Perspective;
use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::render::view::RenderLayers;

/// <https://bevy-cheatbook.github.io/graphics/camera.html?highlight=viewport#viewport>
pub fn spawn_camera(mut commands: Commands) {
    let viewport = Some(Viewport {
        physical_position: UVec2::new(0, 0),
        physical_size: UVec2::new(150, 150),
        ..default()
    });
    let camera = Camera {
        order: 2,
        viewport,
        ..default()
    };
    let projection = Perspective(PerspectiveProjection { ..default() });
    let transform = Transform::from_xyz(0.0, 0.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y);
    let camera = Camera3dBundle {
        camera,
        projection,
        transform,
        ..default()
    };
    let layers = RenderLayers::layer(RENDER_LAYER);
    commands.spawn((camera, layers, ViewCubeCamera));
}

pub fn on_orbit_changed(
    orbit: Query<&Orbit, Changed<Orbit>>,
    transform: Query<&mut Transform, With<ViewCubeCamera>>,
) {
    on_orbit_changed_internal(orbit, transform);
}

fn on_orbit_changed_internal(
    orbit: Query<&Orbit, Changed<Orbit>>,
    mut transform: Query<&mut Transform, With<ViewCubeCamera>>,
) -> Option<()> {
    let orbit = orbit.get_single().ok()?;
    let mut entity_transform = transform.get_single_mut().ok()?;
    let mut transform = orbit.get_transform();
    transform.translation = spherical_to_cartesian(3.0, orbit.get_polar(), orbit.get_azimuth());
    *entity_transform = transform;
    Some(())
}
