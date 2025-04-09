use crate::*;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use konstruo_beziers::*;
use konstruo_geometry::Polyline;
use konstruo_paths::*;
use konstruo_ui::*;

impl DrawMode {
    /// System to update a [`Path`].
    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    pub(crate) fn update_system(
        drawing: Option<ResMut<DrawMode>>,
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
        curve_added.write(CurveAdded {
            path: drawing.path,
            spline: path.spline.clone(),
        });
    }
}
