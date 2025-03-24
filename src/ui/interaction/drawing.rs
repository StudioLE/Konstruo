use crate::beziers::CubicBezierSpline;
use crate::geometry::vectors::is_almost_equal_to;
use crate::infrastructure::*;
use crate::ui::*;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource)]
pub struct Drawing {
    origins: Vec<Vec3>,
    handles: Vec<Vec3>,
    way: Entity,
    is_ready: bool,
}

impl Drawing {
    /// Create a new [`Drawing`] resource with a [`Way`].
    fn new(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        way_meshes: &Res<WayMeshes>,
        materials: &Res<WayMaterials>,
    ) -> Drawing {
        let spline = CubicBezierSpline::example_2();
        let way = Way::new(spline);
        let entity = way.clone().spawn(commands, meshes, way_meshes, materials);
        commands.entity(entity).insert(Visibility::Hidden);
        for surface in WaySurface::default_surfaces() {
            surface.spawn(commands, meshes, materials, &way, entity);
        }
        Drawing {
            origins: Vec::new(),
            handles: Vec::new(),
            way: entity,
            is_ready: false,
        }
    }

    /// System to update a [`Way`].
    pub(super) fn update_system(
        drawing: Option<ResMut<Drawing>>,
        mut ways: Query<(&mut Way, &mut Visibility)>,
        mut curve_added: EventWriter<CurveAdded>,
        motion: EventReader<MouseMotion>,
        window: Query<&Window, With<PrimaryWindow>>,
        camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    ) {
        let Some(mut drawing) = drawing else {
            return;
        };
        if drawing.handles.is_empty() {
            return;
        }
        if !drawing.is_changed() && motion.is_empty() {
            return;
        }
        let mut origins = drawing.origins.clone();
        let mut handles = drawing.handles.clone();
        let handle_is_next = origins.len() > handles.len();
        if let Ok(cursor) = Cursor::from_window(&window, &camera) {
            if handle_is_next {
                handles.push(cursor);
            } else {
                origins.push(cursor);
            }
        };
        let spline = match CubicBezierSpline::by_origins_and_handles(origins, handles) {
            Ok(spline) => spline,
            Err(e) => {
                warn!("Failed to create spline: {e:?}");
                return;
            }
        };
        let Ok((mut way, mut visibility)) = ways.get_mut(drawing.way) else {
            warn!("Failed to get Way: {:?}", drawing.way);
            return;
        };
        *way = Way::new(spline);
        if drawing.is_ready {
            *visibility = Visibility::Visible;
        } else {
            drawing.is_ready = true;
        }
        curve_added.send(CurveAdded {
            way: drawing.way,
            spline: way.spline.clone(),
        });
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
        mut commands: Commands,
        mut interface: ResMut<InterfaceState>,
        drawing: Res<Drawing>,
        mut ways: Query<&mut Way>,
        mut curve_added: EventWriter<CurveAdded>,
    ) {
        trace!("Complete button was pressed.");
        *interface = InterfaceState::Default;
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
        let Ok(mut way) = ways.get_mut(drawing.way) else {
            warn!("Failed to get Way: {:?}", drawing.way);
            return;
        };
        *way = Way::new(spline);
        curve_added.send(CurveAdded {
            way: drawing.way,
            spline: way.spline.clone(),
        });
        commands.remove_resource::<Drawing>();
    }

    /// Activate [`InterfaceState::DrawWay`].
    pub(crate) fn start_action(
        _trigger: Trigger<Pointer<Up>>,
        mut interface: ResMut<InterfaceState>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        way_meshes: Res<WayMeshes>,
        materials: Res<WayMaterials>,
    ) {
        trace!("Draw Way button was pressed.");
        *interface = InterfaceState::DrawWay;
        let drawing = Drawing::new(&mut commands, &mut meshes, &way_meshes, &materials);
        commands.insert_resource(drawing);
    }

    /// Remove the last control and handle.
    fn undo_action(_trigger: Trigger<Pointer<Up>>, mut drawing: ResMut<Drawing>) {
        trace!("Undo button was pressed.");
        drawing.handles.pop();
        drawing.origins.pop();
    }

    /// Add origin controls on pointer down.
    pub(crate) fn on_pointer_down(
        trigger: Trigger<Pointer<Down>>,
        drawing: Option<ResMut<Drawing>>,
        camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    ) {
        let Some(mut drawing) = drawing else {
            return;
        };
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
        drawing: Option<ResMut<Drawing>>,
        camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    ) {
        let Some(mut drawing) = drawing else {
            return;
        };
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
    }
}
