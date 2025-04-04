use super::*;
use crate::ui::{PrimaryCamera, DEFAULT_FONT, PRESSED_KEYS_Z};
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

/// Plugin to display currently pressed keys.
pub struct PressedKeysPlugin;

impl Plugin for PressedKeysPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, Self::startup_system)
            .add_systems(Update, PressedMouseButton::update_system)
            .add_systems(Update, PressedKey::update_system);
    }
}

impl PressedKeysPlugin {
    const ENABLED: Srgba = tailwind::SLATE_400;
    pub(super) const ACTIVE: Srgba = tailwind::BLUE_400;

    /// System to setup the UI for  [`PressedMouseButton`] and [`PressedKey`].
    fn startup_system(
        mut commands: Commands,
        query: Query<Entity, With<PrimaryCamera>>,
        assets: Res<AssetServer>,
    ) {
        let Ok(camera) = query.get_single() else {
            warn!("Failed to get PrimaryCamera");
            return;
        };
        let font = assets.load(DEFAULT_FONT);
        let full = commands.spawn(fullscreen(camera)).id();
        let container = commands.spawn(container()).set_parent(full).id();
        let mouse = commands
            .spawn(PressedMouseButton::container_bundle())
            .set_parent(container)
            .id();
        let row = commands
            .spawn(PressedMouseButton::buttons_row_bundle())
            .set_parent(mouse)
            .id();
        commands
            .spawn(PressedMouseButton::body_bundle())
            .set_parent(mouse);
        commands
            .spawn(PressedMouseButton::button_left_bundle())
            .set_parent(row);
        commands
            .spawn(PressedMouseButton::button_middle_bundle())
            .set_parent(row);
        commands
            .spawn(PressedMouseButton::button_right_bundle())
            .set_parent(row);
        for _index in 0..PressedKey::KEYS.len() {
            let entity = commands
                .spawn(PressedKey::key_bundle())
                .set_parent(container)
                .id();
            commands
                .spawn(PressedKey::key_label_bundle(font.clone()))
                .set_parent(entity);
        }
    }

    pub(super) fn inactive_color() -> Color {
        Self::ENABLED.with_alpha(0.25).into()
    }
}

fn fullscreen(camera: Entity) -> impl Bundle {
    (
        UiTargetCamera(camera),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::End,
            justify_content: JustifyContent::Start,
            ..default()
        },
        ZIndex(PRESSED_KEYS_Z),
        Pickable::IGNORE,
    )
}

fn container() -> impl Bundle {
    (
        Node {
            margin: UiRect::px(16.0, 16.0, 16.0, 16.0),
            column_gap: Val::Px(8.0),
            align_items: AlignItems::Center,
            ..default()
        },
        Pickable::IGNORE,
    )
}
