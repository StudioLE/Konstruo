use super::*;
use crate::geometry::{Sweep, Vec6};
use crate::ui::{EntityState, EntityStateChanged, InterfaceState};
use crate::GROUND_HEIGHT;
use bevy::prelude::*;

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
        let bundle = (
            self,
            Mesh3d(meshes.add(sweep.clone().to_triangle_list().to_mesh())),
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
        let edges = sweep.get_edges();
        let material = materials.surface_edge_over.clone();
        for (index, edge) in edges.into_iter().enumerate() {
            let bundle = (
                WaySurfaceEdge {
                    index,
                    way: way_entity,
                },
                Visibility::Hidden,
                Mesh3d(meshes.add(edge.to_mesh())),
                MeshMaterial3d(material.clone()),
            );
            commands.spawn(bundle).set_parent(surface_entity);
        }
    }

    /// Update the mesh geometry when the spline changes.
    pub(super) fn on_spline_changed(
        mut events: EventReader<SplineChangedEvent>,
        mut surfaces: Query<(Entity, &WaySurface, &Parent, &mut Mesh3d)>,
        mut edges: Query<
            (&WaySurfaceEdge, &Parent, &mut Mesh3d),
            (With<WaySurfaceEdge>, Without<WaySurface>),
        >,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        for event in events.read() {
            for (entity, surface, parent, mut mesh) in &mut surfaces {
                if parent.get() != event.way {
                    continue;
                }
                let sweep = Sweep::new(&event.spline, surface.offsets);
                let sweep_edges = sweep.get_edges();
                *mesh = Mesh3d(meshes.add(sweep.to_triangle_list().to_mesh()));
                for (edge, parent, mut mesh) in &mut edges {
                    if parent.get() != entity {
                        continue;
                    }
                    let polyline = sweep_edges
                        .get(edge.index)
                        .expect("edge index should exist")
                        .clone()
                        .to_mesh();
                    *mesh = Mesh3d(meshes.add(polyline));
                }
            }
        }
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
