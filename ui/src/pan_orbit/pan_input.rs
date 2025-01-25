use crate::cursor::Cursor;
use crate::pan_orbit::pan::Pan;
use crate::pan_orbit::primary_camera::PrimaryCamera;
use crate::pan_orbit::Orbit;
use bevy::hierarchy::Children;
use bevy::input::mouse::MouseMotion;
use bevy::input::ButtonInput;
use bevy::log::warn;
use bevy::math::Vec3;
use bevy::prelude::KeyCode::{KeyA, KeyD, KeyS, KeyW, ShiftLeft};
use bevy::prelude::{
    Camera, EventReader, GlobalTransform, KeyCode, MouseButton, Mut, Query, Res, Transform, Window,
    With,
};
use bevy::window::PrimaryWindow;

impl Pan {
    /// System to update [`Pan`] in response to input events.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn input_system(
        mut pan: Query<(&mut Pan, &mut Transform, &Children)>,
        orbits: Query<&Orbit>,
        window: Query<&Window, With<PrimaryWindow>>,
        camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
        keys: Res<ButtonInput<KeyCode>>,
        buttons: Res<ButtonInput<MouseButton>>,
        motion: EventReader<MouseMotion>,
    ) {
        let Ok((mut pan, transform, children)) = pan.get_single_mut() else {
            warn!("Failed to get Pan");
            return;
        };
        let Some(orbit) = children.iter().find_map(|&child| orbits.get(child).ok()) else {
            warn!("Failed to get Orbit from Pan");
            return;
        };
        let left_shift_pressed = keys.pressed(ShiftLeft);
        keyboard_input(&mut pan, orbit, keys);
        mouse_button_input(&mut pan, &window, &camera, buttons);
        if !left_shift_pressed {
            mouse_motion_input(&mut pan, transform, &window, &camera, motion);
        }
    }
}

fn keyboard_input(pan: &mut Mut<Pan>, orbit: &Orbit, keys: Res<ButtonInput<KeyCode>>) {
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

fn mouse_button_input(
    pan: &mut Mut<Pan>,
    window: &Query<&Window, With<PrimaryWindow>>,
    camera: &Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Middle) {
        if let Ok(position) = Cursor::on_ground(window, camera) {
            pan.dragging = Some(position);
        };
    }
    if buttons.just_released(MouseButton::Middle) {
        pan.dragging = None;
        pan.stop();
    }
}

fn mouse_motion_input(
    pan: &mut Mut<Pan>,
    transform: Mut<Transform>,
    window: &Query<&Window, With<PrimaryWindow>>,
    camera: &Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    mut motion: EventReader<MouseMotion>,
) {
    if pan.dragging.is_some() && motion.read().next().is_some() {
        if let Ok(position) = Cursor::on_ground(window, camera) {
            pan.by_grab(transform, position);
        };
    }
}
