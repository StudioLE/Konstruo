use bevy::asset::{Assets, Handle};
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;

pub fn spawn_line_strip(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    vertices: Vec<Vec3>,
    material: Handle<StandardMaterial>,
) {
    let mesh = Mesh::new(
        PrimitiveTopology::LineStrip,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    let bundle = MaterialMeshBundle {
        mesh: Mesh3d(meshes.add(mesh)),
        material: MeshMaterial3d(material),
        ..default()
    };
    commands.spawn(bundle);
}
