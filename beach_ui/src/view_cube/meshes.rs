use bevy::prelude::*;

#[allow(clippy::struct_field_names)]
#[derive(Resource)]
pub struct ViewCubeMeshes {
    pub side: Handle<Mesh>,
    pub edge: Handle<Mesh>,
    pub corner: Handle<Mesh>,
}

pub fn insert_meshes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let view_cube_side = Cuboid::new(0.6, 0.6, 0.2);
    let view_cube_edge = Cuboid::from_length(1.0);
    let view_cube_corner = Cuboid::from_length(0.2);
    commands.insert_resource(ViewCubeMeshes {
        side: meshes.add(view_cube_side),
        edge: meshes.add(view_cube_edge),
        corner: meshes.add(view_cube_corner),
    });
}
