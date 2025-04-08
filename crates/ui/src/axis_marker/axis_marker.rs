use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use konstruo_core::Vec3Extensions;
use konstruo_geometry::{Axis, Cuboid};

const DEFAULT_THICKNESS: f32 = 0.1;
const DEFAULT_LENGTH: f32 = 1.0;

/// A mesh representation of the axes.
///
/// Each axis has represented with its standard color:
/// X: Red
/// Y: Green
/// Z: Blue
#[derive(Component, Debug)]
#[require(InheritedVisibility, Transform)]
pub struct AxisMarker;

/// Factory to spawn an [`AxisMarker`] and its geometry.
pub struct AxisMarkerFactory<'w> {
    pub commands: Commands<'w, 'w>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<StandardMaterial>>,
}

impl AxisMarkerFactory<'_> {
    /// Create an [`AxisMarker`] and its child geometry.
    #[allow(clippy::must_use_candidate)]
    pub fn spawn(
        mut self,
        origin: Option<Vec3>,
        thickness: Option<f32>,
        length: Option<f32>,
    ) -> Entity {
        let thickness = thickness.unwrap_or(DEFAULT_THICKNESS);
        let length = length.unwrap_or(DEFAULT_LENGTH);
        let origin = origin.unwrap_or(Vec3::ZERO);
        let bundle = (AxisMarker, Transform::from_translation(origin));
        let x = self.axis_geometry(thickness, length, Axis::X);
        let y = self.axis_geometry(thickness, length, Axis::Y);
        let z = self.axis_geometry(thickness, length, Axis::Z);
        self.commands
            .spawn(bundle)
            .with_children(|parent| {
                parent.spawn(x);
                parent.spawn(y);
                parent.spawn(z);
            })
            .id()
    }

    fn axis_geometry(&mut self, thickness: f32, length: f32, axis: Axis) -> impl Bundle {
        let mesh = Cuboid::new(get_transform(thickness, length, axis))
            .get_triangles()
            .to_mesh();
        let material = StandardMaterial {
            base_color: get_color(axis).into(),
            alpha_mode: AlphaMode::Opaque,
            perceptual_roughness: 1.0,
            unlit: true,
            ..default()
        };
        (
            MeshMaterial3d(self.materials.add(material)),
            Mesh3d(self.meshes.add(mesh)),
        )
    }
}

fn get_color(axis: Axis) -> Srgba {
    match axis {
        Axis::X => tailwind::RED_600,
        Axis::Y => tailwind::GREEN_600,
        Axis::Z => tailwind::SKY_600,
    }
}

fn get_transform(thickness: f32, length: f32, axis: Axis) -> Transform {
    let direction = axis.get_vector();
    let inverse = Vec3Extensions::invert_0_and_1(direction);
    let scale = direction * (length + thickness) + inverse * thickness;
    Transform::from_translation(direction * length * 0.5).with_scale(scale)
}
