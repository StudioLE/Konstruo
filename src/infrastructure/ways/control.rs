use super::*;
use crate::beziers::ControlType;
use crate::mathematics::QUARTER_PI;
use crate::ui::{Cursor, EntityState};
use crate::ui::{EntityStateChanged, PrimaryCamera};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use ControlType::*;

/// A control point that manipulates a [`Way`].
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct WayControl {
    /// Type of the control.
    control_type: ControlType,
    /// Index of the Curve in the spline of the [`Way`].
    curve: usize,
    /// Translation at the start of the drag operation.
    drag: Option<Vec3>,
}

impl WayControl {
    /// Create a new [`WayControl`].
    #[must_use]
    pub fn new(control_type: ControlType, curve: usize) -> Self {
        Self {
            control_type,
            curve,
            drag: None,
        }
    }

    /// Create a bundle for a [`WayControl`].
    #[must_use]
    pub fn bundle(
        meshes: &Res<WayMeshes>,
        materials: &Res<WayMaterials>,
        control_type: ControlType,
        curve: usize,
        position: Vec3,
    ) -> (
        WayControl,
        Transform,
        Mesh3d,
        MeshMaterial3d<StandardMaterial>,
    ) {
        let mesh = match control_type {
            Start | End => meshes.control_origin.clone(),
            _ => meshes.control_handle.clone(),
        };
        (
            WayControl::new(control_type, curve),
            get_transform(control_type, position),
            Mesh3d(mesh),
            MeshMaterial3d(materials.control_node.clone()),
        )
    }

    /// Factory method to spawn [`WayControl`] for each control point in a [`Way`]
    pub(super) fn spawn(
        commands: &mut Commands,
        meshes: &Res<WayMeshes>,
        materials: &Res<WayMaterials>,
        way: &Way,
        parent: Entity,
    ) {
        for (curve, bezier) in way.spline.get_curves().iter().enumerate() {
            let mut bundles = Vec::new();
            if curve == 0 {
                bundles.push(Self::bundle(meshes, materials, Start, curve, bezier.start));
            }
            bundles.push(Self::bundle(
                meshes,
                materials,
                StartHandle,
                curve,
                bezier.start_handle,
            ));
            bundles.push(Self::bundle(
                meshes,
                materials,
                EndHandle,
                curve,
                bezier.end_handle,
            ));
            bundles.push(Self::bundle(meshes, materials, End, curve, bezier.end));
            for bundle in bundles {
                commands
                    .spawn(bundle)
                    .set_parent(parent)
                    .observe(on_pointer_over)
                    .observe(on_pointer_out)
                    .observe(on_pointer_drag_start)
                    .observe(on_pointer_drag)
                    .observe(on_pointer_drag_end);
            }
        }
    }

    /// Update the controls when the spline changes.
    pub(super) fn on_spline_changed(
        mut events: EventReader<SplineChangedEvent>,
        mut controls: Query<(&WayControl, &Parent, &mut Transform)>,
    ) {
        for event in events.read() {
            for (control, parent, mut transform) in &mut controls {
                if parent.get() != event.way {
                    continue;
                }
                let Some(translation) = event
                    .spline
                    .get_control(control.control_type, control.curve)
                else {
                    warn!("Failed to set WayControl transform. Control does not exist");
                    continue;
                };
                *transform = get_transform(control.control_type, translation);
            }
        }
    }

    /// Update the [`WayControl`] visibility when the [`EntityState`] of the [`Way`] changes.
    pub(super) fn on_state_changed(
        mut events: EventReader<EntityStateChanged>,
        mut controls: Query<(&Parent, &mut Visibility), With<WayControl>>,
    ) {
        for event in events.read() {
            for (parent, mut visibility) in &mut controls {
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
    materials: Res<WayMaterials>,
    mut query: Query<(&WayControl, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    let Ok((control, mut material)) = query.get_mut(event.entity()) else {
        error!("Failed to get material of WayControl");
        return;
    };
    if control.drag.is_none() {
        *material = MeshMaterial3d(materials.control_node_over.clone());
    }
}

fn on_pointer_out(
    event: Trigger<Pointer<Out>>,
    materials: Res<WayMaterials>,
    mut query: Query<(&WayControl, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    let Ok((control, mut material)) = query.get_mut(event.entity()) else {
        error!("Failed to get WayControl");
        return;
    };
    if control.drag.is_none() {
        *material = MeshMaterial3d(materials.control_node.clone());
    }
}

fn on_pointer_drag_start(
    event: Trigger<Pointer<DragStart>>,
    materials: Res<WayMaterials>,
    mut query: Query<(
        &mut WayControl,
        &Transform,
        &mut MeshMaterial3d<StandardMaterial>,
    )>,
) {
    let Ok((mut control, transform, mut material)) = query.get_mut(event.entity()) else {
        error!("Failed to get WayControl");
        return;
    };
    control.drag = Some(transform.translation);
    *material = MeshMaterial3d(materials.control_node_drag.clone());
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn on_pointer_drag(
    event: Trigger<Pointer<Drag>>,
    mut event_writer: EventWriter<SplineChangedEvent>,
    controls: Query<(&WayControl, &Parent, &mut Transform)>,
    mut ways: Query<(&mut Way, Entity)>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
) {
    let Ok((control, parent, _transform)) = controls.get(event.entity()) else {
        error!("Failed to get WayControl");
        return;
    };
    let Ok(translation) = Cursor::from_window(&window, &camera) else {
        warn!("Failed to get cursor on ground");
        return;
    };
    let Ok((mut way, entity)) = ways.get_mut(parent.get()) else {
        warn!("Failed to get Way");
        return;
    };
    way.spline
        .update_control(control.control_type, control.curve, translation);
    event_writer.send(SplineChangedEvent {
        way: entity,
        spline: way.spline.clone(),
    });
}

fn on_pointer_drag_end(
    event: Trigger<Pointer<DragEnd>>,
    materials: Res<WayMaterials>,
    mut query: Query<(&mut WayControl, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    let Ok((mut control, mut material)) = query.get_mut(event.entity()) else {
        error!("Failed to get WayControl");
        return;
    };
    control.drag = None;
    *material = MeshMaterial3d(materials.control_node.clone());
}
