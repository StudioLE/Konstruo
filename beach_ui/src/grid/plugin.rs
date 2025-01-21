use crate::grid::Grid;
use bevy::app::{App, Plugin, Startup};

/// Plugin to display a geometric grid.
pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Grid::startup_system);
    }
}
