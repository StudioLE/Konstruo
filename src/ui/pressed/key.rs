use crate::ui::*;
use bevy::color::palettes::basic::BLACK;
use bevy::prelude::*;
use KeyCode::*;

/// A pressed key.
#[derive(Component, Debug)]
pub struct PressedKey;

/// A label for a [`PressedKey`].
#[derive(Component, Debug)]
pub struct PressedKeyLabel;

impl PressedKey {
    pub(super) const KEYS: [KeyCode; 5] = [ShiftLeft, KeyW, KeyA, KeyS, KeyD];

    /// System to set the visibility of [`PressedKey`] and the text of [`PressedKeyLabel`].
    ///
    /// Note: This is inefficient as it loops through every [`PressedKey`] every frame but this cost is
    /// acceptable given it's only intended for diagnostic and tutorial purposes.
    pub(super) fn update_system(
        mut keys: Query<(Entity, &mut Visibility), With<PressedKey>>,
        mut labels: Query<(&ChildOf, &mut Text), With<PressedKeyLabel>>,
        input: Res<ButtonInput<KeyCode>>,
    ) {
        let pressed: Vec<_> = Self::KEYS
            .iter()
            .filter(|&&code| input.pressed(code))
            .collect();
        for (index, (entity, mut visibility)) in keys.iter_mut().enumerate() {
            let Some(code) = pressed.get(index) else {
                *visibility = Visibility::Hidden;
                continue;
            };
            *visibility = Visibility::Visible;
            let Some((_, mut label)) = labels
                .iter_mut()
                .find(|(child_of, _text)| child_of.parent == entity)
            else {
                warn!("Failed to find PressedKeyLabel for PressedKey: {index}");
                continue;
            };
            *label = Text::new(Self::get_text(code));
        }
    }
    #[must_use]
    pub(super) fn key_bundle() -> impl Bundle {
        (
            PressedKey,
            Node {
                height: Val::Px(32.0),
                min_width: Val::Px(32.0),
                padding: UiRect::axes(Val::Px(8.0), Val::Px(2.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(PressedKeysPlugin::ACTIVE.into()),
            BorderRadius::all(Val::Px(8.0)),
            Pickable::IGNORE,
        )
    }

    #[must_use]
    pub(super) fn key_label_bundle(font: Handle<Font>) -> impl Bundle {
        (
            PressedKeyLabel,
            Text::new(""),
            TextColor::from(BLACK),
            TextFont {
                font,
                font_size: 16.0,
                ..default()
            },
        )
    }

    #[must_use]
    pub(super) fn get_text(key: &KeyCode) -> &str {
        match key {
            ShiftLeft => "SHIFT",
            KeyW => "W",
            KeyA => "A",
            KeyS => "S",
            KeyD => "D",
            _ => "",
        }
    }
}
