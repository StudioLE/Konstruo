use bevy::prelude::*;

/// Vertical stack of [`FloatingActionButton`].
///
/// Could alternatively be implemented as a bottom app bar:
/// - <https://m3.material.io/components/bottom-app-bar/overview>
#[derive(Component)]
#[require(Node(create_node), PickingBehavior(create_picking_behavior))]
pub struct ActionsBar;

fn create_node() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::End,
        ..default()
    }
}

fn create_picking_behavior() -> PickingBehavior {
    PickingBehavior::IGNORE
}
