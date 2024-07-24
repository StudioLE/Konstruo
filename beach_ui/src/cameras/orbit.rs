use beach_core::constraints::clamp_float::ClampFloat;
use beach_core::constraints::clamp_vec3::ClampVec3;
use beach_core::mathematics::constants::*;
use beach_core::mathematics::spherical_coordinate_system::*;
use beach_core::movement::target_based_movement::TargetBasedMovement;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use KeyCode::*;

/// Orbital state of the camera.
#[derive(Component)]
pub struct Orbit {
    pub movement: TargetBasedMovement,
    pub dragging: bool,
}

impl Default for Orbit {
    fn default() -> Self {
        Self {
            movement: TargetBasedMovement {
                current: Vec3::new(250.0, 0.0, 0.0),
                clamp: ClampVec3 {
                    x: ClampFloat::Fixed(10.0, 2500.0),
                    y: ClampFloat::Fixed(0.0, HALF_PI),
                    z: ClampFloat::Wrapped(TWO_PI),
                },
                target: None,
                speed: Vec3::new(250.0, HALF_PI, PI),
            },
            dragging: false,
        }
    }
}

impl Orbit {
    /// Distance from the origin in metres.
    pub fn get_radius(&self) -> f32 {
        self.movement.current.x
    }

    /// Angle from the zenith in radians.
    pub fn get_polar(&self) -> f32 {
        self.movement.current.y
    }

    /// Angle from the zenith in radians.
    pub fn get_altitude(&self) -> f32 {
        HALF_PI - self.get_polar()
    }

    /// Angle from Z to the entity in the XZ plane.
    pub fn get_azimuth(&self) -> f32 {
        self.movement.current.z
    }

    /// Get the position and rotation of the camera.
    pub fn get_transform(&self) -> Transform {
        let mut transform = Transform::from_translation(self.get_position());
        transform.rotation = self.get_rotation();
        transform
    }

    /// Get the position relative to the origin.
    fn get_position(&self) -> Vec3 {
        SphericalCoordinateSystem::spherical_to_cartesian(
            self.get_radius(),
            self.get_polar(),
            self.get_azimuth(),
        )
    }

    /// Get the rotation looking to the origin.
    fn get_rotation(&self) -> Quat {
        let y = self.get_azimuth();
        let x = self.get_altitude() * -1.0;
        Quat::from_euler(EulerRot::YXZ, y, x, 0.0)
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
    fn in_direction_of_motion(&mut self, mouse: MouseMotion) {
        let direction = mouse.delta.normalize();
        let polar = direction.y * -1.0 * 0.1;
        let azimuthal = direction.x * -1.0 * 0.04;
        let displacement = Vec3::new(0.0, polar, azimuthal);
        self.movement.set_target_relative_to_position(displacement);
    }

    fn stop(&mut self) {
        self.movement.remove_target();
    }
}

/// Update the movement once per frame.
pub fn on_update(mut query: Query<&mut Orbit>) {
    for mut orbit in &mut query {
        orbit.update();
    }
}

/// Update the transform if the position changes.
pub fn on_changed(mut query: Query<(&mut Transform, &Orbit), Changed<Orbit>>) {
    for (mut transform, orbit) in &mut query {
        *transform = orbit.get_transform();
    }
}

/// Respond to input events.
pub fn on_input(
    mut query: Query<&mut Orbit>,
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    wheel_event: EventReader<MouseWheel>,
    motion_event: EventReader<MouseMotion>,
) {
    if let Ok(mut orbit) = query.get_single_mut() {
        let left_shift_pressed = keys.pressed(ShiftLeft);
        keyboard_input(&mut orbit, keys);
        mouse_button_input(&mut orbit, buttons);
        if left_shift_pressed {
            mouse_motion_input(&mut orbit, motion_event);
            scroll_wheel_input(&mut orbit, wheel_event);
        }
    }
}

fn keyboard_input(orbit: &mut Mut<Orbit>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.pressed(ShiftLeft) {
        if keys.pressed(KeyW) {
            orbit.in_direction(POLAR_AXIS * -1.0, 0.1);
        }
        if keys.pressed(KeyA) {
            orbit.in_direction(AZIMUTHAL_AXIS * -1.0, 0.1);
        }
        if keys.pressed(KeyS) {
            orbit.in_direction(POLAR_AXIS, 0.1);
        }
        if keys.pressed(KeyD) {
            orbit.in_direction(AZIMUTHAL_AXIS, 0.1);
        }
    }
    if keys.any_just_released([KeyW, KeyA, KeyS, KeyD, Equal, Minus]) {
        orbit.stop();
    }
    if keys.pressed(Minus) {
        orbit.in_direction(RADIAL_AXIS, 0.1);
    }
    if keys.pressed(Equal) {
        orbit.in_direction(RADIAL_AXIS * -1.0, 0.1);
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
            MouseScrollUnit::Line => scroll_wheel.y,
            MouseScrollUnit::Pixel => scroll_wheel.y,
        };
        let direction = if vertical == 0.0 {
            return;
        } else if vertical > 0.0 {
            -1.0
        } else {
            1.0
        };
        orbit.in_direction(RADIAL_AXIS * direction, 0.2);
    }
}
