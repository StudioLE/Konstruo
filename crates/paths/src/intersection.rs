use bevy::prelude::*;
use konstruo_beziers::CubicBezierSpline;

/// An intersection between [`Path`].
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct PathIntersection {
    entities: Vec<(Entity, CubicBezierSpline)>,
}

impl PathIntersection {
    pub(super) fn new(entities: Vec<(Entity, CubicBezierSpline)>) -> Self {
        Self { entities }
    }

    pub fn get_splines(&self) -> Vec<&CubicBezierSpline> {
        self.entities.iter().map(|(_, spline)| spline).collect()
    }

    pub fn get_corners(&self) -> Vec<CubicBezierSpline> {
        let mut source = self.get_splines();
        let mut corners = Vec::with_capacity(source.len());
        source.push(source.first().expect("should be at least two splines"));
        for pair in source.windows(2) {
            let mut current = pair[0].clone();
            let next = pair[1].clone();
            current.reverse();
            let mut curves = current.to_curves();
            curves.append(&mut next.to_curves());
            let curve = CubicBezierSpline::new(curves).expect("should be valid");
            corners.push(curve);
        }
        corners
    }
}
