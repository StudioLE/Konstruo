use crate::ENVIRONMENT_MAX;
use bevy::asset::Assets;
use bevy::color::palettes::*;
use bevy::prelude::*;
use bevy::render::render_resource::Face;

/// A graphical representation of the sky.
#[derive(Component)]
pub struct Sky;

/// A graphical representation of the sky.
pub struct SkyPlugin;

impl Sky {
    /// System to spawn [`Sky`] on startup.
    fn startup_system(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let sphere = Sphere::new(ENVIRONMENT_MAX / 2.0);
        let material = StandardMaterial {
            base_color: tailwind::SKY_200.into(),
            double_sided: true,
            unlit: true,
            cull_mode: Some(Face::Front),
            ..default()
        };
        let bundle = (
            Sky,
            Mesh3d(meshes.add(sphere)),
            MeshMaterial3d(materials.add(material)),
        );
        commands.spawn(bundle);
    }
}

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Sky::startup_system);
    }
}
