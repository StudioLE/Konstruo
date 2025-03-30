use crate::architecture::Pitch;
use crate::architecture::*;
use crate::distribution::Distributable;
use crate::geometry::Cuboid;
use crate::geometry::*;
use bevy::prelude::*;
use Orientation::*;

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
        mesh: Handle<Mesh>,
        materials: &Res<BuildingMaterials>,
    ) -> (
        Solid,
        Transform,
        Mesh3d,
        MeshMaterial3d<StandardMaterial>,
        Visibility,
    ) {
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

    fn distribute_openings(&self, side: Orientation) -> Option<Vec<Cuboid>> {
        let (right, up, _back) = side.to_elevation_axis();
        // TODO: This will ignore duplicate orientations
        let factories = self.openings.as_ref()?;
        let factory = factories.iter().find(|factory| factory.side == side)?;
        let container = factory.distribute(self.get_scale(), right, up);
        let openings = container
            .items
            .iter()
            .map(|item| {
                let transform = Transform::from_translation(item.translation)
                    .with_scale(item.source.size.expect("size should be set"))
                    .with_rotation(Quat::from_rotation_z(side.get_z_rotation()));
                // TODO: Rotation is likely incorrect for top
                Cuboid::new(transform)
            })
            .collect();
        Some(openings)
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
        if let Some(pitch) = self.pitch {
            self.spawn_pitched(commands, building_meshes, materials, pitch, order, parent);
        } else {
            self.spawn_cuboid(commands, meshes, building_meshes, materials, order, parent);
        };
    }

    /// Spawn a [`BuildingModule`] and hidden [`Edge`].
    pub(super) fn spawn_cuboid(
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
        let bundle = self.edge_bundle(building_meshes, materials);
        commands.spawn(bundle).set_parent(module);
        let cuboid = Cuboid::new(Transform::from_scale(self.get_scale()));
        let mut rectangles = Vec::new();
        for side in Orientation::get_all() {
            let face = cuboid.get_face(side);
            let Some(openings) = self.distribute_openings(side) else {
                rectangles.push(face);
                continue;
            };
            let mut opening_rectangles = Vec::new();
            for opening in openings {
                let bundle = (
                    Opening,
                    Edge,
                    Mesh3d(meshes.add(opening.get_edges().to_mesh())),
                    MeshMaterial3d(materials.edges.clone()),
                );
                commands.spawn(bundle).set_parent(module);
                let mesh = TriangleList::from_rectangles(vec![
                    opening.get_face_reversed(Back),
                    opening.get_face_reversed(Left),
                    opening.get_face_reversed(Right),
                    opening.get_face_reversed(Top),
                    opening.get_face_reversed(Bottom),
                ])
                .to_mesh();
                let bundle = (
                    Opening,
                    Edge,
                    Mesh3d(meshes.add(mesh)),
                    MeshMaterial3d(materials.face.clone()),
                );
                commands.spawn(bundle).set_parent(module);
                opening_rectangles.push(opening.get_face_reversed(Front));
            }
            let (right, up, _back) = side.to_elevation_axis();
            let subdivision = Subdivision {
                bounds: face,
                openings: opening_rectangles,
                main_axis: right,
                cross_axis: up,
            };
            match subdivision.execute() {
                Ok(mut s) => rectangles.append(&mut s),
                Err(e) => {
                    rectangles.push(face);
                    warn!("Failed to create openings in BuildingModule {side} facade: {e:?}");
                }
            };
        }
        let mesh = TriangleList::from_rectangles(rectangles).to_mesh();
        let bundle = Self::cuboid_solid_bundle(meshes.add(mesh), materials);
        commands.spawn(bundle).set_parent(module);
    }

    /// Spawn a [`BuildingModule`] and hidden [`Edge`].
    fn spawn_pitched(
        &self,
        commands: &mut Commands,
        meshes: &Res<BuildingMeshes>,
        materials: &Res<BuildingMaterials>,
        pitch: Pitch,
        order: usize,
        parent: Entity,
    ) {
        let bundle = self.bundle(order);
        let module = commands.spawn(bundle).set_parent(parent).id();
        let bundle = self.pitched_solid_bundle(meshes, materials, pitch);
        commands.spawn(bundle).set_parent(module);
        let bundle = self.edge_bundle(meshes, materials);
        commands.spawn(bundle).set_parent(module);
    }

    /// Get the scale of [`BuildingModule`].
    fn get_scale(&self) -> Vec3 {
        Vec3::new(self.width, self.length, self.height)
    }
}

impl BuildingModule {}
