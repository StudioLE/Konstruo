use super::*;
use crate::beziers::CubicBezierSpline;
use crate::geometry::{Polyline, TriangleList};
use crate::GROUND_HEIGHT;
use bevy::prelude::*;

/// A surface formed by two lines from a [Way].
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct WaySurface {
    /// Depth of the surface.
    depth: f32,
    /// Offsets from the way.
    offsets: [f32; 2],
    /// Offsets from the way.
    purpose: SurfaceType,
}

pub enum SurfaceType {
    /// - <https://en.wikipedia.org/wiki/Carriageway>
    Carriageway,
    /// - <https://en.wikipedia.org/wiki/Footway>
    Footway,
    /// - <https://en.wikipedia.org/wiki/Road_verge>
    Verge,
}

impl WaySurface {
    /// Create a new [`WaySurface`] offset from [`Way`].
    #[must_use]
    pub fn new(depth: f32, offsets: [f32; 2], purpose: SurfaceType) -> Self {
        let offsets = if offsets[0] <= offsets[1] {
            offsets
        } else {
            [offsets[1], offsets[0]]
        };
        Self {
            depth,
            offsets,
            purpose,
        }
    }

    /// Create a new [`WaySurface`] centered at [`Way`].
    #[must_use]
    pub fn centered(depth: f32, width: f32, purpose: SurfaceType) -> Self {
        Self::new(depth, [width * -0.5, width * 0.5], purpose)
    }

    /// Regenerate the mesh geometry.
    pub fn regenerate(&self, meshes: &mut ResMut<Assets<Mesh>>, mut mesh: Mut<Mesh3d>, way: &Way) {
        *mesh = Mesh3d(meshes.add(self.get_mesh(way)));
    }

    /// Spawn a [`WaySurface`] with its mesh geometry.
    pub fn spawn(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &Res<WayMaterials>,
        way: &Way,
        parent: Entity,
    ) {
        let mesh = self.get_mesh(way);
        let material = materials.get_surface(&self.purpose);
        let bundle = (
            self,
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(material),
            Transform::from_translation(Vec3::new(0.0, 0.0, GROUND_HEIGHT)),
            PickingBehavior::default(),
        );
        commands
            .spawn(bundle)
            .observe(on_pointer_over)
            .observe(on_pointer_out)
            .set_parent(parent);
    }

    /// Get the splines of each edge.
    fn get_splines(&self, way: &Way) -> [CubicBezierSpline; 2] {
        [
            way.spline.offset(self.offsets[0], OFFSET_ACCURACY),
            way.spline.offset(self.offsets[1], OFFSET_ACCURACY),
        ]
    }

    /// Get the polylines of each edge.
    fn get_polylines(&self, way: &Way) -> [Polyline; 2] {
        let splines = self.get_splines(way);
        [
            splines[0].flatten(FLATTEN_TOLERANCE).into(),
            splines[1].flatten(FLATTEN_TOLERANCE).into(),
        ]
    }

    /// Get the [`Mesh`].
    fn get_mesh(&self, way: &Way) -> Mesh {
        let polylines = self.get_polylines(way);
        TriangleList::between_polylines_3d(polylines, self.depth).to_mesh()
    }
}

fn on_pointer_over(
    event: Trigger<Pointer<Over>>,
    materials: Res<WayMaterials>,
    mut query: Query<(&WaySurface, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    let Ok((_surface, mut material)) = query.get_mut(event.entity()) else {
        error!("Failed to get WaySurface");
        return;
    };
    *material = MeshMaterial3d(materials.surface_over.clone());
}

fn on_pointer_out(
    event: Trigger<Pointer<Out>>,
    materials: Res<WayMaterials>,
    mut query: Query<(&WaySurface, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    let Ok((surface, mut material)) = query.get_mut(event.entity()) else {
        error!("Failed to get WaySurface");
        return;
    };
    *material = MeshMaterial3d(materials.get_surface(&surface.purpose));
}
