use crate::beziers::{CubicBezier, CubicBezierSpline};
use crate::geometry::vectors::is_almost_equal_to;
use crate::infrastructure::{SplineChangedEvent, Way, WayMaterials, WayMeshes, WaySurface};
use crate::ui::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Default, Resource)]
pub struct Drawing {
    pub pressed: Vec<Vec3>,
    pub released: Vec<Vec3>,
    pub entity: Option<Entity>,
    pub needs_update: bool,
}

impl Drawing {
    /// System to update [`Drawing`] on input.
    #[allow(clippy::too_many_arguments, clippy::integer_division)]
    pub(super) fn input_system(
        interface: Res<InterfaceState>,
        mut drawing: ResMut<Drawing>,
        buttons: Res<ButtonInput<MouseButton>>,
        window: Query<&Window, With<PrimaryWindow>>,
        camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    ) {
        if *interface != InterfaceState::DrawWay {
            return;
        }
        let is_pressed = buttons.just_pressed(MouseButton::Left);
        let is_released = buttons.just_released(MouseButton::Left);
        if !is_pressed && !is_released {
            return;
        }
        let Ok(position) = Cursor::on_ground(&window, &camera) else {
            warn!("Failed to get Cursor position");
            return;
        };
        if is_pressed {
            drawing.pressed.push(position);
            return;
        }
        assert_eq!(
            drawing.pressed.len(),
            drawing.released.len() + 1,
            "Pressed and released counts do not match"
        );
        let last_pressed = drawing
            .pressed
            .last()
            .expect("Should be a matching pressed");
        if is_almost_equal_to(position, *last_pressed) {
            warn!("Press and release are too close");
            drawing.pressed.pop();
            return;
        }
        drawing.released.push(position);
        drawing.needs_update = true;
    }

    /// System to update a [`Way`]
    #[allow(clippy::too_many_arguments, clippy::integer_division)]
    pub(super) fn update_system(
        mut commands: Commands,
        mut drawing: ResMut<Drawing>,
        mut meshes: ResMut<Assets<Mesh>>,
        way_meshes: Res<WayMeshes>,
        materials: Res<WayMaterials>,
        mut ways: Query<&mut Way>,
        mut event_writer: EventWriter<SplineChangedEvent>,
    ) {
        if !drawing.needs_update || drawing.released.len() < 2 {
            return;
        }
        drawing.needs_update = false;
        let Some(spline) = drawing.get_spline() else {
            return;
        };
        let Some(entity) = drawing.entity else {
            let way = Way::new(spline);
            let entity = way
                .clone()
                .spawn(&mut commands, &mut meshes, &way_meshes, &materials);
            for surface in WaySurface::default_surfaces() {
                surface.spawn(&mut commands, &mut meshes, &materials, &way, entity);
            }
            drawing.entity = Some(entity);
            return;
        };
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

    #[must_use]
    #[allow(clippy::indexing_slicing)]
    fn get_spline(&self) -> Option<CubicBezierSpline> {
        if self.pressed.len() != self.released.len() {
            warn!(
                "Pressed and released do not match: {} != {}",
                self.pressed.len(),
                self.released.len()
            );
            return None;
        }
        let mut curves = Vec::new();
        let count = self.pressed.len() - 1;
        for i in 0..count {
            let start = self.pressed[i];
            let start_handle = self.released[i];
            let end = self.pressed[i + 1];
            let next_handle = self.released.get(i + 1);
            let end_handle = if let Some(next_handle) = next_handle {
                let translation = end - *next_handle;
                end + translation
            } else {
                start_handle
            };
            // TODO: Move this to a CubicBezier::new() method
            if is_almost_equal_to(start, start_handle) {
                warn!("Start and start handle are too close: {i}");
                return None;
            }
            if is_almost_equal_to(start, end) {
                warn!("Start and end are too close: {i}");
                return None;
            }
            if is_almost_equal_to(end_handle, end) {
                warn!("End and end handle are too close: {i}");
                return None;
            }
            curves.push(CubicBezier {
                start,
                start_handle,
                end_handle,
                end,
            });
        }
        trace!("Spline with {} curves", curves.len());
        Some(CubicBezierSpline { curves })
    }
}
