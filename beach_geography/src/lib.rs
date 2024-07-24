use crate::sun::spawn_sun;
use bevy::app::{App, Startup};

pub mod ground;
pub mod sun;

/// Register systems for environment.
pub fn environment_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_sun);
}
