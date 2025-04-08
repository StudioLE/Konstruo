use super::*;
use crate::pan_orbit::orbit::SPEED_MODIFIER;
use bevy::prelude::{Query, Transform};

impl Orbit {
    /// System to update [`Orbit`] once per frame.
    pub fn update_system(mut query: Query<(&mut Orbit, &mut Transform)>) {
        for (mut orbit, mut transform) in &mut query {
            if orbit.translation.is_update_required() {
                orbit.translation.update();
                let radius = orbit.get_spherical_coordinates().get_radius();
                orbit.translation.speed.x = radius * SPEED_MODIFIER;
                *transform = orbit.get_cartesian_transform();
            }
        }
    }
}
