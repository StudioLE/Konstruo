use crate::architecture::Pitch;
use crate::architecture::*;
use crate::distribution::Distributable;
use crate::geometry::Cuboid;
use crate::geometry::*;
use bevy::prelude::*;

/// A building module.
#[derive(Clone, Component, Debug, Default)]
#[require(InheritedVisibility, Transform)]
pub struct BuildingModule;

/// A factory for creating [`BuildingModule`].
#[derive(Clone, Debug)]
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
    /// Is this a pitched module?
    pub openings: Option<Vec<OpeningFactory>>,
}

impl Default for BuildingModuleFactory {
    fn default() -> Self {
        Self {
            level: 0,
            width: 1.000,
            length: 1.000,
            height: 2.400,
            margin: None,
            pitch: None,
            openings: None,
        }
    }
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

    /// Create a bundle for the cuboid solid geometry of [`BuildingModule`] with subtracted openings.
    fn cuboid_solid_bundle(
        &self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &Res<BuildingMaterials>,
    ) -> (
        Solid,
        Transform,
        Mesh3d,
        MeshMaterial3d<StandardMaterial>,
        Visibility,
    ) {
        let mesh = meshes.add(self.create_cuboid_with_openings().to_mesh());
        (
            Solid,
            Transform::default(),
            Mesh3d(mesh),
            MeshMaterial3d(materials.face.clone()),
            Visibility::Visible,
        )
    }

    /// Create a bundle for the pitched solid geometry of [`BuildingModule`].
    fn pitched_solid_bundle(
        &self,
        meshes: &Res<BuildingMeshes>,
        materials: &Res<BuildingMaterials>,
        pitch: Pitch,
    ) -> (
        Solid,
        Transform,
        Mesh3d,
        MeshMaterial3d<StandardMaterial>,
        Visibility,
    ) {
        let mesh = match pitch {
            Pitch::LeftToRight => meshes.pitch_left_right.clone(),
            Pitch::FrontToBack => meshes.pitch_front_back.clone(),
        };
        (
            Solid,
            Transform::from_scale(self.get_scale()),
            Mesh3d(mesh),
            MeshMaterial3d(materials.face.clone()),
            Visibility::Visible,
        )
    }

    fn create_cuboid_with_openings(&self) -> TriangleList {
        let cuboid = Cuboid::new(Transform::from_scale(self.get_scale()));
        let mut triangles = Vec::new();
        for orientation in Orientation::get_all() {
            let face = cuboid.get_face(orientation);
            triangles.append(&mut Triangle::from_rectangle(face).to_vec());
        }
        TriangleList::new(triangles)
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
            Visibility::Visible,
        )
    }

    /// Spawn a [`BuildingModule`] and hidden [`Edge`].
    pub(super) fn spawn(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        building_meshes: &Res<BuildingMeshes>,
        materials: &Res<BuildingMaterials>,
        order: usize,
        parent: Entity,
    ) {
        let bundle = self.bundle(order);
        let module = commands.spawn(bundle).set_parent(parent).id();
        let bundle = if let Some(pitch) = self.pitch {
            self.pitched_solid_bundle(building_meshes, materials, pitch)
        } else {
            self.cuboid_solid_bundle(meshes, materials)
        };
        commands.spawn(bundle).set_parent(module);
        let bundle = self.edge_bundle(building_meshes, materials);
        commands.spawn(bundle).set_parent(module);
        for openings in self.openings.iter().flatten() {
            openings.spawn(
                commands,
                building_meshes,
                materials,
                self.get_scale(),
                module,
            );
        }
    }

    /// Get the scale of [`BuildingModule`].
    fn get_scale(&self) -> Vec3 {
        Vec3::new(self.width, self.length, self.height)
    }
}

impl BuildingModule {}
