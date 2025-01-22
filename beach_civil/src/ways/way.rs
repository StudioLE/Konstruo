use crate::ways::controls::WayControl;
use crate::ways::edges::WayEdges2d;
use crate::ways::line::WayLine;
use beach_core::beziers::flatten::flatten_bezier;
use bevy::prelude::*;

#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct Way {
    pub curve: Vec<[Vec3; 4]>,
    pub width: f32,
    pub depth: f32,
    pub flatten_tolerance: f32,
    pub offset_accuracy: f32,
}

impl Default for Way {
    fn default() -> Self {
        Self {
            curve: Vec::new(),
            width: 1.0,
            depth: 1.0,
            flatten_tolerance: 0.010,
            offset_accuracy: 1.0,
        }
    }
}

impl Way {
    /// Get the cubic bezier curve of the way.
    pub fn get_curve(&self) -> CubicBezier<Vec3> {
        CubicBezier::new(self.curve.clone())
    }

    /// Get the polyline of the way by flattening the bezier curve.
    pub fn get_polyline(&self) -> Vec<Vec3> {
        flatten_bezier(&self.get_curve(), self.flatten_tolerance)
    }

    /// System to create [`WayLine`], [`WayEdges2d`], and [`WayControl`] when a [`Way`] is added.
    pub fn added_system(mut commands: Commands, query: Query<(Entity, &Way), Added<Way>>) {
        for (entity, way) in query.iter() {
            commands.spawn(WayLine::from_way(way)).set_parent(entity);
            commands.spawn(WayEdges2d::from_way(way)).set_parent(entity);
            for p in way.curve.clone() {
                commands
                    .spawn(WayControl::new(p[0], p[1]))
                    .set_parent(entity);
                commands
                    .spawn(WayControl::new(p[3], p[2]))
                    .set_parent(entity);
            }
        }
    }
}
