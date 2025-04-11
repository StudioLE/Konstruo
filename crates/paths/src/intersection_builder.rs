use crate::*;
use bevy::prelude::*;
use konstruo_beziers::constants::LENGTH_ACCURACY;
use konstruo_beziers::*;
use konstruo_core::Vec3Extensions;
use PathIntersectionError::Mismatch;

/// An intersection between [`Path`].
#[derive(Component, Default)]
#[require(InheritedVisibility, Transform)]
pub struct PathIntersectionBuilder {
    entities: Vec<(Entity, CubicBezierSpline)>,
}

#[derive(Debug, PartialEq)]
pub enum PathIntersectionError {
    Count(usize),
    Mismatch(usize),
}

impl PathIntersectionBuilder {
    pub fn add(&mut self, path: Entity, spline: CubicBezierSpline) {
        self.entities.push((path, spline));
    }

    pub fn build(self) -> Result<PathIntersection, PathIntersectionError> {
        if self.entities.len() < 2 {
            return Err(PathIntersectionError::Count(self.entities.len()));
        }
        let entities = start_from_intersection(self.entities)?;
        let entities = wind_ccw(entities)?;
        Ok(PathIntersection::new(entities))
    }
}

fn start_from_intersection(
    source: Vec<(Entity, CubicBezierSpline)>,
) -> Result<Vec<(Entity, CubicBezierSpline)>, PathIntersectionError> {
    let mut entities = Vec::with_capacity(source.len());
    for (i, (entity, mut spline)) in source.into_iter().enumerate() {
        if i == 0 {
            entities.push((entity, spline));
            continue;
        }
        let start = spline.get_start();
        let end = spline.get_end();
        if i == 1 {
            let first_end = entities[0].1.get_end();
            if first_end.is_almost_equal_to(start) || first_end.is_almost_equal_to(end) {
                entities[0].1.reverse();
            }
        }
        let origin = entities[0].1.get_start();
        if end.is_almost_equal_to(origin) {
            spline.reverse();
        } else if !start.is_almost_equal_to(origin) {
            return Err(Mismatch(i));
        }
        entities.push((entity, spline));
    }
    Ok(entities)
}

fn wind_ccw(
    mut entities: Vec<(Entity, CubicBezierSpline)>,
) -> Result<Vec<(Entity, CubicBezierSpline)>, PathIntersectionError> {
    let origin = entities[0].1.get_start();
    entities.sort_by(|(_, a), (_, b)| {
        let a = angle_from_x(origin, a);
        let b = angle_from_x(origin, b);
        a.partial_cmp(&b).expect("should be valid")
    });
    Ok(entities)
}

fn angle_from_x(origin: Vec3, spline: &CubicBezierSpline) -> f32 {
    let param = spline
        .get_param_at_length(1.0, LENGTH_ACCURACY)
        .expect("should be valid");
    let point = spline.get_point_at_param(param);
    let vector = point - origin;
    vector.angle_between_on_plane(Vec3::X, Vec3::Z)
}
