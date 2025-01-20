use crate::cameras::orbit::Orbit;
use crate::cameras::primary_camera::PrimaryCamera;
use crate::tools::cursor::get_cursor_position;
use beach_core::constraints::clamp_float::ClampFloat;
use beach_core::constraints::clamp_vec3::ClampVec3;
use beach_core::movement::target_based_movement::TargetBasedMovement;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use KeyCode::*;

/// Pan state of the camera.
#[derive(Component)]
#[require(InheritedVisibility)]
pub struct Pan {
    pub movement: TargetBasedMovement,
    /// Is dragging mode currently active?
    /// The value is the cursor position on the XY plane when dragging was started.
    pub dragging: Option<Vec3>,
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
                speed: Vec3::splat(500.0),
            },
            dragging: None,
        }
    }
}

impl Pan {
    /// Get the position.
    pub fn get_transform(&self) -> Transform {
        Transform::from_translation(self.movement.current)
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

    fn by_grab(
        &mut self,
        mut transform: Mut<Transform>,
        cursor_position: Vec3,
    ) {
        let Some(drag_origin) = self.dragging else {
            warn!("Failed to get drag origin");
            return;
        };
        let translation = drag_origin - cursor_position;    
        self.movement.set_target_relative_to_position(translation);
        let target = self.movement.current + translation;
        *transform = Transform::from_translation(target);
    }

    fn stop(&mut self) {
        self.movement.remove_target();
    }
}

/// Update the movement once per frame.
pub fn on_update(mut query: Query<(&mut Pan, &mut Transform)>) {
    for (mut pan, mut transform) in &mut query {
        if pan.movement.update() {
            *transform = pan.get_transform();
        }
    }
}

/// Respond to input events.
#[allow(clippy::too_many_arguments)]
pub fn on_input(
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

fn keyboard_input(pan: &mut Mut<Pan>, orbit: &Orbit, keys: Res<ButtonInput<KeyCode>>) {
    if !keys.pressed(ShiftLeft) {
        if keys.pressed(KeyW) {
            pan.in_direction(relative_direction(orbit, Vec3::Y), 1.0);
        }
        if keys.pressed(KeyA) {
            pan.in_direction(relative_direction(orbit, Vec3::X) * -1.0, 1.0);
        }
        if keys.pressed(KeyS) {
            pan.in_direction(relative_direction(orbit, Vec3::Y) * -1.0, 1.0);
        }
        if keys.pressed(KeyD) {
            pan.in_direction(relative_direction(orbit, Vec3::X), 1.0);
        }
    }
    if keys.any_just_released([KeyW, KeyA, KeyS, KeyD]) {
        pan.stop();
    }
}

/// Get a direction relative to the camera position
///
/// Project it on the XY plane
pub fn relative_direction(orbit: &Orbit, direction: Vec3) -> Vec3 {
    orbit
        .get_rotation()
        .mul_vec3(direction)
        .with_z(0.0)
        .normalize()
}

fn mouse_button_input(
    pan: &mut Mut<Pan>,
    window: &Query<&Window, With<PrimaryWindow>>,
    camera: &Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Middle) {
        if let Ok(position) = get_cursor_position(window, camera) {
            pan.dragging = Some(position)
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
        if let Ok(position) = get_cursor_position(window, camera) {
            pan.by_grab(transform, position)
        };
    }
}
