use crate::pan_orbit::pan::Pan;
use bevy::prelude::*;

impl Pan {
    /// System to update [`Pan`] once per frame.
    pub(super) fn update_system(mut query: Query<(&mut Pan, &mut Transform)>) {
        for (mut pan, mut transform) in &mut query {
            if pan.translation.update() {
                *transform = pan.get_transform();
            }
        }
    }
}
