use crate::ways::{WayControl, WayMaterials, WayMeshes, WaySurface};
use beach_core::beziers::CubicBezierSpline;
use beach_core::geometry::meshes::create_linestrip;
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
    pub(super) spline: CubicBezierSpline,
}

impl Way {
    /// Create a [`Way`]
    pub fn new(spline: CubicBezierSpline) -> Self {
        Self { spline }
    }

    /// System to create [`Mesh3d`], [`WaySurface`], and [`WayControl`] when a [`Way`] is added.
    pub fn added_system(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        way_meshes: Res<WayMeshes>,
        materials: Res<WayMaterials>,
        query: Query<(Entity, &Way), Added<Way>>,
    ) {
        for (entity, way) in query.iter() {
            let polyline = way.spline.flatten(FLATTEN_TOLERANCE);
            let bundle = (
                Mesh3d(meshes.add(create_linestrip(polyline))),
                MeshMaterial3d(materials.control_line.clone()),
            );
            commands.spawn(bundle).set_parent(entity);
            commands
                .spawn(WaySurface::from_center(way, 5.0))
                .set_parent(entity);
            WayControl::spawn(&mut commands, &way_meshes, &materials, way, entity);
        }
    }
}
