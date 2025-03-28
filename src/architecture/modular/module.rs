use crate::architecture::{BuildingMaterials, BuildingMeshes, Level, Pitch};
use crate::distribution::Distributable;
use crate::geometry::{Edge, Vec6};
use bevy::prelude::*;

/// A building module.
#[derive(Clone, Component, Debug, Default)]
#[require(InheritedVisibility, Transform)]
pub struct BuildingModule;

/// A factory for creating [`BuildingModule`].
pub struct BuildingModuleFactory {
    /// Level number
    /// 0: Ground
    /// 1: First
    /// -1: Basement
    pub level: isize,
    /// Width from left to right
    pub width: f32,
    /// Length from front to back
    pub length: f32,
    /// Height from bottom to top
    pub height: f32,
    /// Margins or offsets
    pub margin: Option<Vec6>,
    /// Is this a pitched module?
    pub pitch: Option<Pitch>,
}

impl BuildingModuleFactory {
    /// Create a bundle for [`BuildingModule`].
    fn bundle(
        &self,
        meshes: &Res<BuildingMeshes>,
        materials: &Res<BuildingMaterials>,
        order: usize,
    ) -> (
        BuildingModule,
        Level,
        Transform,
        Distributable,
        Mesh3d,
        MeshMaterial3d<StandardMaterial>,
    ) {
        let distributable = Distributable {
            order,
            size: Some(Vec3::new(self.width, self.length, self.height)),
            margin: self.margin,
        };
        let mesh = match self.pitch {
            None => meshes.cuboid.clone(),
            Some(Pitch::LeftToRight) => meshes.pitch_left_right.clone(),
            Some(Pitch::FrontToBack) => meshes.pitch_front_back.clone(),
        };
        (
            BuildingModule,
            Level { level: self.level },
            Transform::from_scale(Vec3::new(self.width, self.length, self.height)),
            distributable,
            Mesh3d(mesh),
            MeshMaterial3d(materials.face.clone()),
        )
    }

    /// Spawn a [`BuildingModule`] and hidden [`Edge`].
    pub(super) fn spawn(
        &self,
        commands: &mut Commands,
        meshes: &Res<BuildingMeshes>,
        materials: &Res<BuildingMaterials>,
        order: usize,
        parent: Entity,
    ) {
        let bundle = self.bundle(meshes, materials, order);
        let entity = commands.spawn(bundle).set_parent(parent).id();
        let edges = match self.pitch {
            None => meshes.cuboid_edges.clone(),
            Some(Pitch::LeftToRight) => meshes.pitch_left_right_edges.clone(),
            Some(Pitch::FrontToBack) => meshes.pitch_front_back_edges.clone(),
        };
        let bundle = (
            Mesh3d(edges),
            MeshMaterial3d(materials.edges.clone()),
            Edge,
            Visibility::Hidden,
        );
        commands.spawn(bundle).set_parent(entity);
    }
}

impl BuildingModule {}
