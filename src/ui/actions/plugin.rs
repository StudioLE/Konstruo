use super::*;
use crate::ui::{PrimaryCamera, DEFAULT_FONT, GESTURE_ICON};
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

/// Plugin to enable Floating Action Buttons (FAB).
pub struct FloatingActionPlugin;

impl Plugin for FloatingActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, Self::startup_system);
    }
}

impl FloatingActionPlugin {
    fn startup_system(
        mut commands: Commands,
        assets: Res<AssetServer>,
        query: Query<Entity, With<PrimaryCamera>>,
    ) {
        let font = assets.load(DEFAULT_FONT);
        let Ok(camera) = query.get_single() else {
            warn!("Failed to get PrimaryCamera");
            return;
        };
        let bundle = (TargetCamera(camera), FloatingActionContainer);
        let container = commands.spawn(bundle).id();
        for i in 0..4 {
            let button = commands
                .spawn(FloatingActionButton)
                .set_parent(container)
                .id();
            let bundle = (
                ImageNode::new(assets.load(GESTURE_ICON)),
                Node {
                    margin: UiRect::new(Val::Px(0.0), Val::Px(12.0), Val::Px(0.0), Val::Px(0.0)),
                    ..default()
                },
            );
            commands.spawn(bundle).set_parent(button);
            let bundle = (
                Text::new(format!("Action {i}")),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor::from(tailwind::GRAY_800),
            );
            commands.spawn(bundle).set_parent(button);
        }
    }
}
