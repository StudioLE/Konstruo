use super::*;
use crate::beziers::ControlType::*;
use crate::beziers::CubicBezierSpline;
use crate::extensions::*;
use crate::geometry::Polyline;
use crate::ui::{EntityState, EntityStateChanged};
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

    /// Update the [`PathControlLine`] visibility when the [`EntityState`] of the [`Path`] changes.
    pub(super) fn on_state_changed(
        mut events: EventReader<EntityStateChanged>,
        mut lines: Query<(&ChildOf, &mut Visibility), With<PathControlLine>>,
    ) {
        for event in events.read() {
            for (child_of, mut visibility) in &mut lines {
                if child_of.parent != event.entity {
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
        mut lines: Query<(&PathControlLine, &ChildOf, &mut Mesh3d), Without<Path>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        for event in events.read() {
            for (line, child_of, mut mesh) in &mut lines {
                if child_of.parent != event.path {
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
        lines: Query<(Entity, &ChildOf), With<PathControlLine>>,
        commands: Commands,
        meshes: ResMut<Assets<Mesh>>,
        path_meshes: Res<PathMeshes>,
        materials: Res<PathMaterials>,
    ) {
        let mut factory = PathFactory {
            commands,
            meshes,
            path_meshes,
            materials,
        };
        for event in events.read() {
            event.path.despawn_children(&mut factory.commands, &lines);
            factory.spawn_control_lines(&event.spline, event.path, Visibility::Visible);
        }
    }
}

impl PathFactory<'_> {
    /// Spawn [`PathControlLine`] between the two sets of control point in a [`Path`]
    pub(super) fn spawn_control_lines(
        &mut self,
        spline: &CubicBezierSpline,
        path: Entity,
        visibility: Visibility,
    ) {
        for (curve, bezier) in spline.get_curves().iter().enumerate() {
            let line = vec![bezier.get_control(Start), bezier.get_control(StartHandle)];
            let start = (
                PathControlLine::new(curve, true),
                Mesh3d(self.meshes.add(Polyline::new(line).to_mesh())),
                MeshMaterial3d(self.materials.control_line.clone()),
                visibility,
                ChildOf { parent: path },
            );
            let line = vec![bezier.get_control(End), bezier.get_control(EndHandle)];
            let end = (
                PathControlLine::new(curve, false),
                Mesh3d(self.meshes.add(Polyline::new(line).to_mesh())),
                MeshMaterial3d(self.materials.control_line.clone()),
                visibility,
                ChildOf { parent: path },
            );
            self.commands.spawn(start);
            self.commands.spawn(end);
        }
    }
}
