use crate::ways::control_line::WayLine;
use crate::ways::controls::{WayControlBundle};
use crate::ways::edges::WayEdges2d;
use beach_core::beziers::flatten::flatten_bezier;
use bevy::math::vec3;
use bevy::prelude::*;

#[derive(Component)]
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
}

pub fn spawn_way_example(mut commands: Commands) {
    let vertices = vec![
        [
            vec3(0.0, 0.0, 70.0),
            vec3(30.0, 0.0, 70.0),
            vec3(30.0, 0.0, 40.0),
            vec3(50.0, 0.0, 40.0),
        ],
        [
            vec3(50.0, 0.0, 40.0),
            vec3(70.0, 0.0, 40.0),
            vec3(70.0, 0.0, 15.0),
            vec3(70.0, 0.0, 0.0),
        ],
    ];
    let way = Way {
        curve: vertices,
        width: 5.0,
        depth: 1.0,
        ..Way::default()
    };
    commands.spawn((way, SpatialBundle::default()));
}

/// System to create [`WayLine`], [`WayEdges2d`], and [`WayControl`] when a [`Way`] is added.
pub fn on_way_added(mut commands: Commands, query: Query<(Entity, &Way), Added<Way>>) {
    for (entity, way) in query.iter() {
        commands
            .spawn((WayLine::from_way(way), SpatialBundle::default()))
            .set_parent(entity);
        commands
            .spawn((WayEdges2d::from_way(way), SpatialBundle::default()))
            .set_parent(entity);
        for p in way.curve.clone() {
            commands
                .spawn(WayControlBundle::new(p[0], p[1]))
                .set_parent(entity);
            commands
                .spawn(WayControlBundle::new(p[3], p[2]))
                .set_parent(entity);
        }
    }
}
