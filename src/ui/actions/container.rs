use bevy::prelude::*;

#[derive(Component)]
#[require(Node(create_node))]
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
