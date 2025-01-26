use super::*;
use crate::ui::pan_orbit::orbit::SPEED_MODIFIER;
use bevy::prelude::{Query, Transform};

impl Orbit {
    /// System to update [`Orbit`] once per frame.
    pub fn update_system(mut query: Query<(&mut Orbit, &mut Transform)>) {
        for (mut orbit, mut transform) in &mut query {
            if orbit.translation.update() {
                orbit.translation.speed.x = orbit.get_radius() * SPEED_MODIFIER;
                *transform = orbit.get_cartesian_transform();
            }
        }
    }
}
