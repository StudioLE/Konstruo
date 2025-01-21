use crate::cameras::orbit::Orbit;
use bevy::prelude::{Query, Transform};

impl Orbit {
    /// System to update the movement once per frame.
    pub fn update_system(mut query: Query<(&mut Orbit, &mut Transform)>) {
        for (mut orbit, mut transform) in &mut query {
            if orbit.movement.update() {
                *transform = orbit.get_transform();
            }
        }
    }
}
