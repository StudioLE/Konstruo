use bevy::ecs::component::ComponentInfo;
use bevy::prelude::*;
use std::process::exit;

pub fn debug_world(world: &mut World) {
    // Iterate over all entities in the world
    let entities = world.iter_entities();
    for entity in entities {
        let components = entity
            .archetype()
            .components()
            .map(|id| {
                world
                    .components()
                    .get_info(id)
                    .expect("component should have info")
            })
            .fold(String::new(), |mut output, info| {
                output.push_str(short_name(info));
                output.push_str(", ");
                output
            });
        println!("{} ({})", entity.id(), components);
    }
    exit(1);
}

fn short_name(component: &ComponentInfo) -> &str {
    component
        .name()
        .split("::")
        .last()
        .expect("should be at least one component")
}
