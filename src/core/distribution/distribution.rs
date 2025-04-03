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
    /// How far should the spline be offset?
    pub spline_offset: Option<f32>,
    /// Should the translation of the [`Transform`] be set so the container is at ground level.
    ///
    /// This is not applied to nested distributions.
    pub translate_to_ground: bool,
}

#[derive(Debug)]
pub enum DistributionError {
    ExceededSplineLength { actual: f32, expected: f32 },
}

impl Distribution {
    /// Distribute the [`Distributable`] children.
    pub fn distribute(
        &self,
        children: &Children,
        distributables: &mut Query<(
            Entity,
            &Distributable,
            &mut Transform,
            Option<&Distribution>,
            Option<&Children>,
        )>,
    ) -> Container {
        let children = children.to_vec();
        self.distribute_internal(&children, distributables)
    }

    /// Distribute the [`Distributable`] children.
    ///
    /// [`Entity`] can be cloned but [`Children`] can't.
    /// Therefore to avoid borrowing of query it's easier to work with the children as entities.
    fn distribute_internal(
        &self,
        children: &Vec<Entity>,
        distributables: &mut Query<(
            Entity,
            &Distributable,
            &mut Transform,
            Option<&Distribution>,
            Option<&Children>,
        )>,
    ) -> Container {
        let unsorted = process_children(children, distributables);
        let (entities, items) = sort_and_split_children(unsorted);
        let container = self.flex.execute(items);
        for (entity, distributed) in entities.iter().zip(&container.items) {
            let components = distributables.get_mut(*entity).expect("entity exists");
            let mut transform = components.2;
            if let Some(spline) = &self.spline {
                match get_transform_along_spline(spline, distributed, transform.scale) {
                    Ok(t) => {
                        *transform = t;
                    }
                    Err(DistributionError::ExceededSplineLength { actual, expected }) => {
                        error!("Failed to distribute item along a spline. The spline length is {actual:.3} but {expected:.3} was required.");
                    }
                }
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
        mut roots: Query<
            (Entity, &Distribution, &mut Transform, &Children),
            (Added<Distribution>, Without<Distributable>),
        >,
        mut containers: Query<
            (&Parent, &mut Transform),
            (
                With<DiagnosticContainer>,
                Without<Distributable>,
                Without<Distribution>,
            ),
        >,
        mut distributables: Query<(
            Entity,
            &Distributable,
            &mut Transform,
            Option<&Distribution>,
            Option<&Children>,
        )>,
    ) {
        for (entity, distribution, mut transform, children) in roots.iter_mut() {
            let container = distribution.distribute(children, &mut distributables);
            if distribution.translate_to_ground {
                translate_to_ground(&container, &mut transform);
            }
            for (parent, mut transform) in containers.iter_mut() {
                if parent.get() != entity {
                    continue;
                }
                *transform = transform.with_scale(container.size);
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
            }
        }
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

fn get_transform_along_spline(
    spline: &CubicBezierSpline,
    distributed: &Distributed,
    scale: Vec3,
) -> Result<Transform, DistributionError> {
    let spline_length = spline.get_length(LENGTH_ACCURACY);
    let distance = distributed.translation.x + spline_length * 0.5;
    if distance > spline_length {
        return Err(DistributionError::ExceededSplineLength {
            actual: spline_length,
            expected: distance,
        });
    }
    let param = spline
        .get_param_at_length(distance, LENGTH_ACCURACY)
        .expect("distance should be in range");
    let point = spline.get_point_at_param(param);
    let tangent = spline.get_tangent_at_param(param);
    let rotation = Quat::from_rotation_arc(Vec3::X, tangent);
    let translation = point
        + Transform::from_rotation(rotation).transform_point(distributed.translation.with_x(0.0));
    let transform = Transform::from_translation(translation)
        .with_rotation(rotation)
        .with_scale(scale);
    Ok(transform)
}
