use super::*;
use crate::SurfaceType::{Carriageway, Footway};
use bevy::prelude::*;
use bevy::render::mesh::MeshAabb;
use bevy::render::primitives::Aabb;
use konstruo_core::constants::PATH_ELEVATION;
use konstruo_core::EntityExtensions;
use konstruo_geometry::*;
use konstruo_ui::{EntityState, OnEntityState, Selectable};
use std::collections::HashSet;

const SURFACE_TO_PATH_GENERATIONS: usize = 1;
const EDGE_TO_PATH_GENERATIONS: usize = 2;
static WIREFRAME_ENABLED: bool = false;

/// A surface formed by two lines from a [`Path`].
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct PathSurface {
    /// Offsets from the path.
    ///
    /// Front and Back values are ignored.
    offsets: Vec6,
    /// Type of surface.
    purpose: SurfaceType,
}

/// Type of surface.
pub enum SurfaceType {
    /// - <https://en.wikipedia.org/wiki/Carriagepath>
    Carriageway,
    /// - <https://en.wikipedia.org/wiki/Footpath>
    Footway,
    /// - <https://en.wikipedia.org/wiki/Road_verge>
    Verge,
}

impl PathSurface {
    /// Create a new [`PathSurface`] offset from [`Path`].
    #[must_use]
    pub fn new(offsets: Vec6, purpose: SurfaceType) -> Self {
        let offsets = offsets.fix_order();
        Self { offsets, purpose }
    }

    /// Create a new [`PathSurface`] centered at [`Path`].
    #[must_use]
    pub fn centered(width: f32, depth: f32, purpose: SurfaceType) -> Self {
        Self::new(
            Vec6::new(width * -0.5, width * 0.5, 0.0, 0.0, 0.0, depth),
            purpose,
        )
    }

    #[must_use]
    pub fn default_surfaces() -> Vec<PathSurface> {
        vec![
            PathSurface::centered(4.8, 0.025, Carriageway),
            PathSurface::new(Vec6::new(2.4, 4.4, 0.0, 0.0, 0.0, 0.125), Footway),
            PathSurface::new(Vec6::new(-4.4, -2.4, 0.0, 0.0, 0.0, 0.125), Footway),
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
                let sweep = Sweep::new(&event.spline, surface.offsets);
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
        let sweep = Sweep::new(&path.spline, surface.offsets);
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
        let material = self.materials.get_surface(&surface.purpose);
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
