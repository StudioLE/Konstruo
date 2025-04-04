use super::*;
use crate::ui::Cursor;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use KeyCode::{KeyA, KeyD, KeyS, KeyW, ShiftLeft};

impl Pan {
    /// System to update [`Pan`] in response to keyboard input events.
    pub(super) fn keyboard_input_system(
        mut pan: Query<(&mut Pan, &Children)>,
        orbits: Query<&Orbit>,
        keys: Res<ButtonInput<KeyCode>>,
    ) {
        let Ok((mut pan, children)) = pan.single_mut() else {
            warn!("Failed to get Pan");
            return;
        };
        // TODO: This can be simplified by getting Orbit first and fetching its parent
        let Some(orbit) = children.iter().find_map(|child| orbits.get(child).ok()) else {
            warn!("Failed to get Orbit from Pan");
            return;
        };
        if !keys.pressed(ShiftLeft) && keys.any_pressed([KeyW, KeyA, KeyS, KeyD]) {
            let mut direction = Vec3::ZERO;
            if keys.pressed(KeyW) {
                direction += Vec3::Y;
            }
            if keys.pressed(KeyS) {
                direction += Vec3::NEG_Y;
            }
            if keys.pressed(KeyA) {
                direction += Vec3::NEG_X;
            }
            if keys.pressed(KeyD) {
                direction += Vec3::X;
            }
            direction = direction.normalize_or_zero();
            if direction != Vec3::ZERO {
                direction = orbit
                    .get_orientation()
                    .mul_vec3(direction.normalize())
                    .with_z(0.0)
                    .normalize();
                pan.in_direction(direction);
            }
        }
        if keys.any_just_released([KeyW, KeyA, KeyS, KeyD]) {
            pan.stop();
        }
    }

    /// System to update [`Pan`] in response to mouse button input events.
    pub(super) fn mouse_button_input_system(
        mut pan: Query<&mut Pan>,
        window: Query<&Window, With<PrimaryWindow>>,
        camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
        buttons: Res<ButtonInput<MouseButton>>,
    ) {
        let Ok(mut pan) = pan.single_mut() else {
            warn!("Failed to get Pan");
            return;
        };
        if buttons.just_pressed(MouseButton::Middle) {
            if let Ok(position) = Cursor::from_window(&window, &camera) {
                pan.dragging = Some(position);
            }
        }
        if buttons.just_released(MouseButton::Middle) {
            pan.dragging = None;
            pan.stop();
        }
    }

    /// System to update [`Pan`] in response to mouse motion input events.
    pub(super) fn mouse_motion_input_system(
        mut pan: Query<(&mut Pan, &mut Transform)>,
        keys: Res<ButtonInput<KeyCode>>,
        window: Query<&Window, With<PrimaryWindow>>,
        camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
        mut motion: EventReader<MouseMotion>,
    ) {
        let Ok((mut pan, transform)) = pan.single_mut() else {
            warn!("Failed to get Pan");
            return;
        };
        if keys.pressed(ShiftLeft) {
            return;
        }
        if pan.dragging.is_some() && motion.read().next().is_some() {
            if let Ok(position) = Cursor::from_window(&window, &camera) {
                pan.by_grab(transform, position);
            }
        }
    }
}
