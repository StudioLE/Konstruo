use crate::*;
use bevy::prelude::*;
use konstruo_beziers::*;
use konstruo_core::Vec3Extensions;
use konstruo_paths::*;
use konstruo_ui::*;

impl DrawMode {
    /// Get the actions when [`DrawMode`] is active.
    pub(crate) fn actions() -> Vec<Action> {
        vec![
            Action {
                label: String::from("Undo"),
                icon: Icon::font_awesome("undo"),
                on_press: Observer::new(DrawMode::undo_action),
            },
            Action {
                label: String::from("Complete"),
                icon: Icon::font_awesome("check"),
                on_press: Observer::new(DrawMode::complete_action),
            },
        ]
    }

    /// Update the [`Path`] on action button pressed.
    fn complete_action(
        trigger: On<Pointer<Release>>,
        mut commands: Commands,
        mut interface: ResMut<InterfaceState>,
        drawing: Res<DrawMode>,
        mut paths: Query<&mut Path>,
        mut curve_added: MessageWriter<CurveAdded>,
    ) {
        if trigger.button != PointerButton::Primary {
            return;
        }
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
        let Ok(mut path) = paths.get_mut(drawing.path) else {
            warn!("Failed to get Path: {:?}", drawing.path);
            return;
        };
        *path = Path::new(spline);
        curve_added.write(CurveAdded {
            path: drawing.path,
            spline: path.spline.clone(),
        });
        commands.remove_resource::<DrawMode>();
    }

    /// Activate [`InterfaceState::DrawPath`].
    pub(crate) fn start_action(
        trigger: On<Pointer<Release>>,
        mut interface: ResMut<InterfaceState>,
        commands: Commands,
        meshes: ResMut<Assets<Mesh>>,
        path_meshes: Res<PathMeshes>,
        materials: Res<PathMaterials>,
    ) {
        if trigger.button != PointerButton::Primary {
            return;
        }
        trace!("Draw Path button was pressed.");
        *interface = InterfaceState::DrawPath;
        let mut factory = PathFactory {
            commands,
            meshes,
            path_meshes,
            materials,
        };
        let drawing = DrawMode::new(&mut factory);
        factory.commands.insert_resource(drawing);
    }

    /// Remove the last control and handle.
    fn undo_action(trigger: On<Pointer<Release>>, mut drawing: ResMut<DrawMode>) {
        if trigger.button != PointerButton::Primary {
            return;
        }
        trace!("Undo button was pressed.");
        drawing.handles.pop();
        drawing.origins.pop();
    }

    /// Add origin controls on pointer down.
    pub(crate) fn on_pointer_down(
        trigger: On<Pointer<Press>>,
        drawing: Option<ResMut<DrawMode>>,
        camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    ) {
        let Some(mut drawing) = drawing else {
            return;
        };
        if trigger.button != PointerButton::Primary {
            return;
        }
        let Ok(cursor) = Cursor::from_position(&camera, trigger.pointer_location.position) else {
            warn!("Failed to get cursor position");
            return;
        };
        drawing.origins.push(cursor);
    }

    /// Add handle controls on pointer up.
    pub(crate) fn on_pointer_up(
        trigger: On<Pointer<Release>>,
        drawing: Option<ResMut<DrawMode>>,
        camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    ) {
        let Some(mut drawing) = drawing else {
            return;
        };
        if trigger.button != PointerButton::Primary {
            return;
        }
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
        if cursor.is_almost_equal_to(*last_pressed) {
            warn!("Press and release are too close");
            drawing.origins.pop();
            return;
        }
        drawing.handles.push(cursor);
    }
}
