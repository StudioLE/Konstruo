use crate::tools::cursor::Cursor;
use beach_core::constraints::clamp_float::ClampFloat;
use beach_core::constraints::clamp_vec3::ClampVec3;
use beach_core::movement::target_based_movement::TargetBasedMovement;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use KeyCode::*;

/// Pan state of the camera.
#[derive(Component)]
pub struct Pan {
    pub movement: TargetBasedMovement,
    pub drag_origin: Option<Vec3>,
}

impl Default for Pan {
    fn default() -> Self {
        Self {
            movement: TargetBasedMovement {
                current: Vec3::ZERO,
                clamp: ClampVec3 {
                    x: ClampFloat::Fixed(-1000.0, 1000.0),
                    y: ClampFloat::Fixed(-1000.0, 1000.0),
                    z: ClampFloat::Fixed(-1000.0, 1000.0),
                },
                target: None,
                speed: Vec3::new(100.0, 100.0, 100.0),
            },
            drag_origin: None,
        }
    }
}

impl Pan {
    /// Get the position.
    pub fn get_transform(&self) -> Transform {
        Transform::from_translation(self.movement.current)
    }

    fn update(&mut self) {
        self.movement.update();
    }

    /// Orbit the camera in direction relative to the Azimuth.
    fn in_direction(&mut self, direction: Vec3, modifier: f32) {
        let velocity = direction * self.movement.speed * modifier;
        self.movement.set_target_relative_to_position(velocity);
    }

    /// Orbit the camera in the direction of the mouse motion.
    #[allow(dead_code)]
    fn in_direction_of_motion(&mut self, mouse: MouseMotion) {
        let direction = mouse.delta.normalize();
        let polar = direction.y * -1.0 * 0.1;
        let azimuthal = direction.x * -1.0 * 0.04;
        let displacement = Vec3::new(0.0, polar, azimuthal);
        self.movement.set_target_relative_to_position(displacement);
    }

    fn by_grab(&mut self, cursor: &Res<Cursor>) {
        let translation = self.drag_origin.expect("Drag already confirmed.") - cursor.position;
        self.movement.set_target_relative_to_position(translation);
    }

    fn stop(&mut self) {
        self.movement.remove_target();
    }
}

/// Update the movement once per frame.
pub fn on_update(mut query: Query<&mut Pan>) {
    for mut pan in &mut query {
        pan.update();
    }
}

/// Update the transform if the position changes.
pub fn on_changed(mut query: Query<(&mut Transform, &Pan), Changed<Pan>>) {
    for (mut transform, pan) in &mut query {
        *transform = pan.get_transform();
    }
}

/// Respond to input events.
pub fn on_input(
    mut query: Query<&mut Pan>,
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    motion_event: EventReader<MouseMotion>,
    cursor: Res<Cursor>,
) {
    let Ok(mut orbit) = query.get_single_mut() else {
        return;
    };
    let left_shift_pressed = keys.pressed(ShiftLeft);
    keyboard_input(&mut orbit, keys);
    mouse_button_input(&mut orbit, buttons, &cursor);
    if !left_shift_pressed {
        mouse_motion_input(&mut orbit, motion_event, &cursor);
    }
}

fn keyboard_input(pan: &mut Mut<Pan>, keys: Res<ButtonInput<KeyCode>>) {
    if !keys.pressed(ShiftLeft) {
        if keys.pressed(KeyW) {
            pan.in_direction(Vec3::Z * -1.0, 1.0);
        }
        if keys.pressed(KeyA) {
            pan.in_direction(Vec3::X * -1.0, 1.0);
        }
        if keys.pressed(KeyS) {
            pan.in_direction(Vec3::Z, 1.0);
        }
        if keys.pressed(KeyD) {
            pan.in_direction(Vec3::X, 1.0);
        }
    }
    if keys.any_just_released([KeyW, KeyA, KeyS, KeyD]) {
        pan.stop();
    }
}

fn mouse_button_input(
    pan: &mut Mut<Pan>,
    buttons: Res<ButtonInput<MouseButton>>,
    cursor: &Res<Cursor>,
) {
    if buttons.just_pressed(MouseButton::Middle) {
        pan.drag_origin = Some(cursor.position);
    }
    if buttons.just_released(MouseButton::Middle) {
        pan.drag_origin = None;
        pan.stop();
    }
}

fn mouse_motion_input(
    pan: &mut Mut<Pan>,
    mut motion_event: EventReader<MouseMotion>,
    cursor: &Res<Cursor>,
) {
    if pan.drag_origin.is_some() && motion_event.read().next().is_some() {
        pan.by_grab(cursor);
    }
}
