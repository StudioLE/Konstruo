use crate::beziers::CubicBezierSpline;
use crate::geometry::vectors::is_almost_equal_to;
use crate::infrastructure::*;
use crate::ui::*;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::cmp::Ordering;

#[derive(Default, Resource)]
pub struct Drawing {
    origins: Vec<Vec3>,
    handles: Vec<Vec3>,
    entity: Option<Entity>,
    needs_update: bool,
}

impl Drawing {
    /// System to update a [`Way`]
    #[allow(clippy::too_many_arguments)]
    pub(super) fn update_system(
        interface: Res<InterfaceState>,
        mut commands: Commands,
        mut drawing: ResMut<Drawing>,
        mut meshes: ResMut<Assets<Mesh>>,
        way_meshes: Res<WayMeshes>,
        materials: Res<WayMaterials>,
        mut ways: Query<&mut Way>,
        mut curve_added: EventWriter<CurveAdded>,
        motion: EventReader<MouseMotion>,
        window: Query<&Window, With<PrimaryWindow>>,
        camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    ) {
        if *interface != InterfaceState::DrawWay
            || drawing.handles.is_empty()
            || (!drawing.needs_update && motion.is_empty())
        {
            return;
        }
        drawing.needs_update = false;
        let mut origins = drawing.origins.clone();
        let mut handles = drawing.handles.clone();
        if let Ok(cursor) = Cursor::from_window(&window, &camera) {
            match origins.len().cmp(&handles.len()) {
                Ordering::Less => {
                    unreachable!("Origins count should always be greater than handles");
                }
                Ordering::Equal => {
                    origins.push(cursor);
                }
                Ordering::Greater => {
                    handles.push(cursor);
                }
            }
        };
        let spline = match CubicBezierSpline::by_origins_and_handles(origins, handles) {
            Ok(spline) => spline,
            Err(e) => {
                warn!("Failed to create spline: {e:?}");
                return;
            }
        };
        let Some(entity) = drawing.entity else {
            create_way(
                &mut drawing,
                &mut commands,
                &mut meshes,
                &way_meshes,
                &materials,
                spline,
            );
            return;
        };
        update_way(&mut ways, &mut curve_added, spline, entity);
    }

    /// Update the [`Way`] on complete.
    pub(crate) fn on_complete(
        &mut self,
        ways: &mut Query<&mut Way>,
        curve_added: &mut EventWriter<CurveAdded>,
    ) {
        let Some(entity) = self.entity else {
            self.reset();
            return;
        };
        let spline = match CubicBezierSpline::by_origins_and_handles(
            self.origins.clone(),
            self.handles.clone(),
        ) {
            Ok(spline) => spline,
            Err(e) => {
                warn!("Failed to create spline: {e:?}");
                return;
            }
        };
        update_way(ways, curve_added, spline, entity);
        self.reset();
    }

    /// Remove the last control and handle.
    pub(crate) fn undo(&mut self) {
        self.handles.pop();
        self.origins.pop();
        self.needs_update = true;
    }

    fn reset(&mut self) {
        self.entity = None;
        self.origins.clear();
        self.handles.clear();
        self.needs_update = false;
    }

    /// Add origin controls on pointer down.
    pub(crate) fn on_pointer_down(
        trigger: Trigger<Pointer<Down>>,
        interface: Res<InterfaceState>,
        mut drawing: ResMut<Drawing>,
        camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    ) {
        if *interface != InterfaceState::DrawWay {
            return;
        }
        if trigger.button != PointerButton::Primary {
            return;
        };
        let Ok(cursor) = Cursor::from_position(&camera, trigger.pointer_location.position) else {
            warn!("Failed to get cursor position");
            return;
        };
        drawing.origins.push(cursor);
    }

    /// Add handle controls on pointer down.
    pub(crate) fn on_pointer_up(
        trigger: Trigger<Pointer<Up>>,
        interface: Res<InterfaceState>,
        mut drawing: ResMut<Drawing>,
        camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    ) {
        if *interface != InterfaceState::DrawWay {
            return;
        }
        if trigger.button != PointerButton::Primary {
            return;
        };
        let Ok(cursor) = Cursor::from_position(&camera, trigger.pointer_location.position) else {
            warn!("Failed to get cursor position");
            return;
        };
        assert_eq!(
            drawing.origins.len(),
            drawing.handles.len() + 1,
            "Pressed and released counts do not match"
        );
        let last_pressed = drawing
            .origins
            .last()
            .expect("Should be a matching pressed");
        if is_almost_equal_to(cursor, *last_pressed) {
            warn!("Press and release are too close");
            drawing.origins.pop();
            return;
        }
        drawing.handles.push(cursor);
        drawing.needs_update = true;
    }
}

fn create_way(
    drawing: &mut ResMut<Drawing>,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    way_meshes: &Res<WayMeshes>,
    materials: &Res<WayMaterials>,
    spline: CubicBezierSpline,
) {
    let way = Way::new(spline);
    let entity = way.clone().spawn(commands, meshes, way_meshes, materials);
    for surface in WaySurface::default_surfaces() {
        surface.spawn(commands, meshes, materials, &way, entity);
    }
    drawing.entity = Some(entity);
}

pub(super) fn update_way(
    ways: &mut Query<&mut Way>,
    curve_added: &mut EventWriter<CurveAdded>,
    spline: CubicBezierSpline,
    entity: Entity,
) {
    let Ok(mut way) = ways.get_mut(entity) else {
        warn!("Failed to get Way: {entity:?}");
        return;
    };
    *way = Way::new(spline);
    curve_added.send(CurveAdded {
        way: entity,
        spline: way.spline.clone(),
    });
}
