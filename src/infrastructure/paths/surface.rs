use super::*;
use crate::geometry::{Edge, Polyline, Sweep, TriangleList, Vec6, Wireframe};
use crate::infrastructure::SurfaceType::{Carriageway, Footway};
use crate::ui::{EntityState, EntityStateChanged, InterfaceState};
use crate::{Helpers, GROUND_HEIGHT};
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

    /// Spawn a [`PathSurface`] with its mesh geometry.
    pub fn spawn(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &Res<PathMaterials>,
        path: &Path,
        path_entity: Entity,
    ) {
        let sweep = Sweep::new(&path.spline, self.offsets);
        let material = materials.get_surface(&self.purpose);
        let triangles = sweep.clone().to_triangle_list();
        let bundle = (
            self,
            Mesh3d(meshes.add(triangles.clone().to_mesh())),
            MeshMaterial3d(material),
            Transform::from_translation(Vec3::new(0.0, 0.0, GROUND_HEIGHT)),
            PickingBehavior::default(),
        );
        let surface_entity = commands
            .spawn(bundle)
            .observe(on_pointer_over)
            .observe(on_pointer_out)
            .observe(on_pointer_click)
            .set_parent(path_entity)
            .id();
        if WIREFRAME_ENABLED {
            spawn_wireframe(commands, meshes, materials, triangles, surface_entity);
        }
        spawn_edges(commands, meshes, materials, sweep, surface_entity, false);
    }

    /// Update the mesh geometry when the spline changes.
    pub(super) fn on_spline_changed(
        mut commands: Commands,
        mut events: EventReader<SplineChanged>,
        mut surfaces: Query<(Entity, &PathSurface, &Parent, &mut Mesh3d, &mut Aabb)>,
        edges: Query<(Entity, &Parent), (With<Edge>, Without<PathSurface>)>,
        wireframes: Query<
            (Entity, &Parent),
            (With<Wireframe>, Without<PathSurface>, Without<Edge>),
        >,
        mut meshes: ResMut<Assets<Mesh>>,
        materials: Res<PathMaterials>,
    ) {
        let mut duplicates = 0;
        let mut updated = HashSet::new();
        for event in events.read() {
            if !updated.insert(event.path) {
                duplicates += 1;
                continue;
            };
            for (entity, surface, parent, mut mesh, mut aabb) in &mut surfaces {
                if parent.get() != event.path {
                    continue;
                }
                let sweep = Sweep::new(&event.spline, surface.offsets);
                let triangles = sweep.clone().to_triangle_list();
                let m = triangles.clone().to_mesh();
                // TODO: Due to entity picking bug the AABB must also be updated. This will likely be fixed in the future.
                // https://github.com/bevyengine/bevy/issues/18221
                *aabb = m.compute_aabb().expect("Should be able to compute AABB");
                *mesh = Mesh3d(meshes.add(m));
                Helpers::despawn_children(&mut commands, &edges, entity);
                spawn_edges(&mut commands, &mut meshes, &materials, sweep, entity, true);
                if WIREFRAME_ENABLED {
                    Helpers::despawn_children(&mut commands, &wireframes, entity);
                    spawn_wireframe(&mut commands, &mut meshes, &materials, triangles, entity);
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
        surfaces: Query<&Parent, (Without<Edge>, With<PathSurface>)>,
        mut edges: Query<(&Parent, &mut Visibility), With<Edge>>,
    ) {
        let mut duplicates = 0;
        let mut updated = HashSet::new();
        for event in events.read() {
            if !updated.insert(event) {
                duplicates += 1;
                continue;
            }
            for (parent, mut visibility) in &mut edges {
                let Ok(surface_parent) = surfaces.get(parent.get()) else {
                    warn!("Failed to get PathSurface of Edge");
                    continue;
                };
                if surface_parent.get() != event.entity {
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

fn spawn_wireframe(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &Res<PathMaterials>,
    triangles: TriangleList,
    surface_entity: Entity,
) {
    for triangle in triangles.get_triangles() {
        let polyline = Polyline::new(triangle.get_vertices().to_vec());
        let bundle = (
            Wireframe,
            Mesh3d(meshes.add(polyline.to_mesh())),
            MeshMaterial3d(materials.wireframe.clone()),
        );
        commands.spawn(bundle).set_parent(surface_entity);
    }
}

fn spawn_edges(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &Res<PathMaterials>,
    sweep: Sweep,
    surface_entity: Entity,
    is_selected: bool,
) {
    let edges = sweep.get_edges();
    let material = materials.edge.clone();
    let visibility = if is_selected {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
    for edge in edges {
        let bundle = (
            Edge,
            visibility,
            Mesh3d(meshes.add(edge.to_mesh())),
            MeshMaterial3d(material.clone()),
        );
        commands.spawn(bundle).set_parent(surface_entity);
    }
}

fn on_pointer_over(
    trigger: Trigger<Pointer<Over>>,
    mut changed: EventWriter<EntityStateChanged>,
    mut paths: Query<&mut EntityState, With<Path>>,
    surfaces: Query<&Parent, (With<PathSurface>, Without<Path>)>,
) {
    let Ok(parent) = surfaces.get(trigger.entity()) else {
        error!("Failed to get parent of PathSurface");
        return;
    };
    let Ok(mut state) = paths.get_mut(parent.get()) else {
        warn!("Failed to get Path");
        return;
    };
    if *state != EntityState::Selected {
        *state = EntityState::Hovered;
        changed.send(EntityStateChanged {
            entity: parent.get(),
            state: EntityState::Hovered,
        });
    }
}

fn on_pointer_out(
    trigger: Trigger<Pointer<Out>>,
    mut changed: EventWriter<EntityStateChanged>,
    mut paths: Query<&mut EntityState, With<Path>>,
    surfaces: Query<&Parent, (With<PathSurface>, Without<Path>)>,
) {
    let Ok(parent) = surfaces.get(trigger.entity()) else {
        error!("Failed to get parent of PathSurface");
        return;
    };
    let Ok(mut state) = paths.get_mut(parent.get()) else {
        warn!("Failed to get Path");
        return;
    };
    if *state != EntityState::Selected {
        *state = EntityState::Default;
        changed.send(EntityStateChanged {
            entity: parent.get(),
            state: EntityState::Default,
        });
    }
}

fn on_pointer_click(
    trigger: Trigger<Pointer<Click>>,
    surfaces: Query<&Parent, (With<PathSurface>, Without<Path>)>,
    mut paths: Query<&mut EntityState, With<Path>>,
    mut interface: ResMut<InterfaceState>,
    mut changed: EventWriter<EntityStateChanged>,
) {
    let Ok(parent) = surfaces.get(trigger.entity()) else {
        error!("Failed to get parent of PathSurface");
        return;
    };
    let Ok(mut path_state) = paths.get_mut(parent.get()) else {
        warn!("Failed to get Path");
        return;
    };
    if *path_state != EntityState::Selected {
        *path_state = EntityState::Selected;
        *interface = InterfaceState::PathSelected {
            path: parent.get(),
            surface: trigger.entity(),
        };
        changed.send(EntityStateChanged {
            entity: parent.get(),
            state: EntityState::Selected,
        });
    }
}
