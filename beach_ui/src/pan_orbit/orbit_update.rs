use crate::pan_orbit::Orbit;
use bevy::prelude::{Query, Transform};

impl Orbit {
    /// System to update [`Orbit`] once per frame.
    pub fn update_system(mut query: Query<(&mut Orbit, &mut Transform)>) {
        for (mut orbit, mut transform) in &mut query {
            if orbit.translation.update() {
                *transform = orbit.get_cartesian_transform();
            }
        }
    }
}
