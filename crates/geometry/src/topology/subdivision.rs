use crate::Polygon;
use bevy::prelude::*;
use konstruo_core::Vec3Helpers;
use std::ops::Neg;
// TODO: Rename to RectangularOpeningSubtraction

/// A strategy to subdivide a rectangle into multiple rectangle by subtracting openings.
#[derive(Clone)]
pub struct Subdivision {
    pub bounds: [Vec3; 4],
    pub openings: Vec<[Vec3; 4]>,
    pub main_axis: Vec3,
    pub cross_axis: Vec3,
}

#[derive(Debug)]
pub enum SubdivisionError {
    BoundWinding,
    GetBoundTop,
    GetBoundBottom,
    OpeningWinding(usize),
    GetOpeningLeft(usize),
    GetOpeningRight(usize),
}

impl Subdivision {
    /// Divide into multiple rectangles.
    pub fn execute(self) -> Result<Vec<[Vec3; 4]>, SubdivisionError> {
        let normal = self.main_axis.cross(self.cross_axis).normalize();
        if !Vec3Helpers::is_ccw(&self.bounds, normal).expect("bounds should be valid") {
            return Err(SubdivisionError::BoundWinding);
        }
        let mut top_bound = get_edge_by_direction(&get_edges(self.bounds), self.main_axis.neg())
            .ok_or(SubdivisionError::GetBoundTop)?;
        let mut bottom_bound = get_edge_by_direction(&get_edges(self.bounds), self.main_axis)
            .ok_or(SubdivisionError::GetBoundTop)?;
        let mut rectangles = Vec::new();
        for (index, opening) in self.openings.into_iter().enumerate() {
            if Vec3Helpers::is_ccw(&opening, normal).expect("opening should be valid") {
                return Err(SubdivisionError::OpeningWinding(index));
            }
            let edges = get_edges(opening);
            let left = get_edge_by_direction(&edges, self.cross_axis)
                .ok_or(SubdivisionError::GetOpeningLeft(index))?;
            let right = get_edge_by_direction(&edges, self.cross_axis.neg())
                .ok_or(SubdivisionError::GetOpeningRight(index))?;
            // Create rectangle to the left of the opening
            let full = [
                bottom_bound[0],
                Vec3Helpers::project_point_to_line(left[0], bottom_bound),
                Vec3Helpers::project_point_to_line(left[0], top_bound),
                top_bound[1],
            ];
            top_bound[1] = full[2];
            bottom_bound[0] = full[1];
            push_if_not_zero(&mut rectangles, full);
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
            push_if_not_zero(&mut rectangles, above);
            push_if_not_zero(&mut rectangles, below);
        }
        // Create last rectangle
        let last = [bottom_bound[0], bottom_bound[1], top_bound[0], top_bound[1]];
        push_if_not_zero(&mut rectangles, last);
        Ok(rectangles)
    }

    pub fn example() -> Self {
        Self {
            bounds: [
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(12.0, 0.0, 0.0),
                Vec3::new(12.0, 12.0, 0.0),
                Vec3::new(0.0, 12.0, 0.0),
            ],
            openings: vec![
                [
                    Vec3::new(1.0, 3.0, 0.0),
                    Vec3::new(1.0, 9.0, 0.0),
                    Vec3::new(2.0, 9.0, 0.0),
                    Vec3::new(2.0, 3.0, 0.0),
                ],
                [
                    Vec3::new(3.0, 2.0, 0.0),
                    Vec3::new(3.0, 5.0, 0.0),
                    Vec3::new(4.0, 5.0, 0.0),
                    Vec3::new(4.0, 2.0, 0.0),
                ],
                [
                    Vec3::new(4.0, 7.0, 0.0),
                    Vec3::new(4.0, 10.0, 0.0),
                    Vec3::new(5.0, 10.0, 0.0),
                    Vec3::new(5.0, 7.0, 0.0),
                ],
                [
                    Vec3::new(6.0, 0.0, 0.0),
                    Vec3::new(6.0, 12.0, 0.0),
                    Vec3::new(8.0, 12.0, 0.0),
                    Vec3::new(8.0, 0.0, 0.0),
                ],
                [
                    Vec3::new(9.0, 6.0, 0.0),
                    Vec3::new(9.0, 12.0, 0.0),
                    Vec3::new(11.0, 12.0, 0.0),
                    Vec3::new(11.0, 6.0, 0.0),
                ],
            ],
            main_axis: Vec3::X,
            cross_axis: Vec3::Y,
        }
    }
}

fn push_if_not_zero(rectangles: &mut Vec<[Vec3; 4]>, rectangle: [Vec3; 4]) {
    let polygon = Polygon::from_open(rectangle.to_vec()).expect("polygon should be valid");
    if polygon.get_area() > 0.0 {
        rectangles.push(rectangle);
    }
}

#[allow(clippy::indexing_slicing)]
fn get_edges(rectangle: [Vec3; 4]) -> Vec<[Vec3; 2]> {
    let polygon = Polygon::from_open(rectangle.to_vec()).expect("polygon should be valid");
    polygon.get_edges()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute() {
        // Arrange
        let subdivision = Subdivision::example();

        // Act
        let result = subdivision.execute();

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.expect("should be some").len(), 12);
    }
}
