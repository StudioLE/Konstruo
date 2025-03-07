use super::*;
use crate::beziers::ControlType::*;
use crate::beziers::CubicBezierSpline;
use crate::geometry::Polyline;
use crate::ui::{EntityState, EntityStateChanged};
use bevy::prelude::*;

/// A line between control points of a [`Way`].
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
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
        spline: &CubicBezierSpline,
        way: Entity,
    ) {
        for (i, bezier) in spline.get_curves().iter().enumerate() {
            let i = i * 4;
            let line = vec![bezier.get_control(Start), bezier.get_control(StartHandle)];
            let start = (
                WayControlLine::new(i, i + 1),
                Mesh3d(meshes.add(Polyline::new(line).to_mesh())),
                MeshMaterial3d(materials.control_line.clone()),
            );
            let line = vec![bezier.get_control(End), bezier.get_control(EndHandle)];
            let end = (
                WayControlLine::new(i + 3, i + 2),
                Mesh3d(meshes.add(Polyline::new(line).to_mesh())),
                MeshMaterial3d(materials.control_line.clone()),
            );
            commands.spawn(start).set_parent(way);
            commands.spawn(end).set_parent(way);
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

    /// Update the [`Transform`] when a control is moved.
    pub(super) fn on_control_moved(
        mut events: EventReader<ControlMoved>,
        mut lines: Query<(&WayControlLine, &Parent, &mut Mesh3d), Without<Way>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        for event in events.read() {
            for (line, parent, mut mesh) in &mut lines {
                if parent.get() != event.way {
                    continue;
                }
                // TODO: Update this to remove get_controls()
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
                *mesh = Mesh3d(meshes.add(Polyline::new(vec![*anchor, *handle]).to_mesh()));
            }
        }
    }

    /// Re-spawn [`WayControlLine`] when a curve is added or removed.
    pub(super) fn on_curve_added(
        mut events: EventReader<CurveAdded>,
        lines: Query<(Entity, &Parent), With<WayControlLine>>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        materials: Res<WayMaterials>,
    ) {
        for event in events.read() {
            for (entity, parent) in lines.iter() {
                if parent.get() != event.way {
                    continue;
                }
                commands.entity(entity).despawn();
            }
            WayControlLine::spawn(
                &mut commands,
                &mut meshes,
                &materials,
                &event.spline,
                event.way,
            );
        }
    }
}
