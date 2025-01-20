use crate::cameras::primary_camera::PrimaryCamera;
use crate::tools::cursor::get_cursor_position;
use bevy::color::palettes::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::f32::consts::PI;

const ORIGIN_LENGTH: f32 = 100.0;

const CURSOR_LENGTH: f32 = 10.0;

const LINE_WIDTH_100: f32 = 0.3;

const LINE_WIDTH_300: f32 = 1.0;

const LINE_WIDTH_500: f32 = 2.0;

const LINE_WIDTH_700: f32 = 3.0;

const LINE_WIDTH_900: f32 = 4.0;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct Gizmos100;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct Gizmos300;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct Gizmos500;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct Gizmos700;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct Gizmos900;

pub fn configure_gizmos(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<Gizmos100>();
    config.line_width = LINE_WIDTH_100;
    config.depth_bias = -0.1;
    let (config, _) = config_store.config_mut::<Gizmos300>();
    config.line_width = LINE_WIDTH_300;
    config.depth_bias = -0.3;
    let (config, _) = config_store.config_mut::<Gizmos500>();
    config.line_width = LINE_WIDTH_500;
    config.depth_bias = -0.5;
    let (config, _) = config_store.config_mut::<Gizmos700>();
    config.line_width = LINE_WIDTH_700;
    config.depth_bias = -0.7;
    let (config, _) = config_store.config_mut::<Gizmos900>();
    config.line_width = LINE_WIDTH_900;
    config.depth_bias = -0.9;
}

pub fn draw_origin_gizmo(gizmos: Gizmos<Gizmos700>) {
    draw_axis_gizmo(gizmos, Vec3::ZERO, ORIGIN_LENGTH);
}

pub fn draw_cursor_gizmo(
    gizmos: Gizmos<Gizmos700>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
) {
    match get_cursor_position(&window, &camera) {
        Ok(position) => draw_axis_gizmo(gizmos, position, CURSOR_LENGTH),
        Err(e) => {
            warn!("{e:?}");
        }
    };
}

pub fn draw_grid(
    mut thin: Gizmos<Gizmos100>,
    mut medium: Gizmos<Gizmos300>,
    mut thick: Gizmos<Gizmos500>,
) {
    let color = basic::WHITE.with_alpha(0.1);
    // TODO: Visibility by zoom level
    thin.grid(
        Quat::from_rotation_x(PI / 2.0),
        UVec2::splat(1000),
        Vec2::splat(1.0),
        color,
    );
    medium.grid(
        Quat::from_rotation_x(PI / 2.0),
        UVec2::splat(100),
        Vec2::splat(10.0),
        color,
    );
    thick.grid(
        Quat::from_rotation_x(PI / 2.0),
        UVec2::splat(10),
        Vec2::splat(100.0),
        color,
    );
}

fn draw_axis_gizmo(mut gizmos: Gizmos<Gizmos700>, origin: Vec3, length: f32) {
    gizmos.ray(
        origin - Vec3::X * length * 0.5,
        Vec3::X * length,
        tailwind::RED_700,
    );
    gizmos.ray(
        origin - Vec3::Y * length * 0.5,
        Vec3::Y * length,
        tailwind::GREEN_700,
    );
    gizmos.ray(
        origin - Vec3::Z * length * 0.5,
        Vec3::Z * length,
        tailwind::SKY_700,
    );
}
