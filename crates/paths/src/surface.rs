use super::*;
use bevy::prelude::*;
use bevy::render::mesh::MeshAabb;
use bevy::render::primitives::Aabb;
use konstruo_beziers::Sweep;
use konstruo_core::constants::PATH_ELEVATION;
use konstruo_core::EntityExtensions;
use konstruo_geometry::*;
use konstruo_ui::{EntityState, OnEntityState, Selectable};
use std::collections::HashSet;
use PathSurfacePosition::*;
use PathSurfaceType::*;

const SURFACE_TO_PATH_GENERATIONS: usize = 1;
const EDGE_TO_PATH_GENERATIONS: usize = 2;
static WIREFRAME_ENABLED: bool = false;

/// A surface formed by two lines from a [`Path`].
///
/// At present the surface is extruded up but this will change:
/// <https://github.com/StudioLE/Konstruo/issues/34>
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct PathSurface {
    info: PathSurfaceInfo,
}

/// A definition for creating a [`PathSurface`].
#[derive(Clone, Debug)]
pub struct PathSurfaceInfo {
    /// Width from side to side.
    pub width: f32,
    /// Depth from top to bottom.
    pub depth: f32,
    /// Is it centered over or offset from the path?
    pub position: PathSurfacePosition,
    /// Type of surface.
    pub purpose: PathSurfaceType,
}

/// Positioning of the surface.
///
/// Is it centered over or offset from the path?
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PathSurfacePosition {
    /// Centered over the [`Path`].
    Centered,
    /// Offset by value to the side of the [`Path`].
    ///
    /// Values can be positive or negative to indicate which side.
    Offset(f32),
}

/// Type of surface.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PathSurfaceType {
    /// - <https://en.wikipedia.org/wiki/Carriagepath>
    Carriageway,
    /// - <https://en.wikipedia.org/wiki/Footpath>
    Footway,
    /// - <https://en.wikipedia.org/wiki/Road_verge>
    Verge,
}

impl PathSurfaceInfo {
    #[must_use]
    pub fn get_vec6(&self) -> Vec6 {
        match self.position {
            Offset(offset) => Vec6 {
                left: self.width * -0.5 + offset,
                right: self.width * 0.5 + offset,
                top: self.depth,
                ..default()
            },
            Centered => Vec6 {
                left: self.width * -0.5,
                right: self.width * 0.5,
                top: self.depth,
                ..default()
            },
        }
    }
}

impl PathSurface {
    /// Create a new [`PathSurface`] offset from [`Path`].
    #[must_use]
    pub fn new(
        width: f32,
        depth: f32,
        position: PathSurfacePosition,
        purpose: PathSurfaceType,
    ) -> Self {
        Self {
            info: PathSurfaceInfo {
                width,
                depth,
                position,
                purpose,
            },
        }
    }

    #[must_use]
    pub fn default_surfaces() -> Vec<PathSurface> {
        vec![
            PathSurface::new(4.8, 0.025, Centered, Carriageway),
            PathSurface::new(2.0, 0.125, Offset(3.4), Footway),
            PathSurface::new(2.0, 0.125, Offset(-3.4), Footway),
        ]
    }

    /// Update the mesh geometry when the spline changes.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn on_spline_changed(
        commands: Commands,
        meshes: ResMut<Assets<Mesh>>,
        path_meshes: Res<PathMeshes>,
        materials: Res<PathMaterials>,
        mut events: EventReader<SplineChanged>,
        mut surfaces: Query<(Entity, &PathSurface, &ChildOf, &mut Mesh3d, &mut Aabb)>,
        edges: Query<(Entity, &ChildOf), (With<Edge>, Without<PathSurface>)>,
        wireframes: Query<
            (Entity, &ChildOf),
            (With<Wireframe>, Without<PathSurface>, Without<Edge>),
        >,
    ) {
        let mut factory = PathFactory {
            commands,
            meshes,
            path_meshes,
            materials,
        };
        let mut duplicates = 0;
        let mut updated = HashSet::new();
        for event in events.read() {
            if !updated.insert(event.path) {
                duplicates += 1;
                continue;
            }
            for (entity, surface, child_of, mut mesh, mut aabb) in &mut surfaces {
                if child_of.parent != event.path {
                    continue;
                }
                let sweep = Sweep::new(&event.spline, surface.info.get_vec6());
                let triangles = sweep.clone().to_triangle_list();
                let m = triangles.clone().to_mesh();
                // TODO: Due to entity picking bug the AABB must also be updated. This will likely be fixed in the future.
                // https://github.com/bevyengine/bevy/issues/18221
                *aabb = m.compute_aabb().expect("Should be able to compute AABB");
                *mesh = Mesh3d(factory.meshes.add(m));
                entity.despawn_children(&mut factory.commands, &edges);
                factory.spawn_edges(sweep, entity, true);
                if WIREFRAME_ENABLED {
                    entity.despawn_children(&mut factory.commands, &wireframes);
                    factory.spawn_wireframe(triangles, entity);
                }
            }
        }
        if duplicates > 0 {
            trace!("Ignored {duplicates} duplicate SplineChanged events");
        }
    }
}

impl PathFactory<'_> {
    /// Spawn a [`PathSurface`] with its mesh geometry.
    pub fn spawn_surface(&mut self, surface: PathSurface, path: &Path, path_entity: Entity) {
        let sweep = Sweep::new(&path.spline, surface.info.get_vec6());
        let triangles = sweep.clone().to_triangle_list();
        let surface_bundle = self.surface_bundle(surface, triangles.clone(), path_entity);
        let surface_entity = self.commands.spawn(surface_bundle).id();
        if WIREFRAME_ENABLED {
            self.spawn_wireframe(triangles, surface_entity);
        }
        self.spawn_edges(sweep, surface_entity, false);
    }

    /// Spawn a [`PathSurface`] with its mesh geometry.
    fn surface_bundle(
        &mut self,
        surface: PathSurface,
        triangles: TriangleList,
        parent: Entity,
    ) -> impl Bundle {
        let material = self.materials.get_surface(&surface.info.purpose);
        (
            surface,
            Mesh3d(self.meshes.add(triangles.clone().to_mesh())),
            MeshMaterial3d(material),
            Transform::from_translation(Vec3::new(0.0, 0.0, PATH_ELEVATION)),
            Selectable::new(SURFACE_TO_PATH_GENERATIONS),
            Pickable::default(),
            ChildOf { parent },
        )
    }

    fn spawn_wireframe(&mut self, triangles: TriangleList, parent: Entity) {
        let lines = triangles
            .get_triangles()
            .iter()
            .flat_map(Triangle::to_lines)
            .collect();
        let bundle = (
            Wireframe,
            Mesh3d(self.meshes.add(LineList::from_lines(lines).to_mesh())),
            MeshMaterial3d(self.materials.wireframe.clone()),
            ChildOf { parent },
        );
        self.commands.spawn(bundle);
    }

    fn spawn_edges(&mut self, sweep: Sweep, parent: Entity, is_selected: bool) {
        let edges = sweep.get_edges();
        let lines = edges.iter().flat_map(Polyline::to_lines).collect();
        let lines = LineList::from_lines(lines);
        let material = self.materials.edge.clone();
        let visibility = if is_selected {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        let bundle = (
            Edge,
            OnEntityState::new(
                EDGE_TO_PATH_GENERATIONS,
                vec![EntityState::Hovered, EntityState::Selected],
            ),
            visibility,
            Mesh3d(self.meshes.add(lines.to_mesh())),
            MeshMaterial3d(material.clone()),
            ChildOf { parent },
        );
        self.commands.spawn(bundle);
    }
}
