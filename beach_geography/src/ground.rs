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
    let bundle = PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(1000.0, 1000.0)),
        material: materials.add(StandardMaterial::from_color(tailwind::LIME_800)),
        ..default()
    };
    commands.spawn((bundle, Ground));
}
