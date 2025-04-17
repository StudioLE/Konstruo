use crate::*;
use bevy::prelude::*;
use konstruo_beziers::constants::*;
use konstruo_beziers::CubicBezier;
use konstruo_beziers::*;
use konstruo_geometry::Line;
use ControlType::End;
use PathSurfacePosition::*;

/// An intersection between [`Path`].
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct PathIntersection {
    pub paths: Vec<PathIntersectionInfo>,
}

pub struct PathIntersectionInfo {
    pub entity: Entity,
    pub spline: CubicBezierSpline,
    pub info: PathSurfaceInfo,
}

impl PathIntersectionInfo {
    fn get_offset_value(&self, inside: bool) -> f32 {
        if let Offset(mut offset) = self.info.position {
            if inside {
                offset *= -1.0;
            }
            self.info.width * 0.5 + offset
        } else {
            self.info.width * 0.5
        }
    }
}

impl PathIntersection {
    pub(super) fn new(entities: Vec<PathIntersectionInfo>) -> Self {
        Self { paths: entities }
    }

    pub fn get_centered_polygon(&self) -> Result<CubicBezierSpline, CubicBezierSplineError> {
        let mut curves: Vec<CubicBezier> = Vec::new();
        for i in 0..self.paths.len() {
            let corner = self.get_offset_corner(i, false)?;
            if i != 0 {
                let start = curves
                    .last()
                    .expect("should be at least one curve")
                    .get_control(End);
                let end = corner.get_start();
                curves.push(
                    CubicBezier::from_line(Line::new(start, end)).expect("bezier should be valid"),
                );
            }
            curves.append(&mut corner.to_curves());
        }
        CubicBezierSpline::new(curves)
    }

    fn get_offset_corner(
        &self,
        index: usize,
        inside: bool,
    ) -> Result<CubicBezierSpline, CubicBezierSplineError> {
        let index_b = if index + 1 < self.paths.len() {
            index + 1
        } else {
            0
        };
        let path_a = self.paths.get(index).expect("index should be valid");
        let dist_a = path_a.get_offset_value(inside) * -1.0;
        let mut spline_a = path_a.spline.offset(dist_a, OFFSET_ACCURACY)?;
        spline_a.reverse();
        let path_b = self.paths.get(index_b).expect("next index should be valid");
        let dist_b = path_b.get_offset_value(inside);
        let spline_b = path_b.spline.offset(dist_b, OFFSET_ACCURACY)?;
        let end_a = spline_a.get_end();
        let start_b = spline_b.get_start();
        if end_a.abs_diff_eq(start_b, CONNECTION_TOLERANCE) {
            let mut curves = spline_a.to_curves();
            curves.append(&mut spline_b.to_curves());
            return CubicBezierSpline::new(curves);
        }
        let Some(intersections) =
            spline_a.get_intersections_with_spline(&spline_b, INTERSECTION_TOLERANCE)
        else {
            todo!("Fillet between the ends of non-intersecting splines");
        };
        if intersections.len() > 1 {
            todo!("Sort intersections to find the closest to the end");
        }
        let intersection = *intersections.first().expect("should be at least one");
        let param_a = spline_a.get_param_nearest_to(intersection, INTERSECTION_ACCURACY);
        let param_b = spline_b.get_param_nearest_to(intersection, INTERSECTION_ACCURACY);
        let [split_a, _] = spline_a.split_at_param(param_a)?;
        let [_, split_b] = spline_b.split_at_param(param_b)?;
        let mut curves = split_a.to_curves();
        curves.append(&mut split_b.to_curves());
        CubicBezierSpline::new(curves)
    }
}
