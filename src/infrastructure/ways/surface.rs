use super::*;
use crate::geometry::{Sweep, Vec6};
use crate::ui::{EntityState, InterfaceState};
use crate::GROUND_HEIGHT;
use bevy::prelude::*;

/// A surface formed by two lines from a [Way].
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
    pub fn centered(depth: f32, width: f32, purpose: SurfaceType) -> Self {
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
        parent: Entity,
    ) {
        let mesh = self.get_mesh(way);
        let material = materials.get_surface(&self.purpose);
        let bundle = (
            self,
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(material),
            Transform::from_translation(Vec3::new(0.0, 0.0, GROUND_HEIGHT)),
            PickingBehavior::default(),
        );
        commands
            .spawn(bundle)
            .observe(on_pointer_over)
            .observe(on_pointer_out)
            .observe(on_pointer_click)
            .set_parent(parent);
    }

    /// Update the material when the way is hovered.
    pub(super) fn on_way_state_changed(
        surfaces: &mut Query<
            (&WaySurface, &Parent, &mut MeshMaterial3d<StandardMaterial>),
            (With<WaySurface>, Without<Way>),
        >,
        materials: &Res<WayMaterials>,
        way_entity: Entity,
        state: &EntityState,
    ) {
        for (surface, parent, mut material) in surfaces {
            if parent.get() != way_entity {
                continue;
            }
            let handle = match state {
                EntityState::Hovered => materials.surface_over.clone(),
                _ => materials.get_surface(&surface.purpose),
            };
            *material = MeshMaterial3d(handle);
        }
    }

    /// Update the mesh geometry when the spline changes.
    pub(super) fn on_spline_changed(
        mut surfaces: Query<
            (&WaySurface, &Parent, &mut Mesh3d),
            (Without<Way>, Without<WayControlLine>),
        >,
        meshes: &mut ResMut<Assets<Mesh>>,
        way: &Way,
        way_entity: Entity,
    ) {
        for (surface, parent, mut mesh) in &mut surfaces {
            if parent.get() != way_entity {
                continue;
            }
            *mesh = Mesh3d(meshes.add(surface.get_mesh(way)));
        }
    }

    /// Get the [`Mesh`].
    fn get_mesh(&self, way: &Way) -> Mesh {
        Sweep::new(&way.spline, self.offsets)
            .to_triangle_list()
            .to_mesh()
    }
}

fn on_pointer_over(
    event: Trigger<Pointer<Over>>,
    mut ways: Query<&mut EntityState, With<Way>>,
    surfaces: Query<&Parent, (With<WaySurface>, Without<Way>)>,
) {
    let Ok(parent) = surfaces.get(event.entity()) else {
        error!("Failed to get parent of WaySurface");
        return;
    };
    let Ok(mut state) = ways.get_mut(parent.get()) else {
        warn!("Failed to get Way");
        return;
    };
    if *state != EntityState::Selected {
        *state = EntityState::Hovered;
    }
}

fn on_pointer_out(
    event: Trigger<Pointer<Out>>,
    mut ways: Query<&mut EntityState, With<Way>>,
    surfaces: Query<&Parent, (With<WaySurface>, Without<Way>)>,
) {
    let Ok(parent) = surfaces.get(event.entity()) else {
        error!("Failed to get parent of WaySurface");
        return;
    };
    let Ok(mut state) = ways.get_mut(parent.get()) else {
        warn!("Failed to get Way");
        return;
    };
    if *state != EntityState::Selected {
        *state = EntityState::Default;
    }
}

fn on_pointer_click(
    event: Trigger<Pointer<Click>>,
    surfaces: Query<&Parent, (With<WaySurface>, Without<Way>)>,
    mut ways: Query<&mut EntityState, With<Way>>,
    mut interface_state: EventWriter<InterfaceState>,
) {
    trace!("WaySurface clicked");
    let Ok(parent) = surfaces.get(event.entity()) else {
        error!("Failed to get parent of WaySurface");
        return;
    };
    let Ok(mut way_state) = ways.get_mut(parent.get()) else {
        warn!("Failed to get Way");
        return;
    };
    interface_state.send(InterfaceState::WaySelected {
        way: parent.get(),
        surface: event.entity(),
    });
    *way_state = EntityState::Selected;
}
