use super::*;
use crate::beziers::{ControlType, CubicBezierSpline};
use crate::mathematics::QUARTER_PI;
use crate::ui::{Cursor, EntityState};
use crate::ui::{EntityStateChanged, PrimaryCamera};
use crate::Helpers;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
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

    /// Create a bundle for a [`PathControl`].
    #[must_use]
    pub(crate) fn bundle(
        meshes: &Res<PathMeshes>,
        materials: &Res<PathMaterials>,
        control_type: ControlType,
        curve: usize,
        position: Vec3,
        visibility: Visibility,
    ) -> (
        PathControl,
        Transform,
        Mesh3d,
        MeshMaterial3d<StandardMaterial>,
        Visibility,
    ) {
        let mesh = match control_type {
            Start | End => meshes.control_origin.clone(),
            _ => meshes.control_handle.clone(),
        };
        (
            PathControl::new(control_type, curve),
            get_transform(control_type, position),
            Mesh3d(mesh),
            MeshMaterial3d(materials.control_node.clone()),
            visibility,
        )
    }

    /// Factory method to spawn [`PathControl`] for each control point in a [`Path`]
    pub(super) fn spawn(
        commands: &mut Commands,
        meshes: &Res<PathMeshes>,
        materials: &Res<PathMaterials>,
        spline: &CubicBezierSpline,
        path: Entity,
        visibility: Visibility,
    ) {
        for (curve, bezier) in spline.get_curves().iter().enumerate() {
            let mut bundles = Vec::new();
            if curve == 0 {
                bundles.push(Self::bundle(
                    meshes,
                    materials,
                    Start,
                    curve,
                    bezier.get_control(Start),
                    visibility,
                ));
            }
            bundles.push(Self::bundle(
                meshes,
                materials,
                StartHandle,
                curve,
                bezier.get_control(StartHandle),
                visibility,
            ));
            bundles.push(Self::bundle(
                meshes,
                materials,
                EndHandle,
                curve,
                bezier.get_control(EndHandle),
                visibility,
            ));
            bundles.push(Self::bundle(
                meshes,
                materials,
                End,
                curve,
                bezier.get_control(End),
                visibility,
            ));
            for bundle in bundles {
                commands
                    .spawn(bundle)
                    .set_parent(path)
                    .observe(on_pointer_over)
                    .observe(on_pointer_out)
                    .observe(on_pointer_drag_start)
                    .observe(on_pointer_drag)
                    .observe(on_pointer_drag_end);
            }
        }
    }

    /// Update the [`Transform`] when a control is moved.
    pub(super) fn on_control_moved(
        mut events: EventReader<ControlMoved>,
        mut controls: Query<(&PathControl, &Parent, &mut Transform)>,
    ) {
        for event in events.read() {
            for (control, parent, mut transform) in &mut controls {
                if parent.get() != event.path {
                    continue;
                }
                let Some(translation) = event
                    .spline
                    .get_control(control.control_type, control.curve)
                else {
                    warn!("Failed to set PathControl transform. Control does not exist");
                    continue;
                };
                *transform = get_transform(control.control_type, translation);
            }
        }
    }

    /// Re-spawn [`PathControl`] when a curve is added or removed.
    pub(super) fn on_curve_added(
        mut events: EventReader<CurveAdded>,
        mut commands: Commands,
        controls: Query<(Entity, &Parent), With<PathControl>>,
        meshes: Res<PathMeshes>,
        materials: Res<PathMaterials>,
    ) {
        for event in events.read() {
            Helpers::despawn_children(&mut commands, &controls, event.path);
            PathControl::spawn(
                &mut commands,
                &meshes,
                &materials,
                &event.spline,
                event.path,
                Visibility::Visible,
            );
        }
    }

    /// Update the [`PathControl`] visibility when the [`EntityState`] of the [`Path`] changes.
    pub(super) fn on_state_changed(
        mut events: EventReader<EntityStateChanged>,
        mut controls: Query<(&Parent, &mut Visibility), With<PathControl>>,
    ) {
        for event in events.read() {
            for (parent, mut visibility) in &mut controls {
                if parent.get() != event.entity {
                    continue;
                }
                *visibility = match event.state {
                    EntityState::Selected => Visibility::Visible,
                    _ => Visibility::Hidden,
                };
            }
        }
    }
}

fn get_transform(control_type: ControlType, position: Vec3) -> Transform {
    match control_type {
        Start | End => {
            Transform::from_translation(position).with_rotation(Quat::from_rotation_z(QUARTER_PI))
        }
        _ => Transform::from_translation(position),
    }
}

fn on_pointer_over(
    event: Trigger<Pointer<Over>>,
    materials: Res<PathMaterials>,
    mut query: Query<(&EntityState, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    let Ok((state, mut material)) = query.get_mut(event.entity()) else {
        error!("Failed to get material of PathControl");
        return;
    };
    if state != &EntityState::Selected {
        *material = MeshMaterial3d(materials.control_node_over.clone());
    }
}

fn on_pointer_out(
    event: Trigger<Pointer<Out>>,
    materials: Res<PathMaterials>,
    mut query: Query<(&EntityState, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    let Ok((state, mut material)) = query.get_mut(event.entity()) else {
        error!("Failed to get PathControl");
        return;
    };
    if state != &EntityState::Selected {
        *material = MeshMaterial3d(materials.control_node.clone());
    }
}

fn on_pointer_drag_start(
    event: Trigger<Pointer<DragStart>>,
    materials: Res<PathMaterials>,
    mut query: Query<(&mut EntityState, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    let Ok((mut state, mut material)) = query.get_mut(event.entity()) else {
        error!("Failed to get PathControl");
        return;
    };
    *state = EntityState::Selected;
    *material = MeshMaterial3d(materials.control_node_drag.clone());
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn on_pointer_drag(
    event: Trigger<Pointer<Drag>>,
    mut event_writer: EventWriter<ControlMoved>,
    controls: Query<(&PathControl, &Parent, &mut Transform)>,
    mut paths: Query<(&mut Path, Entity)>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
) {
    let Ok((control, parent, _transform)) = controls.get(event.entity()) else {
        error!("Failed to get PathControl");
        return;
    };
    let Ok(translation) = Cursor::from_window(&window, &camera) else {
        warn!("Failed to get cursor on ground");
        return;
    };
    let Ok((mut path, entity)) = paths.get_mut(parent.get()) else {
        warn!("Failed to get Path");
        return;
    };
    path.spline
        .update_control(control.control_type, control.curve, translation);
    event_writer.send(ControlMoved {
        path: entity,
        spline: path.spline.clone(),
    });
}

fn on_pointer_drag_end(
    event: Trigger<Pointer<DragEnd>>,
    materials: Res<PathMaterials>,
    mut query: Query<(&mut EntityState, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    let Ok((mut state, mut material)) = query.get_mut(event.entity()) else {
        error!("Failed to get PathControl");
        return;
    };
    *state = EntityState::Selected;
    *material = MeshMaterial3d(materials.control_node.clone());
}
