use crate::ways::controls::WayControl;
use crate::ways::line::WayLine;
use crate::ways::WayEdges2d;
use beach_core::beziers::flatten::flatten_bezier;
use bevy::prelude::*;

/// Tolerance with which the bezier is flattened into a polyline.
/// The greater the tolerance the more segments to the polyline.
/// By default this is 10 mm which is an acceptable architectural tolerance.
pub const FLATTEN_TOLERANCE: f32 = 0.010;

/// Accuracy of the bezier created by offset.
pub const OFFSET_ACCURACY: f32 = 1.0;

/// A road, route or path defined by one or more cubic bezier curves.
///
/// The way defines the center of the road, route or path.
///
/// In typical use a single way defines the path of multiple constructs.
/// For example a road may have two vehicular lanes and a pavement on each side.
/// Changing the way would change each of these entities, and even affect the buildings
/// distributed alongside.
///
/// The way does not have a transform. Its geometry is defined by the control points of its cubic bezier curves.
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct Way {
    /// Get the cubic bezier curves of the way.
    /// All vectors are
    curves: Vec<[Vec3; 4]>,
}

impl Way {
    /// Create a [`Way`]
    pub fn new(curves: Vec<[Vec3; 4]>) -> Self {
        Self { curves }
    }

    /// Get the cubic bezier curve of the way.
    pub fn get_curve(&self) -> CubicBezier<Vec3> {
        CubicBezier::new(self.curves.clone())
    }

    /// Get the polyline of the way by flattening the bezier curve.
    pub fn get_polyline(&self) -> Vec<Vec3> {
        flatten_bezier(&self.get_curve(), FLATTEN_TOLERANCE)
    }

    /// System to create [`WayLine`], [`WayEdges2d`], and [`WayControl`] when a [`Way`] is added.
    pub fn added_system(mut commands: Commands, query: Query<(Entity, &Way), Added<Way>>) {
        for (entity, way) in query.iter() {
            commands.spawn(WayLine::from_way(way)).set_parent(entity);
            commands
                .spawn(WayEdges2d::from_way(way, 5.0))
                .set_parent(entity);
            for curve in way.curves.clone() {
                commands
                    .spawn(WayControl::new(curve[0], curve[1]))
                    .set_parent(entity);
                commands
                    .spawn(WayControl::new(curve[3], curve[2]))
                    .set_parent(entity);
            }
        }
    }
}
