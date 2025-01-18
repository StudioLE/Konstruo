use bevy::asset::Assets;
use bevy::color::palettes::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Ground;

pub fn spawn_ground(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let bundle = (
        Ground,
        Mesh3d(meshes.add(Plane3d::default().mesh().size(1000.0, 1000.0))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(tailwind::LIME_800))),
    );
    commands.spawn(bundle);
}
