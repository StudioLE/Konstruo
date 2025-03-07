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
        mut events: EventReader<SplineChangedEvent>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut ways: Query<&mut Mesh3d, With<Way>>,
        mut distributions: Query<(&mut Distribution, &Parent), Without<Distributable>>,
    ) {
        for event in events.read() {
            let Ok(mut mesh) = ways.get_mut(event.way) else {
                warn!("Failed to get Way");
                continue;
            };
            let polyline = event.spline.flatten(FLATTEN_TOLERANCE);
            *mesh = Mesh3d(meshes.add(Polyline::new(polyline).to_mesh()));
            redistribute_on_spline_changed(&mut distributions, event);
        }
    }

    /// Spawn a [`Way`] along with its [`Mesh3d`], [`WayControl`], and [`WayControlLine`].
    pub fn spawn(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        way_meshes: &Res<WayMeshes>,
        materials: &Res<WayMaterials>,
    ) -> Entity {
        let polyline = self.spline.flatten(FLATTEN_TOLERANCE);
        let bundle = (
            self.clone(),
            Mesh3d(meshes.add(Polyline::new(polyline).to_mesh())),
            MeshMaterial3d(materials.control_line.clone()),
        );
        let entity = commands.spawn(bundle).id();
        WayControl::spawn(commands, way_meshes, materials, &self, entity);
        WayControlLine::spawn(commands, meshes, materials, &self, entity);
        entity
    }
}

fn redistribute_on_spline_changed(
    distributions: &mut Query<(&mut Distribution, &Parent), Without<Distributable>>,
    event: &SplineChangedEvent,
) {
    for (mut distribution, parent) in distributions {
        if parent.get() != event.way {
            continue;
        }
        let length = event.spline.get_length(LENGTH_ACCURACY);
        let flex = FlexFactory {
            bounds: distribution.flex.bounds.map(|bounds| bounds.with_x(length)),
            ..distribution.flex
        };
        let spline = if let Some(offset) = distribution.spline_offset {
            event
                .spline
                .offset(offset, OFFSET_ACCURACY)
                .expect("spline offset should be valid")
        } else {
            event.spline.clone()
        };
        *distribution = Distribution {
            flex,
            spline: Some(spline),
            ..distribution.clone()
        };
    }
}
