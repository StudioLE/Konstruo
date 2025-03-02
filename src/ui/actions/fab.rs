use bevy::color::palettes::tailwind;
use bevy::prelude::*;

/// A floating action button on the bottom app bar.
/// - <https://m3.material.io/components/bottom-app-bar/overview>
/// - <https://m3.material.io/components/floating-action-button/overview>
#[derive(Component)]
#[require(
    Node(create_node),
    BackgroundColor(create_background_color),
    BorderRadius(create_border_radius)
)]
pub struct FloatingActionButton;

fn create_node() -> Node {
    Node {
        padding: UiRect::all(Val::Px(16.0)),
        overflow: Overflow::visible(),
        margin: UiRect::new(Val::Auto, Val::Px(16.0), Val::Px(16.0), Val::Px(16.0)),
        ..default()
    }
}

fn create_background_color() -> BackgroundColor {
    BackgroundColor(tailwind::BLUE_300.into())
}

fn create_border_radius() -> BorderRadius {
    BorderRadius::all(Val::Px(8.0))
}
