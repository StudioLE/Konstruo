use super::*;
use crate::beziers::CubicBezierSpline;
use crate::distribution::{Distributable, Distribution, FlexFactory};
use crate::geometry::Polyline;
use crate::ui::EntityState;
use bevy::prelude::*;

/// Tolerance with which the bezier is flattened into a polyline.
/// The greater the tolerance the more segments to the polyline.
/// By default this is 10 mm which is an acceptable architectural tolerance.
pub const FLATTEN_TOLERANCE: f32 = 0.010;

/// Accuracy of the bezier created by offset.
pub const OFFSET_ACCURACY: f32 = 1.0;

/// Accuracy used for length calculation.
pub const LENGTH_ACCURACY: f32 = 1e-3;

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
#[require(InheritedVisibility, Transform, EntityState)]
pub struct Way {
    /// Get the cubic bezier curves of the way.
    /// All vectors are
    pub spline: CubicBezierSpline,
}

impl Way {
    /// Create a [`Way`]
    #[must_use]
    pub fn new(spline: CubicBezierSpline) -> Self {
        Self { spline }
    }

    /// System to update all children when the spline has changed.
    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    pub fn on_spline_changed(
        mut meshes: ResMut<Assets<Mesh>>,
        controls: Query<(&WayControl, &Parent, &mut Transform)>,
        lines: Query<(&WayControlLine, &Parent, &mut Mesh3d), Without<Way>>,
        surfaces: Query<
            (&WaySurface, &Parent, &mut Mesh3d),
            (Without<Way>, Without<WayControlLine>),
        >,
        distributions: Query<(&mut Distribution, &Parent), Without<Distributable>>,
        way: &Way,
        way_entity: Entity,
        mut mesh: Mut<Mesh3d>,
    ) {
        let polyline = way.spline.flatten(FLATTEN_TOLERANCE);
        *mesh = Mesh3d(meshes.add(Polyline::new(polyline).to_mesh()));
        let control_points = way.spline.get_controls();
        WayControl::on_spline_changed(controls, way_entity, &control_points);
        WayControlLine::on_spline_changed(lines, &mut meshes, way_entity, control_points);
        WaySurface::on_spline_changed(surfaces, &mut meshes, way, way_entity);
        redistribute_on_spline_changed(distributions, way, way_entity);
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
                Mesh3d(meshes.add(Polyline::new(polyline).to_mesh())),
                MeshMaterial3d(materials.control_line.clone()),
            );
            commands.entity(entity).insert(bundle);
            WayControl::spawn(&mut commands, &way_meshes, &materials, way, entity);
            WayControlLine::spawn(&mut commands, &mut meshes, &materials, way, entity);
        }
    }

    /// System to
    pub fn state_changed_system(
        ways: Query<(Entity, &EntityState), (Changed<EntityState>, With<Way>)>,
        materials: Res<WayMaterials>,
        mut surfaces: Query<
            (&WaySurface, &Parent, &mut MeshMaterial3d<StandardMaterial>),
            (With<WaySurface>, Without<Way>),
        >,
    ) {
        for (entity, state) in ways.iter() {
            WaySurface::on_way_state_changed(&mut surfaces, &materials, entity, state);
        }
    }
}

fn redistribute_on_spline_changed(
    mut distributions: Query<(&mut Distribution, &Parent), Without<Distributable>>,
    way: &Way,
    way_entity: Entity,
) {
    for (mut distribution, parent) in &mut distributions {
        if parent.get() != way_entity {
            continue;
        }
        let length = way.spline.get_length(LENGTH_ACCURACY);
        let flex = FlexFactory {
            bounds: distribution.flex.bounds.map(|bounds| bounds.with_x(length)),
            ..distribution.flex
        };
        let spline = if let Some(offset) = distribution.spline_offset {
            way.spline.offset(offset, OFFSET_ACCURACY)
        } else {
            way.spline.clone()
        };
        *distribution = Distribution {
            flex,
            spline: Some(spline),
            ..distribution.clone()
        };
    }
}
