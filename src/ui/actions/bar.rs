use bevy::color::palettes::tailwind;
use bevy::prelude::*;

/// Bottom App Bar
/// - <https://m3.material.io/components/bottom-app-bar/overview>
#[derive(Component)]
#[require(
    BackgroundColor(create_background_color),
    Node(create_node),
    PickingBehavior(create_picking_behavior)
)]
pub struct ActionsBar;

fn create_background_color() -> BackgroundColor {
    BackgroundColor(tailwind::GRAY_200.into())
}

fn create_node() -> Node {
    Node {
        flex_grow: 1.0,
        justify_content: JustifyContent::Start,
        padding: UiRect::new(Val::Px(4.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
        align_items: AlignItems::Center,
        ..default()
    }
}

fn create_picking_behavior() -> PickingBehavior {
    PickingBehavior::IGNORE
}
