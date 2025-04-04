use super::*;
use crate::geometry::{Edge, Polyline, Sweep, TriangleList, Vec6, Wireframe};
use crate::infrastructure::SurfaceType::{Carriageway, Footway};
use crate::ui::{EntityState, EntityStateChanged, InterfaceState};
use crate::{Helpers, PATH_ELEVATION};
use bevy::prelude::*;
use bevy::render::mesh::MeshAabb;
use bevy::render::primitives::Aabb;
use std::collections::HashSet;

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
            for (entity, surface, parent, mut mesh, mut aabb) in &mut surfaces {
                if parent.parent != event.path {
                    continue;
                }
                let sweep = Sweep::new(&event.spline, surface.offsets);
                let triangles = sweep.clone().to_triangle_list();
                let m = triangles.clone().to_mesh();
                // TODO: Due to entity picking bug the AABB must also be updated. This will likely be fixed in the future.
                // https://github.com/bevyengine/bevy/issues/18221
                *aabb = m.compute_aabb().expect("Should be able to compute AABB");
                *mesh = Mesh3d(factory.meshes.add(m));
                Helpers::despawn_children(&mut factory.commands, &edges, entity);
                factory.spawn_edges(sweep, entity, true);
                if WIREFRAME_ENABLED {
                    Helpers::despawn_children(&mut factory.commands, &wireframes, entity);
                    factory.spawn_wireframe(triangles, entity);
                }
            }
        }
        if duplicates > 0 {
            trace!("Ignored {duplicates} duplicate SplineChanged events");
        }
    }

    /// Update the [`Edge`] visibility when the [`EntityState`] of the [`Path`] changes.
    pub(super) fn on_state_changed(
        mut events: EventReader<EntityStateChanged>,
        surfaces: Query<&ChildOf, (Without<Edge>, With<PathSurface>)>,
        mut edges: Query<(&ChildOf, &mut Visibility), With<Edge>>,
    ) {
        let mut duplicates = 0;
        let mut updated = HashSet::new();
        for event in events.read() {
            if !updated.insert(event) {
                duplicates += 1;
                continue;
            }
            for (parent, mut visibility) in &mut edges {
                let Ok(surface_parent) = surfaces.get(parent.parent) else {
                    continue;
                };
                if surface_parent.parent != event.entity {
                    continue;
                }
                *visibility = match event.state {
                    EntityState::Default => Visibility::Hidden,
                    EntityState::Hovered | EntityState::Selected => Visibility::Visible,
                };
            }
        }
        if duplicates > 0 {
            trace!("Ignored {duplicates} duplicate EntityStateChanged events");
        }
    }
}

impl PathFactory<'_> {
    /// Spawn a [`PathSurface`] with its mesh geometry.
    pub fn spawn_surface(&mut self, surface: PathSurface, path: &Path, path_entity: Entity) {
        let sweep = Sweep::new(&path.spline, surface.offsets);
        let triangles = sweep.clone().to_triangle_list();
        let surface_bundle = self.surface_bundle(surface, triangles.clone(), path_entity);
        let surface_entity = self
            .commands
            .spawn(surface_bundle)
            .observe(on_pointer_over)
            .observe(on_pointer_out)
            .observe(on_pointer_click)
            .id();
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
            Pickable::default(),
            ChildOf { parent },
        )
    }

    // TODO: Revise to create a single wireframe
    fn spawn_wireframe(&mut self, triangles: TriangleList, parent: Entity) {
        for triangle in triangles.get_triangles() {
            let polyline = Polyline::new(triangle.get_vertices().to_vec());
            let bundle = (
                Wireframe,
                Mesh3d(self.meshes.add(polyline.to_mesh())),
                MeshMaterial3d(self.materials.wireframe.clone()),
                ChildOf { parent },
            );
            self.commands.spawn(bundle);
        }
    }

    // TODO: Revise to create a single edges entity
    fn spawn_edges(&mut self, sweep: Sweep, parent: Entity, is_selected: bool) {
        let edges = sweep.get_edges();
        let material = self.materials.edge.clone();
        let visibility = if is_selected {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        for edge in edges {
            let bundle = (
                Edge,
                visibility,
                Mesh3d(self.meshes.add(edge.to_mesh())),
                MeshMaterial3d(material.clone()),
                ChildOf { parent },
            );
            self.commands.spawn(bundle);
        }
    }
}

fn on_pointer_over(
    trigger: Trigger<Pointer<Over>>,
    mut changed: EventWriter<EntityStateChanged>,
    mut paths: Query<&mut EntityState, With<Path>>,
    surfaces: Query<&ChildOf, (With<PathSurface>, Without<Path>)>,
) {
    let Ok(parent) = surfaces.get(trigger.target()) else {
        error!("Failed to get parent of PathSurface");
        return;
    };
    let Ok(mut state) = paths.get_mut(parent.parent) else {
        warn!("Failed to get Path");
        return;
    };
    if *state != EntityState::Selected {
        *state = EntityState::Hovered;
        changed.write(EntityStateChanged {
            entity: parent.parent,
            state: EntityState::Hovered,
        });
    }
}

fn on_pointer_out(
    trigger: Trigger<Pointer<Out>>,
    mut changed: EventWriter<EntityStateChanged>,
    mut paths: Query<&mut EntityState, With<Path>>,
    surfaces: Query<&ChildOf, (With<PathSurface>, Without<Path>)>,
) {
    let Ok(parent) = surfaces.get(trigger.target()) else {
        error!("Failed to get parent of PathSurface");
        return;
    };
    let Ok(mut state) = paths.get_mut(parent.parent) else {
        warn!("Failed to get Path");
        return;
    };
    if *state != EntityState::Selected {
        *state = EntityState::Default;
        changed.write(EntityStateChanged {
            entity: parent.parent,
            state: EntityState::Default,
        });
    }
}

fn on_pointer_click(
    trigger: Trigger<Pointer<Click>>,
    surfaces: Query<&ChildOf, (With<PathSurface>, Without<Path>)>,
    mut paths: Query<&mut EntityState, With<Path>>,
    mut interface: ResMut<InterfaceState>,
    mut changed: EventWriter<EntityStateChanged>,
) {
    let Ok(parent) = surfaces.get(trigger.target()) else {
        error!("Failed to get parent of PathSurface");
        return;
    };
    let Ok(mut path_state) = paths.get_mut(parent.parent) else {
        warn!("Failed to get Path");
        return;
    };
    if *path_state != EntityState::Selected {
        *path_state = EntityState::Selected;
        *interface = InterfaceState::PathSelected {
            path: parent.parent,
            surface: trigger.target(),
        };
        changed.write(EntityStateChanged {
            entity: parent.parent,
            state: EntityState::Selected,
        });
    }
}
