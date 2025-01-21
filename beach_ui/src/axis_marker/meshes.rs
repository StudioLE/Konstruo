use bevy::prelude::*;

#[derive(Resource)]
pub struct AxisMarkerMeshes {
    pub cuboid: Handle<Mesh>,
}

impl AxisMarkerMeshes {
    /// System to insert the [`AxisMarkerMeshes`] resource on startup.
    pub(super) fn startup_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
        commands.insert_resource(AxisMarkerMeshes {
            cuboid: meshes.add(Cuboid::from_length(1.0)),
        });
    }
}
