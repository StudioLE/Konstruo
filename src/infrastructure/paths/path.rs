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
/// The path defines the center of the road, route or path.
///
/// In typical use a single path defines the path of multiple constructs.
/// For example a road may have two vehicular lanes and a pavement on each side.
/// Changing the path would change each of these entities, and even affect the buildings
/// distributed alongside.
///
/// The path does not have a transform. Its geometry is defined by the control points of its cubic bezier curves.
#[derive(Clone, Component)]
#[require(InheritedVisibility, Transform, EntityState)]
pub struct Path {
    /// Get the cubic bezier curves of the path.
    /// All vectors are
    pub spline: CubicBezierSpline,
}

impl Path {
    /// Create a [`Path`]
    #[must_use]
    pub fn new(spline: CubicBezierSpline) -> Self {
        Self { spline }
    }

    /// Update [`Mesh3d`] and [`Distribution`] when the spline changes.
    pub(super) fn on_spline_changed(
        mut events: EventReader<SplineChanged>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut paths: Query<&mut Mesh3d, With<Path>>,
        mut distributions: Query<(&mut Distribution, &Parent), Without<Distributable>>,
    ) {
        for event in events.read() {
            let Ok(mut mesh) = paths.get_mut(event.path) else {
                warn!("Failed to get Path");
                continue;
            };
            let polyline = event.spline.flatten(FLATTEN_TOLERANCE);
            *mesh = Mesh3d(meshes.add(Polyline::new(polyline).to_mesh()));
            redistribute_on_spline_changed(&mut distributions, event);
        }
    }

    /// Spawn a [`Path`] along with its [`Mesh3d`], [`PathControl`], and [`PathControlLine`].
    pub fn spawn(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        path_meshes: &Res<PathMeshes>,
        materials: &Res<PathMaterials>,
    ) -> Entity {
        let polyline = self.spline.flatten(FLATTEN_TOLERANCE);
        let bundle = (
            self.clone(),
            Mesh3d(meshes.add(Polyline::new(polyline).to_mesh())),
            MeshMaterial3d(materials.center_line.clone()),
        );
        let entity = commands.spawn(bundle).id();
        PathControl::spawn(
            commands,
            path_meshes,
            materials,
            &self.spline,
            entity,
            Visibility::Hidden,
        );
        PathControlLine::spawn(
            commands,
            meshes,
            materials,
            &self.spline,
            entity,
            Visibility::Hidden,
        );
        entity
    }
}

fn redistribute_on_spline_changed(
    distributions: &mut Query<(&mut Distribution, &Parent), Without<Distributable>>,
    event: &SplineChanged,
) {
    for (mut distribution, parent) in distributions {
        if parent.get() != event.path {
            continue;
        }
        let spline = if let Some(offset) = distribution.spline_offset {
            event
                .spline
                .offset(offset, OFFSET_ACCURACY)
                .expect("spline offset should be valid")
        } else {
            event.spline.clone()
        };
        let length = spline.get_length(LENGTH_ACCURACY);
        let flex = FlexFactory {
            bounds: distribution.flex.bounds.map(|bounds| bounds.with_x(length)),
            ..distribution.flex
        };
        *distribution = Distribution {
            flex,
            spline: Some(spline),
            ..distribution.clone()
        };
    }
}
