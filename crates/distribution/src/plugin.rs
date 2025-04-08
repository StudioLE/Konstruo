use super::*;
use bevy::app::{App, Update};
use bevy::prelude::*;

pub struct DistributionPlugin;

impl Plugin for DistributionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Distribution::added_system)
            .add_systems(Update, Distribution::changed_system);
    }
}
