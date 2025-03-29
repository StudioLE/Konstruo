use crate::geometry::Vec3Helpers;
use bevy::prelude::*;
use std::ops::Neg;

#[derive(Clone)]
pub struct Subdivision {
    pub bounds: [Vec3; 4],
    pub openings: Vec<[Vec3; 4]>,
    pub main_axis: Vec3,
    pub cross_axis: Vec3,
}

impl Subdivision {
    #[must_use]
    pub fn execute(self) -> Option<Vec<[Vec3; 4]>> {
        let mut top_bound = get_edge_by_direction(&get_edges(self.bounds), self.main_axis.neg())?;
        let mut bottom_bound = get_edge_by_direction(&get_edges(self.bounds), self.main_axis)?;
        let mut rectangles = Vec::new();
        for opening in self.openings {
            let edges = get_edges(opening);
            let left = get_edge_by_direction(&edges, self.cross_axis)?;
            let right = get_edge_by_direction(&edges, self.cross_axis.neg())?;
            // Create rectangle to the left of the opening
            let rectangle = [
                bottom_bound[0],
                Vec3Helpers::project_point_to_line(left[0], bottom_bound),
                Vec3Helpers::project_point_to_line(left[0], top_bound),
                top_bound[1],
            ];
            top_bound[1] = rectangle[2];
            bottom_bound[0] = rectangle[1];
            rectangles.push(rectangle);
            // Create rectangle above the opening
            let above = [
                left[1],
                right[0],
                Vec3Helpers::project_point_to_line(right[0], top_bound),
                Vec3Helpers::project_point_to_line(left[1], top_bound),
            ];
            // Create rectangle below the opening
            let below = [
                Vec3Helpers::project_point_to_line(left[0], bottom_bound),
                Vec3Helpers::project_point_to_line(right[1], bottom_bound),
                right[1],
                left[0],
            ];
            top_bound[1] = above[2];
            bottom_bound[0] = below[1];
            rectangles.push(above);
            rectangles.push(below);
        }
        // Create final region
        let rectangle = [bottom_bound[0], bottom_bound[1], top_bound[0], top_bound[1]];
        rectangles.push(rectangle);
        Some(rectangles)
    }
}

#[allow(clippy::indexing_slicing)]
fn get_edges(rectangle: [Vec3; 4]) -> Vec<[Vec3; 2]> {
    rectangle.windows(2).map(|x| [x[0], x[1]]).collect()
}
#[allow(clippy::indexing_slicing)]
fn get_edge_by_direction(edges: &[[Vec3; 2]], direction: Vec3) -> Option<[Vec3; 2]> {
    edges
        .iter()
        .find(|&edge| {
            let vector = edge[1] - edge[0];
            let direction2 = vector.normalize();
            Vec3Helpers::is_almost_equal_to(direction, direction2)
        })
        .copied()
}
