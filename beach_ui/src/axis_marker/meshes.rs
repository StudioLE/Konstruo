use bevy::prelude::*;

#[derive(Resource)]
pub struct AxisMarkerMeshes {
    pub cuboid: Handle<Mesh>,
}

pub fn insert_meshes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(AxisMarkerMeshes {
        cuboid: meshes.add(Cuboid::from_length(1.0)),
    });
}
