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

    /// Get the actions when [`Drawing`] is active.
    pub(crate) fn actions() -> Vec<Action> {
        vec![
            Action {
                label: String::from("Undo"),
                icon: Icon::font_awesome("undo"),
                on_press: Observer::new(Drawing::undo_action),
            },
            Action {
                label: String::from("Complete"),
                icon: Icon::font_awesome("check"),
                on_press: Observer::new(Drawing::complete_action),
            },
        ]
    }

    /// Update the [`Way`] on action button pressed.
    fn complete_action(
        _trigger: Trigger<Pointer<Up>>,
        mut interface: ResMut<InterfaceState>,
        mut drawing: ResMut<Drawing>,
        mut ways: Query<&mut Way>,
        mut curve_added: EventWriter<CurveAdded>,
    ) {
        trace!("Complete button was pressed.");
        *interface = InterfaceState::Default;
        let Some(entity) = drawing.entity else {
            drawing.reset();
            return;
        };
        let spline = match CubicBezierSpline::by_origins_and_handles(
            drawing.origins.clone(),
            drawing.handles.clone(),
        ) {
            Ok(spline) => spline,
            Err(e) => {
                warn!("Failed to create spline: {e:?}");
                return;
            }
        };
        update_way(&mut ways, &mut curve_added, spline, entity);
        drawing.reset();
    }

    /// Activate [`InterfaceState::DrawWay`].
    pub(crate) fn start_action(
        _trigger: Trigger<Pointer<Up>>,
        mut interface: ResMut<InterfaceState>,
    ) {
        trace!("Draw Way button was pressed.");
        *interface = InterfaceState::DrawWay;
        // TODO: Create Drawing resource
    }

    /// Remove the last control and handle.
    fn undo_action(_trigger: Trigger<Pointer<Up>>, mut drawing: ResMut<Drawing>) {
        trace!("Undo button was pressed.");
        drawing.handles.pop();
        drawing.origins.pop();
        drawing.needs_update = true;
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

    /// Add handle controls on pointer up.
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
