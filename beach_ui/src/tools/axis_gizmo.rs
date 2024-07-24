use bevy::color::palettes::tailwind;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct AxisGizmo {
    pub thickness: f32,
    pub length: f32,
    pub color_x: Srgba,
    pub color_y: Srgba,
    pub color_z: Srgba,
}

impl Default for AxisGizmo {
    fn default() -> Self {
        Self {
            thickness: 0.1,
            length: 1.0,
            color_x: tailwind::RED_700,
            color_y: tailwind::GREEN_700,
            color_z: tailwind::SKY_700,
        }
    }
}

impl AxisGizmo {
    /// Spawn an [`AxisGizmo`] entity with child meshes for each axis.
    pub fn spawn(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let spatial = SpatialBundle::default();
        let x = PbrBundle {
            mesh: meshes.add(Cuboid::new(self.length, self.thickness, self.thickness)),
            material: materials.add(StandardMaterial::from_color(self.color_x)),
            ..default()
        };
        let y = PbrBundle {
            mesh: meshes.add(Cuboid::new(self.thickness, self.length, self.thickness)),
            material: materials.add(StandardMaterial::from_color(self.color_y)),
            ..default()
        };
        let z = PbrBundle {
            mesh: meshes.add(Cuboid::new(self.thickness, self.thickness, self.length)),
            material: materials.add(StandardMaterial::from_color(self.color_z)),
            ..default()
        };
        commands.spawn((spatial, self)).with_children(|parent| {
            parent.spawn(x);
            parent.spawn(y);
            parent.spawn(z);
        });
    }
}

pub fn spawn_origin_gizmo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let gizmo = AxisGizmo {
        length: 100.0,
        thickness: 0.050,
        ..default()
    };
    gizmo.spawn(&mut commands, &mut meshes, &mut materials);
}
