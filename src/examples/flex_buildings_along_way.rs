use crate::architecture::{
    BuildingMaterials, BuildingMeshes, BuildingTemplates, ModularBuildingFactory,
};
use crate::distribution::*;
use crate::infrastructure::{Way, OFFSET_ACCURACY};
use bevy::prelude::*;

const ACCURACY: f32 = 1e-3;
const SPLINE_OFFSET: f32 = 10.0;

pub struct FlexBuildingsAlongWayExample;

impl Plugin for FlexBuildingsAlongWayExample {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, way_added_system);
    }
}

fn way_added_system(
    mut commands: Commands,
    query: Query<(Entity, &Way), Added<Way>>,
    meshes: Res<BuildingMeshes>,
    materials: Res<BuildingMaterials>,
) {
    for (entity, way) in query.iter() {
        let spline = way.spline.offset(SPLINE_OFFSET, OFFSET_ACCURACY);
        let spline_length = spline.get_length(ACCURACY);
        let bundle = (Distribution {
            flex: FlexBuilder::new()
                .with_bounds(Vec3::new(spline_length, 0.0, 0.0))
                .with_justify_content(JustifyContent::SpaceAround)
                .with_align_items_cross(AlignItems::FlexStart)
                .build(),
            spline: Some(spline),
            spline_offset: Some(SPLINE_OFFSET),
            translate_to_ground: true,
            ..default()
        },);
        let parent = commands.spawn(bundle).set_parent(entity).id();
        for (distributable, factory) in get_items() {
            let bundle = (distributable, factory.clone());
            let entity = commands.spawn(bundle).set_parent(parent).id();
            factory.spawn_children(&mut commands, &meshes, &materials, entity);
        }
    }
}

fn get_items() -> Vec<(Distributable, ModularBuildingFactory)> {
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
