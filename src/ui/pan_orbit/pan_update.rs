use crate::ui::pan_orbit::pan::{Pan, SPEED_MODIFIER};
use crate::ui::pan_orbit::Orbit;
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

    /// System to update [`Pan`] speed when the orbit changes
    ///
    /// Note `Changed` is also triggered by `&mut T` mutable borrows
    /// - <https://bevy-cheatbook.github.io/programming/change-detection.html#what-gets-detected>
    ///
    /// TODO: Remove mutable borrows that break Changed<Orbit> detection
    pub(super) fn orbit_changed_system(
        mut query: Query<&mut Pan>,
        orbit: Query<&Orbit, Changed<Orbit>>,
    ) {
        let Ok(orbit) = orbit.get_single() else {
            warn!("Failed to get Orbit");
            return;
        };
        for mut pan in &mut query {
            let speed = orbit.get_radius() * SPEED_MODIFIER;
            pan.translation.speed = Vec3::splat(speed);
        }
    }
}
