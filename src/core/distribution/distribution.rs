use super::*;
use bevy::prelude::*;

/// How children with the [`Distributable`] component are to be distributed.
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
        mesh: &mut Option<Mut<Mesh3d>>,
        children: &Children,
        query: &mut Query<(
            Entity,
            &Distributable,
            &mut Transform,
            Option<&Distribution>,
            Option<&Mesh3d>,
            Option<&Children>,
        )>,
    ) {
        let (entities, items) = get_sorted_children(children, query);
        // TODO: Loop through child entities with Distribution and regenerate them first
        let container = distribution.flex.execute(items);
        if distribution.translate_to_ground {
            let translation = transform.translation.with_z(container.size.z * 0.5);
            *transform = Transform::from_translation(translation)
                .with_rotation(transform.rotation)
                .with_scale(transform.scale);
        }
        if distribution.generate_container_mesh {
            if let Some(mut mesh) = mesh.take() {
                *mesh = Mesh3d(meshes.add(Cuboid::from_size(container.size)));
            }
        }
        for (entity, distributed) in entities.iter().zip(container.items) {
            let components = query.get_mut(*entity).expect("entity exists");
            let mut transform = components.2;
            // TODO: If size is different we may need to set scale
            *transform = Transform::from_translation(distributed.translation)
                .with_rotation(transform.rotation)
                .with_scale(transform.scale);
        }
    }

    /// System to distribute children with the [`Distributable`] component when a root [`Distribution`] is added.
    ///
    /// A root [`Distribution`] is one that is not itself [`Distributable`].
    pub fn added_system(
        mut meshes: ResMut<Assets<Mesh>>,
        mut roots: Query<
            (
                &Distribution,
                &mut Transform,
                Option<&mut Mesh3d>,
                &Children,
            ),
            (Added<Distribution>, Without<Distributable>),
        >,
        mut distributables: Query<(
            Entity,
            &Distributable,
            &mut Transform,
            Option<&Distribution>,
            Option<&Mesh3d>,
            Option<&Children>,
        )>,
    ) {
        for (distribution, mut transform, mut mesh, children) in roots.iter_mut() {
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

/// For each of the [`Children`]:
/// - Get the components
/// - Sort them in order
///
/// Returns multiple [`Vec`] with the matching indexes:
/// - [`Entity`]
/// - [`Distributable`]
fn get_sorted_children(
    children: &Children,
    query: &Query<(
        Entity,
        &Distributable,
        &mut Transform,
        Option<&Distribution>,
        Option<&Mesh3d>,
        Option<&Children>,
    )>,
) -> (Vec<Entity>, Vec<Distributable>) {
    let mut children: Vec<_> = children
        .iter()
        .filter_map(|&child| query.get(child).ok())
        .collect();
    children.sort_by_key(|entity| entity.1.order);
    let mut entities = Vec::with_capacity(children.len());
    let mut distributables = Vec::with_capacity(children.len());
    for child in children {
        entities.push(child.0);
        distributables.push(child.1.clone());
    }
    (entities, distributables)
}
