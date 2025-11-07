use super::*;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::input::ButtonInput;
use bevy::prelude::KeyCode::{Equal, KeyA, KeyD, KeyS, KeyW, Minus, ShiftLeft};
use bevy::prelude::*;
use konstruo_geometry::{AZIMUTHAL_AXIS, POLAR_AXIS, RADIAL_AXIS};

impl Orbit {
    /// System to update [`Orbit`] in response to keyboard input events.
    pub(super) fn keyboard_input_system(
        mut query: Query<&mut Orbit>,
        keys: Res<ButtonInput<KeyCode>>,
    ) {
        let Ok(mut orbit) = query.single_mut() else {
            warn!("Failed to get Orbit");
            return;
        };
        if keys.pressed(ShiftLeft) && keys.any_pressed([KeyW, KeyA, KeyS, KeyD]) {
            let mut direction = Vec3::ZERO;
            if keys.pressed(KeyW) {
                direction += POLAR_AXIS * -1.0;
            }
            if keys.pressed(KeyS) {
                direction += POLAR_AXIS;
            }
            if keys.pressed(KeyA) {
                direction += AZIMUTHAL_AXIS * -1.0;
            }
            if keys.pressed(KeyD) {
                direction += AZIMUTHAL_AXIS;
            }
            direction = direction.normalize_or_zero();
            if direction != Vec3::ZERO {
                orbit.in_direction(direction);
            }
        }
        if keys.any_just_released([KeyW, KeyA, KeyS, KeyD, Equal, Minus]) {
            orbit.stop();
        }
        if keys.pressed(Minus) {
            orbit.in_direction(RADIAL_AXIS);
        }
        if keys.pressed(Equal) {
            orbit.in_direction(RADIAL_AXIS * -1.0);
        }
    }

    /// System to update [`Orbit`] in response to mouse button input events.
    pub(super) fn mouse_button_input_system(
        mut query: Query<&mut Orbit>,
        buttons: Res<ButtonInput<MouseButton>>,
    ) {
        let Ok(mut orbit) = query.single_mut() else {
            warn!("Failed to get Orbit");
            return;
        };
        if buttons.just_pressed(MouseButton::Middle) {
            orbit.dragging = true;
        }
        if buttons.just_released(MouseButton::Middle) {
            orbit.dragging = false;
            orbit.stop();
        }
    }

    /// System to update [`Orbit`] in response to mouse motion input events.
    pub(super) fn mouse_motion_input_system(
        mut query: Query<&mut Orbit>,
        keys: Res<ButtonInput<KeyCode>>,
        mut motion_event: MessageReader<MouseMotion>,
    ) {
        let Ok(mut orbit) = query.single_mut() else {
            warn!("Failed to get Orbit");
            return;
        };
        if !keys.pressed(ShiftLeft) {
            return;
        }
        for motion in motion_event.read() {
            if orbit.dragging {
                orbit.in_direction_of_motion(*motion);
            }
        }
    }

    /// System to update [`Orbit`] in response to scroll wheel input events.
    pub(super) fn scroll_wheel_input_system(
        mut query: Query<&mut Orbit>,
        mut wheel_event: MessageReader<MouseWheel>,
    ) {
        let Ok(mut orbit) = query.single_mut() else {
            warn!("Failed to get Orbit");
            return;
        };
        for scroll_wheel in wheel_event.read() {
            let vertical = match scroll_wheel.unit {
                // TODO: Increase speed when scroll unit is pixel
                MouseScrollUnit::Line | MouseScrollUnit::Pixel => scroll_wheel.y,
            };
            let direction = if vertical == 0.0 {
                return;
            } else if vertical > 0.0 {
                -1.0
            } else {
                1.0
            };
            orbit.in_direction(RADIAL_AXIS * direction);
        }
    }
}
