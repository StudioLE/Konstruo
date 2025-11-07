use super::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use konstruo_beziers::{ControlType, CubicBezierSpline};
use konstruo_core::EntityExtensions;
use konstruo_ui::*;
use ControlType::*;

/// A control point that manipulates a [`Path`].
#[derive(Component)]
#[require(InheritedVisibility, Transform, EntityState)]
pub struct PathControl {
    /// Type of the control.
    control_type: ControlType,
    /// Index of the Curve in the spline of the [`Path`].
    curve: usize,
}

impl PathControl {
    /// Create a new [`PathControl`].
    #[must_use]
    pub fn new(control_type: ControlType, curve: usize) -> Self {
        Self {
            control_type,
            curve,
        }
    }

    /// Update the [`Transform`] when a control is moved.
    pub(super) fn on_control_moved(
        mut events: EventReader<ControlMoved>,
        mut controls: Query<(&PathControl, &ChildOf, &mut Transform)>,
    ) {
        for event in events.read() {
            for (control, child_of, mut transform) in &mut controls {
                if child_of.parent() != event.path {
                    continue;
                }
                let Some(translation) = event
                    .spline
                    .get_control(control.control_type, control.curve)
                else {
                    warn!("Failed to set PathControl transform. Control does not exist");
                    continue;
                };
                *transform = Transform::from_translation(translation);
            }
        }
    }

    /// Re-spawn [`PathControl`] when a curve is added or removed.
    pub(super) fn on_curve_added(
        mut events: EventReader<CurveAdded>,
        commands: Commands,
        controls: Query<(Entity, &ChildOf), With<PathControl>>,
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
            event
                .path
                .despawn_children(&mut factory.commands, &controls);
            factory.spawn_controls(&event.spline, event.path, Visibility::Visible);
        }
    }
}

impl PathFactory<'_> {
    /// Spawn [`PathControl`] for each control point in a [`Path`]
    pub(super) fn spawn_controls(
        &mut self,
        spline: &CubicBezierSpline,
        path: Entity,
        visibility: Visibility,
    ) {
        for (curve, bezier) in spline.get_curves().iter().enumerate() {
            let mut bundles = Vec::new();
            if curve == 0 {
                bundles.push(self.control_bundle(
                    Start,
                    curve,
                    bezier.get_control(Start),
                    visibility,
                ));
            }
            bundles.push(self.control_bundle(
                StartHandle,
                curve,
                bezier.get_control(StartHandle),
                visibility,
            ));
            bundles.push(self.control_bundle(
                EndHandle,
                curve,
                bezier.get_control(EndHandle),
                visibility,
            ));
            bundles.push(self.control_bundle(End, curve, bezier.get_control(End), visibility));
            for bundle in bundles {
                self.commands
                    .spawn(bundle)
                    .insert(ChildOf(path))
                    .observe(on_pointer_over)
                    .observe(on_pointer_out)
                    .observe(on_pointer_drag_start)
                    .observe(on_pointer_drag)
                    .observe(on_pointer_drag_end);
            }
        }
    }

    /// Create a bundle for a [`PathControl`].
    #[must_use]
    pub fn control_bundle(
        &mut self,
        control_type: ControlType,
        curve: usize,
        position: Vec3,
        visibility: Visibility,
    ) -> impl Bundle {
        let mesh = match control_type {
            Start | End => self.path_meshes.control_origin.clone(),
            _ => self.path_meshes.control_handle.clone(),
        };
        (
            PathControl::new(control_type, curve),
            OnEntityState::new(1, vec![EntityState::Selected]),
            Transform::from_translation(position),
            Mesh3d(mesh),
            MeshMaterial3d(self.materials.control_node.clone()),
            visibility,
        )
    }
}

fn on_pointer_over(
    trigger: Trigger<Pointer<Over>>,
    materials: Res<PathMaterials>,
    mut query: Query<(&EntityState, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    let Ok((state, mut material)) = query.get_mut(trigger.target()) else {
        error!("Failed to get material of PathControl");
        return;
    };
    if state != &EntityState::Selected {
        *material = MeshMaterial3d(materials.control_node_over.clone());
    }
}

fn on_pointer_out(
    trigger: Trigger<Pointer<Out>>,
    materials: Res<PathMaterials>,
    mut query: Query<(&EntityState, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    let Ok((state, mut material)) = query.get_mut(trigger.target()) else {
        error!("Failed to get PathControl");
        return;
    };
    if state != &EntityState::Selected {
        *material = MeshMaterial3d(materials.control_node.clone());
    }
}

fn on_pointer_drag_start(
    trigger: Trigger<Pointer<DragStart>>,
    materials: Res<PathMaterials>,
    mut query: Query<(&mut EntityState, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    if trigger.button != PointerButton::Primary {
        return;
    }
    let Ok((mut state, mut material)) = query.get_mut(trigger.target()) else {
        error!("Failed to get PathControl");
        return;
    };
    *state = EntityState::Selected;
    *material = MeshMaterial3d(materials.control_node_drag.clone());
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn on_pointer_drag(
    trigger: Trigger<Pointer<Drag>>,
    mut event_writer: EventWriter<ControlMoved>,
    controls: Query<(&PathControl, &ChildOf, &mut Transform)>,
    mut paths: Query<(&mut Path, Entity)>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
) {
    if trigger.button != PointerButton::Primary {
        return;
    }
    let Ok((control, child_of, _transform)) = controls.get(trigger.target()) else {
        error!("Failed to get PathControl");
        return;
    };
    let Ok(translation) = Cursor::from_window(&window, &camera) else {
        warn!("Failed to get cursor on ground");
        return;
    };
    let Ok((mut path, entity)) = paths.get_mut(child_of.parent()) else {
        warn!("Failed to get Path");
        return;
    };
    path.spline
        .update_control(control.control_type, control.curve, translation);
    event_writer.write(ControlMoved {
        path: entity,
        spline: path.spline.clone(),
    });
}

fn on_pointer_drag_end(
    trigger: Trigger<Pointer<DragEnd>>,
    materials: Res<PathMaterials>,
    mut query: Query<(&mut EntityState, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    if trigger.button != PointerButton::Primary {
        return;
    }
    let Ok((mut state, mut material)) = query.get_mut(trigger.target()) else {
        error!("Failed to get PathControl");
        return;
    };
    *state = EntityState::Default;
    *material = MeshMaterial3d(materials.control_node.clone());
}
