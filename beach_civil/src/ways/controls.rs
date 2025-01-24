use crate::ways::materials::WayMaterials;
use crate::ways::meshes::WayMeshes;
use crate::ways::{Way, WaySurface};
use beach_ui::cursor::Cursor;
use beach_ui::pan_orbit::PrimaryCamera;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use ControlType::*;

/// A control point that manipulates a [`Way`].
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct WayControl {
    /// Index of the cubic bezier within the cubic bezier spline of the [`Way`].
    #[allow(dead_code)]
    id: usize,
    /// Type of the control.
    #[allow(dead_code)]
    control: ControlType,
    /// Translation at the start of the drag operation.
    drag: Option<Vec3>,
}

pub enum ControlType {
    Start,
    StartHandle,
    EndHandle,
    End,
}

impl WayControl {
    fn new(curve: usize, control: ControlType) -> Self {
        Self {
            id: curve,
            control,
            drag: None,
        }
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
            let start = if i == 0 {
                Some((
                    WayControl::new(i * 4, Start),
                    Transform::from_translation(bezier.start),
                    Mesh3d(meshes.control_origin.clone()),
                    MeshMaterial3d(materials.control_node.clone()),
                ))
            } else {
                None
            };
            let start_handle = (
                WayControl::new(i * 4 + 1, StartHandle),
                Transform::from_translation(bezier.start_handle),
                Mesh3d(meshes.control_handle.clone()),
                MeshMaterial3d(materials.control_node.clone()),
            );
            let end_handle = (
                WayControl::new(i * 4 + 2, EndHandle),
                Transform::from_translation(bezier.end_handle),
                Mesh3d(meshes.control_handle.clone()),
                MeshMaterial3d(materials.control_node.clone()),
            );
            let end = (
                WayControl::new(i * 4 + 3, End),
                Transform::from_translation(bezier.end),
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

#[allow(clippy::too_many_arguments)]
fn on_pointer_drag(
    event: Trigger<Pointer<Drag>>,
    mut controls: Query<(&WayControl, &Parent, &mut Transform)>,
    mut ways: Query<(&mut Way, Entity, &mut Mesh3d)>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: Res<WayMaterials>,
    surfaces: Query<(Entity, &Parent), With<WaySurface>>,
) {
    let Ok((control, parent, mut transform)) = controls.get_mut(event.entity()) else {
        error!("Failed to get WayControl");
        return;
    };
    let Ok(translation) = Cursor::on_ground(&window, &camera) else {
        warn!("Failed to get cursor on ground");
        return;
    };
    transform.translation = translation;
    let Ok((mut way, entity, mesh)) = ways.get_mut(parent.get()) else {
        warn!("Failed to get Way");
        return;
    };
    way.spline.update_control(control.id, translation);
    Way::regenerate(
        &mut commands,
        meshes,
        materials,
        surfaces,
        &way,
        entity,
        mesh,
    );
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
