use super::*;
use bevy::prelude::*;

/// A factory to spawn [`Path`].
pub struct PathFactory<'w> {
    pub commands: Commands<'w, 'w>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub path_meshes: Res<'w, PathMeshes>,
    pub materials: Res<'w, PathMaterials>,
}
