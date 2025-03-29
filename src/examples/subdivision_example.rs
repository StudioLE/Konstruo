use crate::examples::ExampleMaterials;
use crate::geometry::*;
use bevy::prelude::*;

pub struct SubdivisionExample;

impl Plugin for SubdivisionExample {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, Self::startup_system);
    }
}

impl SubdivisionExample {
    fn startup_system(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let subdivision = Subdivision {
            bounds: [
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(12.0, 0.0, 0.0),
                Vec3::new(12.0, 12.0, 0.0),
                Vec3::new(0.0, 12.0, 0.0),
            ],
            openings: vec![
                [
                    Vec3::new(1.0, 3.0, 0.0),
                    Vec3::new(1.0, 9.0, 0.0),
                    Vec3::new(2.0, 9.0, 0.0),
                    Vec3::new(2.0, 3.0, 0.0),
                ],
                [
                    Vec3::new(3.0, 2.0, 0.0),
                    Vec3::new(3.0, 5.0, 0.0),
                    Vec3::new(4.0, 5.0, 0.0),
                    Vec3::new(4.0, 2.0, 0.0),
                ],
                [
                    Vec3::new(4.0, 7.0, 0.0),
                    Vec3::new(4.0, 10.0, 0.0),
                    Vec3::new(5.0, 10.0, 0.0),
                    Vec3::new(5.0, 7.0, 0.0),
                ],
                [
                    Vec3::new(6.0, 0.0, 0.0),
                    Vec3::new(6.0, 12.0, 0.0),
                    Vec3::new(8.0, 12.0, 0.0),
                    Vec3::new(8.0, 0.0, 0.0),
                ],
                [
                    Vec3::new(9.0, 6.0, 0.0),
                    Vec3::new(9.0, 12.0, 0.0),
                    Vec3::new(11.0, 12.0, 0.0),
                    Vec3::new(11.0, 6.0, 0.0),
                ],
            ],
            main_axis: Vec3::X,
            cross_axis: Vec3::Y,
        };
        let regions = subdivision.execute().expect("should be valid");
        for region in regions {
            spawn_rectangle(&mut commands, &mut meshes, &mut materials, region);
        }
    }
}

fn spawn_rectangle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    rectangle: [Vec3; 4],
) {
    let triangles = Triangle::from_rectangle(rectangle);
    let is_ccw = Vec3Helpers::is_ccw(&rectangle, Vec3::Z).expect("should be valid");
    let material = if is_ccw {
        ExampleMaterials::blue_face_transparent()
    } else {
        ExampleMaterials::red_face()
    };
    let bundle = (
        Solid,
        MeshMaterial3d(materials.add(material)),
        Mesh3d(meshes.add(TriangleList::new(triangles.to_vec()).to_mesh())),
    );
    commands.spawn(bundle);
    for triangle in triangles {
        let material = if is_ccw {
            ExampleMaterials::blue_edge()
        } else {
            ExampleMaterials::red_edge()
        };
        let bundle = (
            Edge,
            MeshMaterial3d(materials.add(material)),
            Mesh3d(meshes.add(triangle.get_edges().to_mesh())),
        );
        commands.spawn(bundle);
    }
}
