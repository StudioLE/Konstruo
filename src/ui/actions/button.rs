use bevy::color::palettes::tailwind;
use bevy::prelude::*;

/// A basic icon button on the button app bar.
/// - <https://m3.material.io/components/bottom-app-bar/overview>
/// - <https://m3.material.io/components/icon-buttons/specs>
#[derive(Component)]
#[require(
    Node(create_node),
    BackgroundColor(create_background_color),
    BorderRadius(create_border_radius)
)]
pub struct ActionButton;

fn create_node() -> Node {
    Node {
        padding: UiRect::all(Val::Px(8.0)),
        overflow: Overflow::visible(),
        margin: UiRect::all(Val::Px(8.0)),
        ..default()
    }
}

fn create_background_color() -> BackgroundColor {
    BackgroundColor(tailwind::SLATE_400.into())
}

fn create_border_radius() -> BorderRadius {
    BorderRadius::all(Val::Px(12.0))
}
