use super::*;
use crate::mathematics::QUARTER_PI;
use crate::ui::PrimaryCamera;
use crate::ui::{Cursor, EntityState};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// A control point that manipulates a [`Way`].
#[derive(Component)]
#[require(InheritedVisibility, Transform, Visibility(|| Visibility::Hidden))]
pub struct WayControl {
    /// Index of the control point in the spline of the [`Way`].
    pub index: usize,
    /// Translation at the start of the drag operation.
    drag: Option<Vec3>,
}

impl WayControl {
    fn new(index: usize) -> Self {
        Self { index, drag: None }
    }

    /// Factory method to spawn [`WayControl`] for each control point in a [`Way`]
    pub(super) fn spawn(
        commands: &mut Commands,
        meshes: &Res<WayMeshes>,
        materials: &Res<WayMaterials>,
        way: &Way,
        parent: Entity,
    ) {
        for (i, bezier) in way.spline.curves.iter().enumerate() {
            let start = (i == 0).then(|| {
                (
                    WayControl::new(i * 4),
                    Transform::from_translation(bezier.start)
                        .with_rotation(Quat::from_rotation_z(QUARTER_PI)),
                    Mesh3d(meshes.control_origin.clone()),
                    MeshMaterial3d(materials.control_node.clone()),
                )
            });
            let start_handle = (
                WayControl::new(i * 4 + 1),
                Transform::from_translation(bezier.start_handle),
                Mesh3d(meshes.control_handle.clone()),
                MeshMaterial3d(materials.control_node.clone()),
            );
            let end_handle = (
                WayControl::new(i * 4 + 2),
                Transform::from_translation(bezier.end_handle),
                Mesh3d(meshes.control_handle.clone()),
                MeshMaterial3d(materials.control_node.clone()),
            );
            let end = (
                WayControl::new(i * 4 + 3),
                Transform::from_translation(bezier.end)
                    .with_rotation(Quat::from_rotation_z(QUARTER_PI)),
                Mesh3d(meshes.control_origin.clone()),
                MeshMaterial3d(materials.control_node.clone()),
            );
            if let Some(start) = start {
                commands
                    .spawn(start)
                    .set_parent(parent)
                    .observe(on_pointer_over)
                    .observe(on_pointer_out)
                    .observe(on_pointer_drag_start)
                    .observe(on_pointer_drag)
                    .observe(on_pointer_drag_end);
            }
            commands
                .spawn(start_handle)
                .set_parent(parent)
                .observe(on_pointer_over)
                .observe(on_pointer_out)
                .observe(on_pointer_drag_start)
                .observe(on_pointer_drag)
                .observe(on_pointer_drag_end);
            commands
                .spawn(end_handle)
                .set_parent(parent)
                .observe(on_pointer_over)
                .observe(on_pointer_out)
                .observe(on_pointer_drag_start)
                .observe(on_pointer_drag)
                .observe(on_pointer_drag_end);
            commands
                .spawn(end)
                .set_parent(parent)
                .observe(on_pointer_over)
                .observe(on_pointer_out)
                .observe(on_pointer_drag_start)
                .observe(on_pointer_drag)
                .observe(on_pointer_drag_end);
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
                if let Some(translation) = event.spline.get_controls().get(control.index) {
                    let index = control.index % 4;
                    if index == 0 || index == 3 {
                        *transform = Transform::from_translation(*translation)
                            .with_rotation(Quat::from_rotation_z(QUARTER_PI));
                    } else {
                        *transform = Transform::from_translation(*translation);
                    }
                } else {
                    warn!(
                        "Failed to set WayControl transform. Index does not exist: {}",
                        control.index
                    );
                };
            }
        }
    }

    /// Update the [`WayControl`] visibility when the [`EntityState`] of the [`Way`] changes.
    pub(super) fn on_state_changed(
        mut events: EventReader<StateChangedEvent>,
        mut controls: Query<(&Parent, &mut Visibility), With<WayControl>>,
    ) {
        for event in events.read() {
            for (parent, mut visibility) in &mut controls {
                if parent.get() != event.way {
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
    let Ok(translation) = Cursor::on_ground(&window, &camera) else {
        warn!("Failed to get cursor on ground");
        return;
    };
    let Ok((mut way, entity)) = ways.get_mut(parent.get()) else {
        warn!("Failed to get Way");
        return;
    };
    way.spline.update_control(control.index, translation);
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
