use super::*;
use crate::geometry::Polyline;
use crate::ui::{EntityState, EntityStateChanged};
use bevy::prelude::*;

/// A line between control points of a [`Way`].
#[derive(Component)]
#[require(InheritedVisibility, Transform, Visibility(|| Visibility::Hidden))]
pub struct WayControlLine {
    /// Index of the anchor in the spline of the [`Way`].
    pub anchor: usize,
    /// Index of the handle in the spline of the [`Way`].
    pub handle: usize,
}

impl WayControlLine {
    /// Create a new [`WayControlLine`]
    fn new(start: usize, end: usize) -> Self {
        Self {
            anchor: start,
            handle: end,
        }
    }

    /// Factory method to spawn [`WayControlLine`] for each control point in a [`Way`]
    pub(super) fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &Res<WayMaterials>,
        way: &Way,
        parent: Entity,
    ) {
        for (i, bezier) in way.spline.curves.iter().enumerate() {
            let i = i * 4;
            let start = (
                WayControlLine::new(i, i + 1),
                Mesh3d(meshes.add(Polyline::new([bezier.start, bezier.start_handle]).to_mesh())),
                MeshMaterial3d(materials.control_line.clone()),
            );
            let end = (
                WayControlLine::new(i + 3, i + 2),
                Mesh3d(meshes.add(Polyline::new([bezier.end, bezier.end_handle]).to_mesh())),
                MeshMaterial3d(materials.control_line.clone()),
            );
            commands.spawn(start).set_parent(parent);
            commands.spawn(end).set_parent(parent);
        }
    }

    /// Update the [`WayControlLine`] visibility when the [`EntityState`] of the [`Way`] changes.
    pub(super) fn on_state_changed(
        mut events: EventReader<EntityStateChanged>,
        mut lines: Query<(&Parent, &mut Visibility), With<WayControlLine>>,
    ) {
        for event in events.read() {
            for (parent, mut visibility) in &mut lines {
                if parent.get() != event.entity {
                    continue;
                }
                *visibility = match event.state {
                    EntityState::Selected => Visibility::Visible,
                    EntityState::Hovered | EntityState::Default => Visibility::Hidden,
                };
            }
        }
    }

    /// Update the control lines when the spline changes.
    pub(super) fn on_spline_changed(
        mut events: EventReader<SplineChangedEvent>,
        mut lines: Query<(&WayControlLine, &Parent, &mut Mesh3d), Without<Way>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        for event in events.read() {
            for (line, parent, mut mesh) in &mut lines {
                if parent.get() != event.way {
                    continue;
                }
                let control_points = event.spline.get_controls();
                let Some(anchor) = control_points.get(line.anchor) else {
                    warn!(
                        "Failed to set WayControlLine. Index does not exist: {}",
                        line.anchor
                    );
                    continue;
                };
                let Some(handle) = control_points.get(line.handle) else {
                    warn!(
                        "Failed to set WayControlLine. Index does not exist: {}",
                        line.handle
                    );
                    continue;
                };
                *mesh = Mesh3d(meshes.add(Polyline::new([*anchor, *handle]).to_mesh()));
            }
        }
    }
}
