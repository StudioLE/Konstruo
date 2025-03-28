use crate::architecture::Pitch;
use crate::architecture::*;
use crate::distribution::Distributable;
use crate::geometry::*;
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
    fn bundle(&self, order: usize) -> (BuildingModule, Level, Distributable) {
        let distributable = Distributable {
            order,
            size: Some(self.get_scale()),
            margin: self.margin,
        };
        (BuildingModule, Level { level: self.level }, distributable)
    }

    /// Create a bundle for the solid geometry of [`BuildingModule`].
    fn solid_bundle(
        &self,
        meshes: &Res<BuildingMeshes>,
        materials: &Res<BuildingMaterials>,
    ) -> (Solid, Transform, Mesh3d, MeshMaterial3d<StandardMaterial>) {
        let mesh = match self.pitch {
            None => meshes.cuboid.clone(),
            Some(Pitch::LeftToRight) => meshes.pitch_left_right.clone(),
            Some(Pitch::FrontToBack) => meshes.pitch_front_back.clone(),
        };
        (
            Solid,
            Transform::from_scale(self.get_scale()),
            Mesh3d(mesh),
            MeshMaterial3d(materials.face.clone()),
        )
    }

    /// Create a bundle for the edge geometry of [`BuildingModule`].
    fn edge_bundle(
        &self,
        meshes: &Res<BuildingMeshes>,
        materials: &Res<BuildingMaterials>,
    ) -> (
        Edge,
        Transform,
        Mesh3d,
        MeshMaterial3d<StandardMaterial>,
        Visibility,
    ) {
        let mesh = match self.pitch {
            None => meshes.cuboid_edges.clone(),
            Some(Pitch::LeftToRight) => meshes.pitch_left_right_edges.clone(),
            Some(Pitch::FrontToBack) => meshes.pitch_front_back_edges.clone(),
        };
        (
            Edge,
            Transform::from_scale(self.get_scale()),
            Mesh3d(mesh),
            MeshMaterial3d(materials.edges.clone()),
            Visibility::Hidden,
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
        let bundle = self.bundle(order);
        let entity = commands.spawn(bundle).set_parent(parent).id();
        let bundle = self.solid_bundle(meshes, materials);
        commands.spawn(bundle).set_parent(entity);
        let bundle = self.edge_bundle(meshes, materials);
        commands.spawn(bundle).set_parent(entity);
    }

    /// Get the scale of [`BuildingModule`].
    fn get_scale(&self) -> Vec3 {
        Vec3::new(self.width, self.length, self.height)
    }
}

impl BuildingModule {}
