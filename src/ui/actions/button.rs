use bevy::color::palettes::tailwind;
use bevy::prelude::*;

/// - <https://m3.material.io/components/floating-action-button/specs#7712fa7f-cd29-4852-86a9-fa2f4a01f6bc>
#[derive(Component)]
#[require(
    Node(create_node),
    BackgroundColor(create_background_color),
    BorderRadius(create_border_radius)
)]
pub struct FloatingActionButton;

fn create_node() -> Node {
    Node {
        width: Val::Px(24.0),
        height: Val::Px(24.0),
        padding: UiRect::all(Val::Px(16.0)),
        margin: UiRect::new(Val::Px(16.0), Val::Px(16.0), Val::Px(0.0), Val::Px(16.0)),
        flex_shrink: 0.0,
        ..default()
    }
}

fn create_background_color() -> BackgroundColor {
    BackgroundColor(tailwind::SKY_600.into())
}

fn create_border_radius() -> BorderRadius {
    BorderRadius::all(Val::Px(8.0))
}
