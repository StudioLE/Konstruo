use crate::beziers::{ControlType, CubicBezierSpline};
use crate::geometry::{Polyline, Vec3Helpers};
use crate::infrastructure::*;
use crate::ui::*;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource)]
pub struct Drawing {
    origins: Vec<Vec3>,
    handles: Vec<Vec3>,
    path: Entity,
    control: Entity,
    line: Entity,
    is_ready: bool,
}

impl Drawing {
    /// System to update a [`Path`].
    #[allow(clippy::too_many_arguments)]
    pub(super) fn update_system(
        drawing: Option<ResMut<Drawing>>,
        mut paths: Query<(&mut Path, &mut Visibility)>,
        mut controls: Query<(&mut Transform, &mut Visibility), (With<PathControl>, Without<Path>)>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut lines: Query<
            (&mut Mesh3d, &mut Visibility),
            (With<PathControlLine>, Without<Path>, Without<PathControl>),
        >,
        mut curve_added: EventWriter<CurveAdded>,
        motion: EventReader<MouseMotion>,
        window: Query<&Window, With<PrimaryWindow>>,
        camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    ) {
        let Some(mut drawing) = drawing else {
            return;
        };
        if !drawing.is_changed() && motion.is_empty() {
            return;
        }
        let mut origins = drawing.origins.clone();
        let mut handles = drawing.handles.clone();
        let is_handle_next = origins.len() > handles.len();
        let Ok(cursor) = Cursor::from_window(&window, &camera) else {
            return;
        };
        if is_handle_next {
            handles.push(cursor);
        } else {
            origins.push(cursor);
        }
        // Update Control
        let Ok((mut transform, mut visibility)) = controls.get_mut(drawing.control) else {
            warn!("Failed to get PathControl: {:?}", drawing.control);
            return;
        };
        if is_handle_next {
            let position = *handles.last().expect("Should be at least one handle");
            *transform = Transform::from_translation(position);
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
        // Update line
        let Ok((mut mesh, mut visibility)) = lines.get_mut(drawing.line) else {
            warn!("Failed to get PathControl: {:?}", drawing.control);
            return;
        };
        if is_handle_next {
            let start = *origins.last().expect("Should be at least one origin");
            let end = *handles.last().expect("Should be at least one handle");
            *mesh = Mesh3d(meshes.add(Polyline::new(vec![start, end]).to_mesh()));
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
        // Update Path
        if origins.len() < 2 {
            return;
        }
        let spline = match CubicBezierSpline::by_origins_and_handles(origins, handles) {
            Ok(spline) => spline,
            Err(e) => {
                warn!("Failed to create spline: {e:?}");
                return;
            }
        };
        let Ok((mut path, mut visibility)) = paths.get_mut(drawing.path) else {
            warn!("Failed to get Path: {:?}", drawing.path);
            return;
        };
        *path = Path::new(spline);
        if drawing.is_ready {
            *visibility = Visibility::Visible;
        } else {
            drawing.is_ready = true;
        }
        curve_added.send(CurveAdded {
            path: drawing.path,
            spline: path.spline.clone(),
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

    /// Update the [`Path`] on action button pressed.
    fn complete_action(
        _trigger: Trigger<Pointer<Up>>,
        mut commands: Commands,
        mut interface: ResMut<InterfaceState>,
        drawing: Res<Drawing>,
        mut paths: Query<&mut Path>,
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
        let Ok(mut path) = paths.get_mut(drawing.path) else {
            warn!("Failed to get Path: {:?}", drawing.path);
            return;
        };
        *path = Path::new(spline);
        curve_added.send(CurveAdded {
            path: drawing.path,
            spline: path.spline.clone(),
        });
        commands.remove_resource::<Drawing>();
    }

    /// Activate [`InterfaceState::DrawPath`].
    pub(crate) fn start_action(
        _trigger: Trigger<Pointer<Up>>,
        mut interface: ResMut<InterfaceState>,
        commands: Commands,
        meshes: ResMut<Assets<Mesh>>,
        path_meshes: Res<PathMeshes>,
        materials: Res<PathMaterials>,
    ) {
        trace!("Draw Path button was pressed.");
        *interface = InterfaceState::DrawPath;
        let mut factory = PathFactory {
            commands,
            meshes,
            path_meshes,
            materials,
        };
        let drawing = factory.create_drawing();
        factory.commands.insert_resource(drawing);
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
        }
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
        if Vec3Helpers::is_almost_equal_to(cursor, *last_pressed) {
            warn!("Press and release are too close");
            drawing.origins.pop();
            return;
        }
        drawing.handles.push(cursor);
    }
}

impl PathFactory<'_> {
    /// Spawn temporary [`Path`] entities and create a new [`Drawing`] resource.
    fn create_drawing(&mut self) -> Drawing {
        let spline = CubicBezierSpline::example_2();
        let path = Path::new(spline);
        let path_entity = self.spawn_path(path.clone());
        self.commands.entity(path_entity).insert(Visibility::Hidden);
        for surface in PathSurface::default_surfaces() {
            self.spawn_surface(surface, &path, path_entity);
        }
        let bundle =
            self.control_bundle(ControlType::StartHandle, 0, Vec3::ZERO, Visibility::Hidden);
        let control = self.commands.spawn(bundle).id();
        let line = vec![Vec3::ZERO, Vec3::ONE];
        let bundle = (
            PathControlLine::new(0, true),
            Mesh3d(self.meshes.add(Polyline::new(line).to_mesh())),
            MeshMaterial3d(self.materials.control_line.clone()),
            Visibility::Hidden,
        );
        let line = self.commands.spawn(bundle).id();
        Drawing {
            origins: Vec::new(),
            handles: Vec::new(),
            path: path_entity,
            control,
            line,
            is_ready: false,
        }
    }
}
