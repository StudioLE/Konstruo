use bevy::prelude::*;

/// Parent to position [`ActionsBar`] at the bottom of the screen.
#[derive(Component)]
#[require(Node(create_node), PickingBehavior(create_picking_behavior))]
pub struct ActionsBarParent;

fn create_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::End,
        justify_content: JustifyContent::End,
        ..default()
    }
}

fn create_picking_behavior() -> PickingBehavior {
    PickingBehavior::IGNORE
}
