use super::*;
use crate::ui::pan_orbit::orbit::SPEED_MODIFIER;
use bevy::prelude::*;

impl Pan {
    /// System to update [`Pan`] once per frame.
    pub(super) fn update_system(mut query: Query<(&mut Pan, &mut Transform)>) {
        for (mut pan, mut transform) in &mut query {
            if pan.translation.is_update_required() {
                pan.translation.update();
                *transform = pan.get_transform();
            }
        }
    }

    /// System to update [`Pan`] speed when the orbit changes
    ///
    /// Note `Changed` is also triggered by `&mut T` mutable borrows
    /// - <https://bevy-cheatbook.github.io/programming/change-detection.html#what-gets-detected>
    pub(super) fn orbit_changed_system(
        mut query: Query<&mut Pan>,
        orbit: Query<&Orbit, Changed<Orbit>>,
    ) {
        let Ok(orbit) = orbit.get_single() else {
            return;
        };
        // trace!("Updating Pan as Orbit has changed");
        for mut pan in &mut query {
            let radius = orbit.get_spherical_coordinates().get_radius();
            let speed = radius * SPEED_MODIFIER;
            pan.translation.speed = Vec3::splat(speed);
        }
    }
}
