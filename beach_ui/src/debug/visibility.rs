use bevy::prelude::*;

#[allow(clippy::type_complexity)]
pub fn debug_visibility_hierarchy(
    keys: Res<ButtonInput<KeyCode>>,
    query: Query<(
        Entity,
        Option<&Parent>,
        Option<&Visibility>,
        Option<&InheritedVisibility>,
    )>,
) {
    if !keys.just_pressed(KeyCode::KeyP) {
        return;
    }
    for (entity, parent, visibility, inherited_visibility) in query.iter() {
        println!(
            "Entity: {:?}, Parent: {:?}, Visibility: {:?}, InheritedVisibility: {:?}",
            entity,
            parent.map(|p| p.get()),
            visibility.is_some(),
            inherited_visibility.is_some()
        );
    }
}
