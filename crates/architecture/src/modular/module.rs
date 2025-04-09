use crate::Pitch;
use crate::*;
use bevy::prelude::*;
use konstruo_distribution::Distributable;
use konstruo_geometry::Cuboid;
use konstruo_geometry::*;
use konstruo_ui::*;
use std::collections::HashMap;
use Orientation::*;

const EDGES_TO_BUILDING_GENERATIONS: usize = 3;
const MESH_TO_BUILDING_GENERATIONS: usize = 3;

/// A building module.
#[derive(Clone, Component, Debug, Default)]
#[require(InheritedVisibility, Transform)]
pub struct BuildingModule;

/// A definition for creating a [`BuildingModule`].
#[derive(Clone, Debug)]
pub struct BuildingModuleInfo {
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
    pub openings: Option<Vec<OpeningDistribution>>,
}

impl Default for BuildingModuleInfo {
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

impl BuildingModuleInfo {
    /// Get the scale of [`BuildingModule`].
    fn get_scale(&self) -> Vec3 {
        Vec3::new(self.width, self.length, self.height)
    }

    /// Get the openings by distributing them on each side.
    fn get_openings(&self) -> HashMap<Orientation, Option<Vec<Cuboid>>> {
        Orientation::get_all()
            .into_iter()
            .map(|side| (side, self.distribute_openings(side)))
            .collect()
    }

    /// Distribute openings for the given side.
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

    /// Get the rectangle geometry forming each face by subtracting the openings.
    fn get_face_rectangles(
        &self,
        openings: &HashMap<Orientation, Option<Vec<Cuboid>>>,
    ) -> Vec<[Vec3; 4]> {
        let cuboid = Cuboid::new(Transform::from_scale(self.get_scale()));
        let mut rectangles = Vec::new();
        for side in Orientation::get_all() {
            let face = cuboid.get_face(side);
            let Some(Some(openings)) = openings.get(&side) else {
                rectangles.push(face);
                continue;
            };
            let mut opening_rectangles = Vec::new();
            for opening in openings {
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
            }
        }
        rectangles
    }
}

impl ModularBuildingFactory<'_> {
    /// Spawn a [`BuildingModule`] and hidden [`Edge`].
    pub(super) fn spawn_module(
        &mut self,
        module: &BuildingModuleInfo,
        order: usize,
        parent: Entity,
    ) {
        if let Some(pitch) = module.pitch {
            self.spawn_pitched(module, pitch, order, parent);
        } else {
            self.spawn_cuboid(module, order, parent);
        }
    }

    /// Spawn a [`BuildingModule`] with  edge and face geometry and openings.
    fn spawn_cuboid(&mut self, module: &BuildingModuleInfo, order: usize, parent: Entity) {
        let openings = module.get_openings();
        let rectangles = module.get_face_rectangles(&openings);
        let openings = openings
            .into_iter()
            .filter_map(|(_, values)| values)
            .flatten();
        let module_bundle = Self::module_bundle(module, order, parent);
        let module_edge_bundle = self.module_edges_bundle(module);
        let modules_faces_bundle = self.cuboid_faces_bundle(rectangles);
        let module_entity = self
            .commands
            .spawn(module_bundle)
            .with_children(|commands| {
                commands.spawn(module_edge_bundle);
                commands.spawn(modules_faces_bundle);
            })
            .id();
        for opening in openings {
            let opening_edges_bundle = self.opening_edges_bundle(&opening, module_entity);
            self.commands.spawn(opening_edges_bundle);
            let opening_faces_bundle = self.opening_faces_bundle(&opening, module_entity);
            self.commands.spawn(opening_faces_bundle);
        }
    }

    /// Spawn a [`BuildingModule`] and hidden [`Edge`].
    fn spawn_pitched(
        &mut self,
        module: &BuildingModuleInfo,
        pitch: Pitch,
        order: usize,
        parent: Entity,
    ) {
        let solid = self.pitched_solid_bundle(module, pitch);
        let edges = self.module_edges_bundle(module);
        self.commands
            .spawn(Self::module_bundle(module, order, parent))
            .with_child(solid)
            .with_child(edges);
    }

    /// Create a bundle for [`BuildingModule`].
    fn module_bundle(module: &BuildingModuleInfo, order: usize, parent: Entity) -> impl Bundle {
        let distributable = Distributable {
            order,
            size: Some(module.get_scale()),
            margin: module.margin,
        };
        (
            Name::new("Building Module"),
            BuildingModule,
            Level {
                level: module.level,
            },
            distributable,
            ChildOf { parent },
        )
    }

    /// Create a bundle for the cuboid solid geometry of [`BuildingModule`] with subtracted openings.
    fn cuboid_faces_bundle(&mut self, rectangles: Vec<[Vec3; 4]>) -> impl Bundle {
        let mesh = TriangleList::from_rectangles(rectangles).to_mesh();
        (
            Name::new("Faces of Building Module"),
            Solid,
            Selectable::new(MESH_TO_BUILDING_GENERATIONS),
            Transform::default(),
            Mesh3d(self.meshes.add(mesh)),
            MeshMaterial3d(self.materials.face.clone()),
            Visibility::Visible,
        )
    }

    /// Create a bundle for the edge geometry of [`BuildingModule`].
    fn module_edges_bundle(&self, module: &BuildingModuleInfo) -> impl Bundle {
        let mesh = match module.pitch {
            None => self.building_meshes.cuboid_edges.clone(),
            Some(Pitch::LeftToRight) => self.building_meshes.pitch_left_right_edges.clone(),
            Some(Pitch::FrontToBack) => self.building_meshes.pitch_front_back_edges.clone(),
        };
        (
            Name::new("Edges of Building Module"),
            OnEntityState::new(
                EDGES_TO_BUILDING_GENERATIONS,
                vec![EntityState::Selected, EntityState::Hovered],
            ),
            Edge,
            Transform::from_scale(module.get_scale()),
            Mesh3d(mesh),
            MeshMaterial3d(self.materials.edges.clone()),
            Visibility::Hidden,
        )
    }

    /// Create a bundle for the pitched solid geometry of [`BuildingModule`].
    fn pitched_solid_bundle(&self, module: &BuildingModuleInfo, pitch: Pitch) -> impl Bundle {
        let mesh = match pitch {
            Pitch::LeftToRight => self.building_meshes.pitch_left_right.clone(),
            Pitch::FrontToBack => self.building_meshes.pitch_front_back.clone(),
        };
        (
            Name::new("Pitched Faces of Building Module"),
            Solid,
            Selectable::new(MESH_TO_BUILDING_GENERATIONS),
            Transform::from_scale(module.get_scale()),
            Mesh3d(mesh),
            MeshMaterial3d(self.materials.face.clone()),
            Visibility::Visible,
        )
    }

    fn opening_edges_bundle(&mut self, cuboid: &Cuboid, parent: Entity) -> impl Bundle {
        (
            Name::new("Opening Edges of Building Module"),
            Opening,
            Edge,
            Mesh3d(self.meshes.add(cuboid.get_edges().to_mesh())),
            MeshMaterial3d(self.materials.edges.clone()),
            Visibility::Hidden,
            ChildOf { parent },
        )
    }

    /// Create the inside faces of an opening.
    fn opening_faces_bundle(&mut self, cuboid: &Cuboid, parent: Entity) -> impl Bundle {
        let triangles = TriangleList::from_rectangles(vec![
            cuboid.get_face_reversed(Back),
            cuboid.get_face_reversed(Left),
            cuboid.get_face_reversed(Right),
            cuboid.get_face_reversed(Top),
            cuboid.get_face_reversed(Bottom),
        ]);
        (
            Name::new("Opening Faces of Building Module"),
            Opening,
            Solid,
            Mesh3d(self.meshes.add(triangles.to_mesh())),
            MeshMaterial3d(self.materials.face.clone()),
            ChildOf { parent },
        )
    }
}
