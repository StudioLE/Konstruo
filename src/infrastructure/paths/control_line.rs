use super::*;
use crate::beziers::ControlType::*;
use crate::beziers::CubicBezierSpline;
use crate::geometry::Polyline;
use crate::ui::{EntityState, EntityStateChanged};
use crate::Helpers;
use bevy::prelude::*;

/// A line between control points of a [`Path`].
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct PathControlLine {
    /// Index of the Curve in the spline of the [`Path`].
    curve: usize,
    /// Is this a start or end.
    is_start: bool,
}

impl PathControlLine {
    /// Create a new [`PathControlLine`]
    #[must_use]
    pub fn new(curve: usize, is_start: bool) -> Self {
        Self { curve, is_start }
    }

    /// Factory method to spawn [`PathControlLine`] for each control point in a [`Path`]
    pub(super) fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &Res<PathMaterials>,
        spline: &CubicBezierSpline,
        path: Entity,
        visibility: Visibility,
    ) {
        for (curve, bezier) in spline.get_curves().iter().enumerate() {
            let line = vec![bezier.get_control(Start), bezier.get_control(StartHandle)];
            let start = (
                PathControlLine::new(curve, true),
                Mesh3d(meshes.add(Polyline::new(line).to_mesh())),
                MeshMaterial3d(materials.control_line.clone()),
                visibility,
            );
            let line = vec![bezier.get_control(End), bezier.get_control(EndHandle)];
            let end = (
                PathControlLine::new(curve, false),
                Mesh3d(meshes.add(Polyline::new(line).to_mesh())),
                MeshMaterial3d(materials.control_line.clone()),
                visibility,
            );
            commands.spawn(start).set_parent(path);
            commands.spawn(end).set_parent(path);
        }
    }

    /// Update the [`PathControlLine`] visibility when the [`EntityState`] of the [`Path`] changes.
    pub(super) fn on_state_changed(
        mut events: EventReader<EntityStateChanged>,
        mut lines: Query<(&Parent, &mut Visibility), With<PathControlLine>>,
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
        mut lines: Query<(&PathControlLine, &Parent, &mut Mesh3d), Without<Path>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        for event in events.read() {
            for (line, parent, mut mesh) in &mut lines {
                if parent.get() != event.path {
                    continue;
                }
                let anchor = if line.is_start { Start } else { End };
                let handle = if line.is_start {
                    StartHandle
                } else {
                    EndHandle
                };
                let line = vec![
                    event
                        .spline
                        .get_control(anchor, line.curve)
                        .expect("control should exist"),
                    event
                        .spline
                        .get_control(handle, line.curve)
                        .expect("control should exist"),
                ];
                *mesh = Mesh3d(meshes.add(Polyline::new(line).to_mesh()));
            }
        }
    }

    /// Re-spawn [`PathControlLine`] when a curve is added or removed.
    pub(super) fn on_curve_added(
        mut events: EventReader<CurveAdded>,
        lines: Query<(Entity, &Parent), With<PathControlLine>>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        materials: Res<PathMaterials>,
    ) {
        for event in events.read() {
            Helpers::despawn_children(&mut commands, &lines, event.path);
            PathControlLine::spawn(
                &mut commands,
                &mut meshes,
                &materials,
                &event.spline,
                event.path,
                Visibility::Visible,
            );
        }
    }
}
