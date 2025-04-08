use super::*;
use bevy::prelude::*;

/// A factory to spawn [`ModularBuilding`].
pub struct ModularBuildingFactory<'w> {
    pub commands: Commands<'w, 'w>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub building_meshes: Res<'w, BuildingMeshes>,
    pub materials: Res<'w, BuildingMaterials>,
}
