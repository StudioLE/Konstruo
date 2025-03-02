use bevy::prelude::*;

#[derive(Component)]
#[require(Node(create_node), PickingBehavior(create_picking_behavior))]
pub struct FloatingActionContainer;

fn create_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::End,
        align_content: AlignContent::End,
        align_items: AlignItems::End,
        flex_wrap: FlexWrap::NoWrap,
        ..default()
    }
}

fn create_picking_behavior() -> PickingBehavior {
    PickingBehavior::IGNORE
}
