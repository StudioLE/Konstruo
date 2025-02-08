use super::*;
use crate::beziers::CubicBezierSpline;
use crate::infrastructure::LENGTH_ACCURACY;
use bevy::prelude::*;

/// How children with the [`Distributable`] component are to be distributed.
#[derive(Clone, Component, Debug, Default)]
#[require(InheritedVisibility, Transform)]
pub struct Distribution {
    pub flex: FlexFactory,
    /// Should the items be distributed along a spline?
    pub spline: Option<CubicBezierSpline>,
    /// Should the translation of the [`Transform`] be set so the container is at ground level.
    ///
    /// This is not applied to nested distributions.
    pub translate_to_ground: bool,
    /// Should a [`Cuboid`] mesh be generated with the container size.
    ///
    /// This is not applied to nested distributions.
    pub generate_container_mesh: bool,
}

impl Distribution {
    /// Distribute the [`Distributable`] children.
    pub fn distribute(
        &self,
        children: &Children,
        query: &mut Query<(
            Entity,
            &Distributable,
            &mut Transform,
            Option<&Distribution>,
            Option<&Children>,
        )>,
    ) -> Container {
        let children = children.to_vec();
        self.distribute_internal(&children, query)
    }

    /// Distribute the [`Distributable`] children.
    ///
    /// [`Entity`] can be cloned but [`Children`] can't.
    /// Therefore to avoid borrowing of query it's easier to work with the children as entities.
    fn distribute_internal(
        &self,
        children: &Vec<Entity>,
        query: &mut Query<(
            Entity,
            &Distributable,
            &mut Transform,
            Option<&Distribution>,
            Option<&Children>,
        )>,
    ) -> Container {
        let unsorted = process_children(children, query);
        let (entities, items) = sort_and_split_children(unsorted);
        let container = self.flex.execute(items);
        for (entity, distributed) in entities.iter().zip(&container.items) {
            let components = query.get_mut(*entity).expect("entity exists");
            let mut transform = components.2;
            if let Some(spline) = &self.spline {
                *transform = get_transform_along_spline(spline, distributed, transform.scale);
            } else {
                *transform = Transform::from_translation(distributed.translation)
                    .with_rotation(transform.rotation)
                    .with_scale(transform.scale);
            }
        }
        container
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
            Option<&Children>,
        )>,
    ) {
        for (distribution, mut transform, mesh, children) in roots.iter_mut() {
            let container = distribution.distribute(children, &mut distributables);
            if distribution.translate_to_ground {
                translate_to_ground(&container, &mut transform);
            }
            if distribution.generate_container_mesh {
                if let Some(mut mesh) = mesh {
                    replace_cuboid_mesh(&mut meshes, &container, &mut mesh);
                }
            }
        }
    }

    /// System to distribute children with the [`Distributable`] component when a root [`Distribution`] is changed.
    ///
    /// A root [`Distribution`] is one that is not itself [`Distributable`].
    pub fn changed_system(
        mut roots: Query<
            (&Distribution, &Children),
            (Changed<Distribution>, Without<Distributable>),
        >,
        mut distributables: Query<(
            Entity,
            &Distributable,
            &mut Transform,
            Option<&Distribution>,
            Option<&Children>,
        )>,
    ) {
        for (distribution, children) in roots.iter_mut() {
            let _container = distribution.distribute(children, &mut distributables);
        }
    }
}

/// For each child entity:
/// - If [`Distribution`] is present then recursively distribute the nested children
/// - Get clones of [`Entity`] and [`Distributable`].
/// - Update the [`Distributable`] size if its children have been distributed
fn process_children(
    children: &Vec<Entity>,
    query: &mut Query<(
        Entity,
        &Distributable,
        &mut Transform,
        Option<&Distribution>,
        Option<&Children>,
    )>,
) -> Vec<(Entity, Distributable)> {
    let mut unsorted = Vec::new();
    for entity in children {
        let Ok(components) = query.get(*entity) else {
            continue;
        };
        let mut distributable = components.1.clone();
        let distribution = components.3.cloned();
        let nested_children = components.4.map(|x| x.to_vec());
        if let Some(distribution) = distribution {
            if let Some(nested_children) = nested_children {
                let container = distribution.distribute_internal(&nested_children, query);
                distributable.size = Some(container.size);
            } else {
                warn!("Entity has `Distribution` component but no children: {entity}");
            };
        };
        unsorted.push((*entity, distributable));
    }
    unsorted
}

/// For each of the [`Children`]:
/// - Sort according to [`Distributable`] order
/// - into separate [`Vec`] with matching index order
fn sort_and_split_children(
    mut unsorted: Vec<(Entity, Distributable)>,
) -> (Vec<Entity>, Vec<Distributable>) {
    unsorted.sort_by_key(|entity| entity.1.order);
    let mut entities = Vec::with_capacity(unsorted.len());
    let mut distributables = Vec::with_capacity(unsorted.len());
    for (entity, distributable) in unsorted {
        entities.push(entity);
        distributables.push(distributable);
    }
    (entities, distributables)
}

/// Update the translation of the [`Transform`] so the container is at ground level.
fn translate_to_ground(container: &Container, transform: &mut Transform) {
    let translation = transform.translation.with_z(container.size.z * 0.5);
    *transform = Transform::from_translation(translation)
        .with_rotation(transform.rotation)
        .with_scale(transform.scale);
}

/// Replace the [`Mesh3d`] with a [`Cuboid`] mesh scaled to the container size.
fn replace_cuboid_mesh(
    meshes: &mut ResMut<Assets<Mesh>>,
    container: &Container,
    mesh: &mut Mesh3d,
) {
    let cuboid = Cuboid::from_size(container.size);
    *mesh = Mesh3d(meshes.add(cuboid));
}

fn get_transform_along_spline(
    spline: &CubicBezierSpline,
    distributed: &Distributed,
    scale: Vec3,
) -> Transform {
    let spline_length = spline.get_length(LENGTH_ACCURACY);
    let distance = distributed.translation.x + spline_length * 0.5;
    let param = spline
        .get_param_at_length(distance, LENGTH_ACCURACY)
        .expect("distance should be in range");
    let point = spline.get_point_at_param(param);
    let tangent = spline.get_tangent_at_param(param);
    let rotation = Quat::from_rotation_arc(Vec3::X, tangent);
    let translation = point
        + Transform::from_rotation(rotation).transform_point(distributed.translation.with_x(0.0));
    Transform::from_translation(translation)
        .with_rotation(rotation)
        .with_scale(scale)
}
