use bevy::color::palettes::*;
use bevy::prelude::*;
use bevy::render::render_resource::Face;
use konstruo_core::constants::{ENVIRONMENT_MAX, GROUND_ELEVATION};

/// A graphical representation of the ground as a flat plane mesh.
#[derive(Component)]
pub struct Ground;

/// A graphical representation of the ground as a flat plane mesh.
pub struct GroundPlugin;

impl Ground {
    /// System to spawn [`Ground`] on startup.
    fn startup_system(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let plane = Plane3d {
            normal: Dir3::Z,
            ..default()
        };
        let mesh = meshes.add(plane.mesh().size(ENVIRONMENT_MAX, ENVIRONMENT_MAX));
        let material = materials.add(Ground::material());
        let bundle = Ground::bundle(mesh, material);
        commands.spawn(bundle);
    }

    /// Create a bundle for [`Ground`].
    fn bundle(mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> impl Bundle {
        (
            Ground,
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_xyz(0.0, 0.0, GROUND_ELEVATION),
        )
    }

    /// Create the [`Ground`] material.
    fn material() -> StandardMaterial {
        StandardMaterial {
            base_color: tailwind::LIME_800.into(),
            perceptual_roughness: 1.0,
            depth_bias: -3.0,
            double_sided: true,
            cull_mode: Some(Face::Back),
            ..default()
        }
    }
}

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Ground::startup_system);
    }
}
