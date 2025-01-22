use bevy::asset::Assets;
use bevy::color::palettes::*;
use bevy::prelude::*;

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
        let mesh = plane.mesh().size(5_000.0, 5_000.0);
        let material = StandardMaterial {
            base_color: tailwind::LIME_800.into(),
            perceptual_roughness: 1.0,
            depth_bias: -2.0,
            double_sided: true,
            cull_mode: None,
            ..default()
        };
        let bundle = (
            Ground,
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(material)),
            Transform::from_xyz(0.0, 0.0, -0.050),
        );
        commands.spawn(bundle);
    }
}

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Ground::startup_system);
    }
}
