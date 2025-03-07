use crate::beziers::{CubicBezier, CubicBezierError, CubicBezierSpline};
use crate::geometry::vectors::is_almost_equal_to;
use crate::infrastructure::*;
use crate::ui::*;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::cmp::Ordering;
use CreateSplineError::*;

#[derive(Default, Resource)]
pub struct Drawing {
    pub origins: Vec<Vec3>,
    pub handles: Vec<Vec3>,
    pub entity: Option<Entity>,
    pub needs_update: bool,
}

#[allow(dead_code)]
#[derive(Debug)]
enum CreateSplineError {
    NoCurves,
    InvalidCounts(usize, usize),
    CurveError(CubicBezierError),
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
        mut event_writer: EventWriter<SplineChangedEvent>,
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
        let spline = match get_spline(origins, handles) {
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
        update_way(&mut ways, &mut event_writer, spline, entity);
    }

    /// Update the [`Way`] on complete.
    pub(crate) fn on_complete(
        &mut self,
        ways: &mut Query<&mut Way>,
        event_writer: &mut EventWriter<SplineChangedEvent>,
    ) {
        let Some(entity) = self.entity else {
            self.reset();
            return;
        };
        let spline = match get_spline(self.origins.clone(), self.handles.clone()) {
            Ok(spline) => spline,
            Err(e) => {
                warn!("Failed to create spline: {e:?}");
                return;
            }
        };
        update_way(ways, event_writer, spline, entity);
        self.reset();
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
    event_writer: &mut EventWriter<SplineChangedEvent>,
    spline: CubicBezierSpline,
    entity: Entity,
) {
    let Ok(mut way) = ways.get_mut(entity) else {
        warn!("Failed to get Way: {entity:?}");
        return;
    };
    *way = Way::new(spline);
    event_writer.send(SplineChangedEvent {
        way: entity,
        spline: way.spline.clone(),
    });
}

#[allow(clippy::indexing_slicing)]
fn get_spline(
    origins: Vec<Vec3>,
    handles: Vec<Vec3>,
) -> Result<CubicBezierSpline, CreateSplineError> {
    if origins.is_empty()
        || (origins.len() != handles.len() && origins.len() != (handles.len() + 1))
    {
        return Err(InvalidCounts(origins.len(), handles.len()));
    }
    let origins = origins.clone();
    let handles = handles.clone();
    let mut curves = Vec::new();
    let count = origins.len() - 1;
    for i in 0..count {
        let start = origins[i];
        let start_handle = handles[i];
        let end = origins[i + 1];
        let next_handle = handles.get(i + 1);
        let end_handle = if let Some(next_handle) = next_handle {
            let translation = end - *next_handle;
            end + translation
        } else {
            start_handle
        };
        let curve = CubicBezier::new(start, start_handle, end_handle, end).map_err(CurveError)?;
        curves.push(curve);
    }
    if curves.is_empty() {
        return Err(NoCurves);
    }
    Ok(CubicBezierSpline { curves })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::beziers::ControlType::{End, Start, StartHandle};

    fn example_spline() -> CubicBezierSpline {
        CubicBezierSpline {
            curves: vec![
                CubicBezier {
                    start: Vec3::new(0.0, 70.0, 0.0),
                    start_handle: Vec3::new(30.0, 70.0, 0.0),
                    end_handle: Vec3::new(30.0, 40.0, 0.0),
                    end: Vec3::new(50.0, 40.0, 0.0),
                },
                CubicBezier {
                    start: Vec3::new(50.0, 40.0, 0.0),
                    start_handle: Vec3::new(70.0, 40.0, 0.0),
                    end_handle: Vec3::new(70.0, 15.0, 0.0),
                    end: Vec3::new(70.0, 0.0, 0.0),
                },
            ],
        }
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn get_spline_test_complete() -> Result<(), CreateSplineError> {
        // Arrange
        let example = example_spline();
        let pressed = [
            example.get_control(Start, 0).unwrap(),
            example.get_control(Start, 1).unwrap(),
            example.get_control(End, 1).unwrap(),
        ];
        let released = [
            example.get_control(StartHandle, 0).unwrap(),
            example.get_control(StartHandle, 1).unwrap(),
            example.get_control(End, 1).unwrap() + Vec3::new(10.0, 0.0, 0.0),
        ];

        // Act
        let result = get_spline(pressed.to_vec(), released.to_vec())?;

        // Assert
        assert_eq!(result.curves.len(), 2);
        Ok(())
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn get_spline_test_missing_released() -> Result<(), CreateSplineError> {
        // Arrange
        let example = example_spline();
        let pressed = [
            example.get_control(Start, 0).unwrap(),
            example.get_control(Start, 1).unwrap(),
            example.get_control(End, 1).unwrap(),
        ];
        let released = [
            example.get_control(StartHandle, 0).unwrap(),
            example.get_control(StartHandle, 1).unwrap(),
            // example.get_control(End, 1).unwrap() + Vec3::new(10.0, 0.0, 0.0),
        ];

        // Act
        let result = get_spline(pressed.to_vec(), released.to_vec())?;

        // Assert
        assert_eq!(result.curves.len(), 2);
        Ok(())
    }
}
