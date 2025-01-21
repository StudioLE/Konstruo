use crate::cameras::pan::Pan;
use bevy::prelude::*;

impl Pan {
    /// System to update the [`Pan`] movement once per frame.
    pub fn update_system(mut query: Query<(&mut Pan, &mut Transform)>) {
        for (mut pan, mut transform) in &mut query {
            if pan.movement.update() {
                *transform = pan.get_transform();
            }
        }
    }
}
