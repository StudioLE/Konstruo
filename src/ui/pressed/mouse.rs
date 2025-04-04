use crate::ui::*;
use bevy::prelude::*;
use MouseButton::*;

/// A pressed mouse button.
#[derive(Component)]
pub struct PressedMouseButton {
    pub button: MouseButton,
}

impl PressedMouseButton {
    /// System to update the [`BackgroundColor`] of [`PressedMouseButton`].
    pub(super) fn update_system(
        mut buttons: Query<(&PressedMouseButton, &mut BackgroundColor)>,
        input: Res<ButtonInput<MouseButton>>,
    ) {
        for (button, mut bg) in buttons.iter_mut() {
            let color = if input.pressed(button.button) {
                PressedKeysPlugin::ACTIVE.into()
            } else {
                PressedKeysPlugin::inactive_color()
            };
            *bg = BackgroundColor(color);
        }
    }

    #[must_use]
    pub(super) fn container_bundle() -> impl Bundle {
        (
            Node {
                align_items: AlignItems::Start,
                justify_content: JustifyContent::End,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                ..default()
            },
            Pickable::IGNORE,
        )
    }

    #[must_use]
    pub(super) fn buttons_row_bundle() -> impl Bundle {
        (
            Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                column_gap: Val::Px(4.0),
                ..default()
            },
            Pickable::IGNORE,
        )
    }

    #[must_use]
    pub(super) fn body_bundle() -> impl Bundle {
        (
            Node {
                width: Val::Px(48.0),
                height: Val::Px(48.0),
                ..default()
            },
            BackgroundColor(PressedKeysPlugin::inactive_color()),
            BorderRadius::bottom(Val::Px(8.0)),
            Pickable::IGNORE,
        )
    }

    #[must_use]
    pub fn button_left_bundle() -> impl Bundle {
        (
            PressedMouseButton { button: Left },
            Node {
                height: Val::Px(32.0),
                width: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(PressedKeysPlugin::inactive_color()),
            BorderRadius::top_left(Val::Px(8.0)),
            Pickable::IGNORE,
        )
    }

    #[must_use]
    pub(super) fn button_middle_bundle() -> impl Bundle {
        (
            PressedMouseButton { button: Middle },
            Node {
                height: Val::Px(24.0),
                width: Val::Px(8.0),
                ..default()
            },
            BackgroundColor(PressedKeysPlugin::inactive_color()),
            BorderRadius::all(Val::Px(8.0)),
            Pickable::IGNORE,
        )
    }

    #[must_use]
    pub(super) fn button_right_bundle() -> impl Bundle {
        (
            PressedMouseButton { button: Right },
            Node {
                height: Val::Px(32.0),
                width: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(PressedKeysPlugin::inactive_color()),
            BorderRadius::top_right(Val::Px(12.0)),
            Pickable::IGNORE,
        )
    }
}
