use super::*;
use bevy::prelude::*;

/// Distribute [`Distributable`] children.
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct Distribution {
    pub flex: FlexFactory,
    /// Should the translation of the [`Transform`] be set so the container is at ground level.
    pub translate_to_ground: bool,
    /// Should a [`Cuboid`] mesh be generated with the container size.
    ///
    /// Material alpha can be set to `0` to hide the mesh.
    pub generate_container_mesh: bool,
}

impl Distribution {
    /// Distribute the [`Distributable`] children.
    pub fn regenerate(
        meshes: &mut ResMut<Assets<Mesh>>,
        distribution: &Distribution,
        transform: &mut Transform,
        mesh: &mut Mesh3d,
        children: &Children,
        distributables: &mut Query<(Entity, &Distributable, &mut Transform), Without<Distribution>>,
    ) {
        let (entities, items) = get_sorted_children(children, distributables);
        let container = distribution.flex.execute(items);
        if distribution.translate_to_ground {
            let translation = transform.translation.with_z(container.size.z * 0.5);
            *transform = Transform::from_translation(translation)
                .with_rotation(transform.rotation)
                .with_scale(transform.scale);
        }
        if distribution.generate_container_mesh {
            *mesh = Mesh3d(meshes.add(Cuboid::from_size(container.size)));
        }
        for (entity, distributed) in entities.iter().zip(container.items) {
            let (_, _, mut transform) = distributables.get_mut(*entity).expect("entity exists");
            // TODO: If size is different we may need to set scale
            *transform = Transform::from_translation(distributed.translation)
                .with_rotation(transform.rotation)
                .with_scale(transform.scale);
        }
    }

    /// System to create [`Mesh3d`], [`WaySurface`], and [`WayControl`] when a [`Way`] is added.
    pub fn added_system(
        mut meshes: ResMut<Assets<Mesh>>,
        mut distributions: Query<
            (&Distribution, &mut Transform, &mut Mesh3d, &Children),
            Added<Distribution>,
        >,
        mut distributables: Query<(Entity, &Distributable, &mut Transform), Without<Distribution>>,
    ) {
        for (distribution, mut transform, mut mesh, children) in distributions.iter_mut() {
            Self::regenerate(
                &mut meshes,
                distribution,
                &mut transform,
                &mut mesh,
                children,
                &mut distributables,
            );
        }
    }
}

fn get_sorted_children(
    children: &Children,
    distributables: &Query<(Entity, &Distributable, &mut Transform), Without<Distribution>>,
) -> (Vec<Entity>, Vec<Distributable>) {
    let mut children: Vec<_> = children
        .iter()
        .filter_map(|&child| distributables.get(child).ok())
        .collect();
    children.sort_by_key(|(_, distributable, _)| distributable.order);
    let mut entities = Vec::with_capacity(children.len());
    let mut items = Vec::with_capacity(children.len());
    for (entity, distributable, _transform) in children {
        entities.push(entity);
        items.push(distributable.clone());
    }
    (entities, items)
}
