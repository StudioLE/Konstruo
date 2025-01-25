use crate::ways::{WayControl, WayControlLine, WayMaterials, WayMeshes, WaySurface};
use bevy::prelude::*;
use geometrician_core::beziers::CubicBezierSpline;
use geometrician_core::geometry::meshes::create_linestrip;
use geometrician_core::mathematics::constants::QUARTER_PI;

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
#[derive(Clone, Component)]
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

    /// System to update everything when a control point is moved.
    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    pub fn regenerate(
        mut meshes: ResMut<Assets<Mesh>>,
        mut controls: Query<(&WayControl, &Parent, &mut Transform)>,
        mut lines: Query<(&WayControlLine, &Parent, &mut Mesh3d), Without<Way>>,
        mut surfaces: Query<
            (&WaySurface, &Parent, &mut Mesh3d),
            (Without<Way>, Without<WayControlLine>),
        >,
        way: &Way,
        way_entity: Entity,
        mut mesh: Mut<Mesh3d>,
    ) {
        // Mesh3d
        let polyline = way.spline.flatten(FLATTEN_TOLERANCE);
        *mesh = Mesh3d(meshes.add(create_linestrip(polyline)));
        // WayControl
        let control_points = way.spline.get_controls();
        for (control, parent, mut transform) in controls.iter_mut() {
            if parent.get() != way_entity {
                continue;
            }
            if let Some(translation) = control_points.get(control.index) {
                let index = control.index % 4;
                if index == 0 || index == 3 {
                    *transform = Transform::from_translation(*translation)
                        .with_rotation(Quat::from_rotation_z(QUARTER_PI));
                } else {
                    *transform = Transform::from_translation(*translation)
                }
            } else {
                warn!(
                    "Failed to set WayControl transform. Index does not exist: {}",
                    control.index
                );
            };
        }
        // WayControlLine
        for (line, parent, mut mesh) in lines.iter_mut() {
            if parent.get() != way_entity {
                continue;
            }
            if let Some(anchor) = control_points.get(line.anchor) {
                if let Some(handle) = control_points.get(line.handle) {
                    *mesh = Mesh3d(meshes.add(create_linestrip(vec![*anchor, *handle])));
                } else {
                    warn!(
                        "Failed to set WayControlLine. Index does not exist: {}",
                        line.handle
                    );
                };
            } else {
                warn!(
                    "Failed to set WayControlLine. Index does not exist: {}",
                    line.anchor
                );
            };
        }
        // WayControlLine
        for (surface, parent, mesh) in surfaces.iter_mut() {
            if parent.get() != way_entity {
                continue;
            }
            surface.regenerate(&mut meshes, mesh, way);
        }
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
            commands.entity(entity).insert(bundle);
            WayControl::spawn(&mut commands, &way_meshes, &materials, way, entity);
            WayControlLine::spawn(&mut commands, &mut meshes, &materials, way, entity);
        }
    }
}
