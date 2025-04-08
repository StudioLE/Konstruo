use bevy::prelude::*;
use konstruo_architecture::*;
use konstruo_beziers::constants::{LENGTH_ACCURACY, OFFSET_ACCURACY};
use konstruo_distribution::*;
use konstruo_paths::Path;

const SPLINE_OFFSET: f32 = 10.0;

pub struct FlexBuildingsAlongPathExample;

#[derive(Resource)]
struct State {
    enabled: bool,
}

impl Plugin for FlexBuildingsAlongPathExample {
    fn build(&self, app: &mut App) {
        app.insert_resource(State::default())
            .add_systems(Update, path_added_system);
    }
}

impl Default for State {
    fn default() -> Self {
        Self { enabled: true }
    }
}

fn spawn_example(factory: &mut ModularBuildingFactory<'_>, entity: Entity, path: &Path) -> Entity {
    let distribution = distribution_bundle(path, entity);
    let distribution_entity = factory.commands.spawn(distribution).id();
    for (distributable, building) in get_items() {
        let plot = factory.spawn(building);
        factory
            .commands
            .entity(plot)
            .insert(distributable)
            .insert(ChildOf {
                parent: distribution_entity,
            });
    }
    distribution_entity
}

fn distribution_bundle(path: &Path, parent: Entity) -> impl Bundle {
    let spline = path
        .spline
        .offset(SPLINE_OFFSET, OFFSET_ACCURACY)
        .expect("spline offset should be valid");
    let spline_length = spline.get_length(LENGTH_ACCURACY);
    (
        Distribution {
            flex: FlexBuilder::new()
                .with_bounds(Vec3::new(spline_length, 0.0, 0.0))
                .with_justify_content(JustifyContent::SpaceAround)
                .with_align_items_cross(AlignItems::FlexStart)
                .build(),
            spline: Some(spline),
            spline_offset: Some(SPLINE_OFFSET),
            translate_to_ground: true,
        },
        ChildOf { parent },
    )
}

fn path_added_system(
    commands: Commands,
    mut state: ResMut<State>,
    meshes: ResMut<Assets<Mesh>>,
    building_meshes: Res<BuildingMeshes>,
    materials: Res<BuildingMaterials>,
    query: Query<(Entity, &Path), Added<Path>>,
) {
    if !state.enabled {
        return;
    }
    let mut factory = ModularBuildingFactory {
        commands,
        meshes,
        building_meshes,
        materials,
    };
    for (entity, path) in query.iter() {
        spawn_example(&mut factory, entity, path);
        state.enabled = false;
    }
}

fn get_items() -> Vec<(Distributable, ModularBuildingInfo)> {
    vec![
        (
            Distributable {
                order: 0,
                ..default()
            },
            BuildingTemplates::_2b4p1s0g(),
        ),
        (
            Distributable {
                order: 1,
                ..default()
            },
            BuildingTemplates::_2b4p1s0g_perp(),
        ),
        (
            Distributable {
                order: 2,
                ..default()
            },
            BuildingTemplates::_3b6p2s2g(),
        ),
        (
            Distributable {
                order: 3,
                ..default()
            },
            BuildingTemplates::_3b6p2s2g_perp(),
        ),
        (
            Distributable {
                order: 4,
                ..default()
            },
            BuildingTemplates::_4b8p2s2g(),
        ),
        (
            Distributable {
                order: 5,
                ..default()
            },
            BuildingTemplates::_4b8p2s2g_perp(),
        ),
    ]
}
