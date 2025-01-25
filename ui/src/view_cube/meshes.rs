use bevy::prelude::*;

#[derive(Resource)]
pub(super) struct ViewCubeMeshes {
    pub face: Handle<Mesh>,
    pub edge: Handle<Mesh>,
    pub corner: Handle<Mesh>,
}
impl ViewCubeMeshes {
    /// System to insert the [`ViewCubeMeshes`] resource on startup.
    pub(super) fn startup_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
        commands.insert_resource(ViewCubeMeshes {
            face: meshes.add(Cuboid::new(0.6, 0.6, 0.2)),
            edge: meshes.add(Cuboid::from_length(1.0)),
            corner: meshes.add(Cuboid::from_length(0.2)),
        });
    }
}
