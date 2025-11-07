use bevy::prelude::*;

pub fn debug_visibility_hierarchy(
    keys: Res<ButtonInput<KeyCode>>,
    query: Query<(
        Entity,
        Option<&ChildOf>,
        Option<&Visibility>,
        Option<&InheritedVisibility>,
    )>,
) {
    if !keys.just_pressed(KeyCode::KeyP) {
        return;
    }
    for (entity, parent, visibility, inherited_visibility) in query.iter() {
        println!(
            "Entity: {:?}, ChildOf: {:?}, Visibility: {:?}, InheritedVisibility: {:?}",
            entity,
            parent.map(|x| x.parent()),
            visibility.is_some(),
            inherited_visibility.is_some()
        );
    }
}
