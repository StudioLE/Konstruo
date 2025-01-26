use super::*;
use crate::ui::*;
use bevy::prelude::*;

/// Plugin to display a geometric grid.
pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, GridMaterials::startup_system)
            .add_systems(PostStartup, Grid::startup_system);
    }
}
