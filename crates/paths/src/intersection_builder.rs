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
    paths: Vec<PathIntersectionInfo>,
}

#[derive(Debug, PartialEq)]
pub enum PathIntersectionError {
    Count(usize),
    Mismatch(usize),
}

impl PathIntersectionBuilder {
    pub fn add(&mut self, entity: Entity, spline: CubicBezierSpline, info: PathSurfaceInfo) {
        self.paths.push(PathIntersectionInfo {
            entity,
            spline,
            info,
        });
    }

    pub fn build(self) -> Result<PathIntersection, PathIntersectionError> {
        if self.paths.len() < 2 {
            return Err(PathIntersectionError::Count(self.paths.len()));
        }
        let entities = start_from_intersection(self.paths)?;
        let entities = wind_ccw(entities);
        Ok(PathIntersection::new(entities))
    }
}

#[allow(clippy::indexing_slicing)]
fn start_from_intersection(
    source: Vec<PathIntersectionInfo>,
) -> Result<Vec<PathIntersectionInfo>, PathIntersectionError> {
    let mut paths = Vec::with_capacity(source.len());
    for (i, mut path) in source.into_iter().enumerate() {
        if i == 0 {
            paths.push(path);
            continue;
        }
        let start = path.spline.get_start();
        let end = path.spline.get_end();
        if i == 1 {
            let first_end = paths[0].spline.get_end();
            if first_end.is_almost_equal_to(start) || first_end.is_almost_equal_to(end) {
                paths[0].spline.reverse();
            }
        }
        let origin = paths[0].spline.get_start();
        if end.is_almost_equal_to(origin) {
            path.spline.reverse();
        } else if !start.is_almost_equal_to(origin) {
            return Err(Mismatch(i));
        }
        paths.push(path);
    }
    Ok(paths)
}

#[allow(clippy::indexing_slicing)]
fn wind_ccw(mut paths: Vec<PathIntersectionInfo>) -> Vec<PathIntersectionInfo> {
    let origin = paths[0].spline.get_start();
    paths.sort_by(|a, b| {
        let a = angle_from_x(origin, &a.spline);
        let b = angle_from_x(origin, &b.spline);
        a.partial_cmp(&b).expect("should be valid")
    });
    paths
}

fn angle_from_x(origin: Vec3, spline: &CubicBezierSpline) -> f32 {
    let param = spline
        .get_param_at_length(1.0, LENGTH_ACCURACY)
        .expect("should be valid");
    let point = spline.get_point_at_param(param);
    let vector = point - origin;
    vector.angle_between_on_plane(Vec3::X, Vec3::Z)
}
