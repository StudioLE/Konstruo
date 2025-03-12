use super::*;
use crate::geometry::{Edge, Polyline, Sweep, TriangleList, Vec6, Wireframe};
use crate::infrastructure::SurfaceType::{Carriageway, Footway};
use crate::ui::{EntityState, EntityStateChanged, InterfaceState};
use crate::{Helpers, GROUND_HEIGHT};
use bevy::prelude::*;
use std::collections::HashSet;

/// A surface formed by two lines from a [`Way`].
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct WaySurface {
    /// Offsets from the way.
    ///
    /// Front and Back values are ignored.
    offsets: Vec6,
    /// Type of surface.
    purpose: SurfaceType,
}

/// Type of surface.
pub enum SurfaceType {
    /// - <https://en.wikipedia.org/wiki/Carriageway>
    Carriageway,
    /// - <https://en.wikipedia.org/wiki/Footway>
    Footway,
    /// - <https://en.wikipedia.org/wiki/Road_verge>
    Verge,
}

impl WaySurface {
    /// Create a new [`WaySurface`] offset from [`Way`].
    #[must_use]
    pub fn new(offsets: Vec6, purpose: SurfaceType) -> Self {
        let offsets = offsets.fix_order();
        Self { offsets, purpose }
    }

    /// Create a new [`WaySurface`] centered at [`Way`].
    #[must_use]
    pub fn centered(width: f32, depth: f32, purpose: SurfaceType) -> Self {
        Self::new(
            Vec6::new(width * -0.5, width * 0.5, 0.0, 0.0, 0.0, depth),
            purpose,
        )
    }

    #[must_use]
    pub fn default_surfaces() -> Vec<WaySurface> {
        vec![
            WaySurface::centered(4.8, 0.025, Carriageway),
            WaySurface::new(Vec6::new(2.4, 4.4, 0.0, 0.0, 0.0, 0.125), Footway),
            WaySurface::new(Vec6::new(-4.4, -2.4, 0.0, 0.0, 0.0, 0.125), Footway),
        ]
    }

    /// Spawn a [`WaySurface`] with its mesh geometry.
    pub fn spawn(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &Res<WayMaterials>,
        way: &Way,
        way_entity: Entity,
    ) {
        let sweep = Sweep::new(&way.spline, self.offsets);
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
            .set_parent(way_entity)
            .id();
        spawn_wireframe(commands, meshes, materials, triangles, surface_entity);
        spawn_edges(commands, meshes, materials, sweep, surface_entity, false);
    }

    /// Update the mesh geometry when the spline changes.
    pub(super) fn on_spline_changed(
        mut commands: Commands,
        mut events: EventReader<SplineChanged>,
        mut surfaces: Query<(Entity, &WaySurface, &Parent, &mut Mesh3d)>,
        edges: Query<(Entity, &Parent), (With<Edge>, Without<WaySurface>)>,
        wireframes: Query<(Entity, &Parent), (With<Wireframe>, Without<WaySurface>, Without<Edge>)>,
        mut meshes: ResMut<Assets<Mesh>>,
        materials: Res<WayMaterials>,
    ) {
        let mut count = 0;
        let mut updated = HashSet::new();
        for event in events.read() {
            count += 1;
            if updated.contains(&event.way) {
                continue;
            }
            updated.insert(event.way);
            for (entity, surface, parent, mut mesh) in &mut surfaces {
                if parent.get() != event.way {
                    continue;
                }
                let sweep = Sweep::new(&event.spline, surface.offsets);
                let triangles = sweep.clone().to_triangle_list();
                *mesh = Mesh3d(meshes.add(triangles.clone().to_mesh()));
                Helpers::despawn_children(&mut commands, &edges, entity);
                spawn_edges(&mut commands, &mut meshes, &materials, sweep, entity, true);
                Helpers::despawn_children(&mut commands, &wireframes, entity);
                spawn_wireframe(&mut commands, &mut meshes, &materials, triangles, entity);
            }
        }
        if count != 0 {
            trace!("Responded to {} of {count} events", updated.len());
        }
    }

    /// Update the [`Edge`] visibility when the [`EntityState`] of the [`Way`] changes.
    pub(super) fn on_state_changed(
        mut events: EventReader<EntityStateChanged>,
        surfaces: Query<&Parent, (Without<Edge>, With<WaySurface>)>,
        mut edges: Query<(&Parent, &mut Visibility), With<Edge>>,
    ) {
        let mut count = 0;
        let mut updated = HashSet::new();
        for event in events.read() {
            count += 1;
            if !updated.insert(event.entity) {
                continue;
            }
            for (parent, mut visibility) in &mut edges {
                let Ok(surface_parent) = surfaces.get(parent.get()) else {
                    warn!("Failed to get WaySurface of Edge");
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
        if count != 0 {
            trace!("Responded to {} of {count} events", updated.len());
        }
    }
}

fn spawn_wireframe(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &Res<WayMaterials>,
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
    materials: &Res<WayMaterials>,
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
    mut ways: Query<&mut EntityState, With<Way>>,
    surfaces: Query<&Parent, (With<WaySurface>, Without<Way>)>,
) {
    let Ok(parent) = surfaces.get(trigger.entity()) else {
        error!("Failed to get parent of WaySurface");
        return;
    };
    let Ok(mut state) = ways.get_mut(parent.get()) else {
        warn!("Failed to get Way");
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
    mut ways: Query<&mut EntityState, With<Way>>,
    surfaces: Query<&Parent, (With<WaySurface>, Without<Way>)>,
) {
    let Ok(parent) = surfaces.get(trigger.entity()) else {
        error!("Failed to get parent of WaySurface");
        return;
    };
    let Ok(mut state) = ways.get_mut(parent.get()) else {
        warn!("Failed to get Way");
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
    surfaces: Query<&Parent, (With<WaySurface>, Without<Way>)>,
    mut ways: Query<&mut EntityState, With<Way>>,
    mut interface: ResMut<InterfaceState>,
    mut changed: EventWriter<EntityStateChanged>,
) {
    trace!("WaySurface clicked");
    let Ok(parent) = surfaces.get(trigger.entity()) else {
        error!("Failed to get parent of WaySurface");
        return;
    };
    let Ok(mut way_state) = ways.get_mut(parent.get()) else {
        warn!("Failed to get Way");
        return;
    };
    if *way_state != EntityState::Selected {
        *way_state = EntityState::Selected;
        *interface = InterfaceState::WaySelected {
            way: parent.get(),
            surface: trigger.entity(),
        };
        changed.send(EntityStateChanged {
            entity: parent.get(),
            state: EntityState::Selected,
        });
    }
}
