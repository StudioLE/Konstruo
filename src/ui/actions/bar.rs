use bevy::prelude::*;

/// Bottom App Bar
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
