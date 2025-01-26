use super::*;
use crate::mathematics::spherical_coordinate_system::{AZIMUTHAL_AXIS, POLAR_AXIS, RADIAL_AXIS};
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::input::ButtonInput;
use bevy::prelude::KeyCode::{Equal, KeyA, KeyD, KeyS, KeyW, Minus, ShiftLeft};
use bevy::prelude::*;

impl Orbit {
    /// System to update [`Orbit`] in response to input events.
    pub fn input_system(
        mut query: Query<&mut Orbit>,
        keys: Res<ButtonInput<KeyCode>>,
        buttons: Res<ButtonInput<MouseButton>>,
        wheel_event: EventReader<MouseWheel>,
        motion_event: EventReader<MouseMotion>,
    ) {
        let Ok(mut orbit) = query.get_single_mut() else {
            warn!("Failed to get Orbit");
            return;
        };
        let left_shift_pressed = keys.pressed(ShiftLeft);
        keyboard_input(&mut orbit, keys);
        mouse_button_input(&mut orbit, buttons);
        if left_shift_pressed {
            mouse_motion_input(&mut orbit, motion_event);
        }
        scroll_wheel_input(&mut orbit, wheel_event);
    }
}

fn keyboard_input(orbit: &mut Mut<Orbit>, keys: Res<ButtonInput<KeyCode>>) {
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

fn mouse_button_input(orbit: &mut Mut<Orbit>, buttons: Res<ButtonInput<MouseButton>>) {
    if buttons.just_pressed(MouseButton::Middle) {
        orbit.dragging = true;
    }
    if buttons.just_released(MouseButton::Middle) {
        orbit.dragging = false;
        orbit.stop();
    }
}

fn mouse_motion_input(orbit: &mut Mut<Orbit>, mut motion_event: EventReader<MouseMotion>) {
    for motion in motion_event.read() {
        if orbit.dragging {
            orbit.in_direction_of_motion(*motion);
        }
    }
}

fn scroll_wheel_input(orbit: &mut Mut<Orbit>, mut wheel_event: EventReader<MouseWheel>) {
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
