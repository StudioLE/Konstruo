use crate::ui::gizmos::weights::{Light, Medium, Thin};
use bevy::color::palettes::basic;
use bevy::math::{Quat, UVec2, Vec2};
use bevy::prelude::*;
use std::f32::consts::PI;

pub struct GridGizmo;

impl GridGizmo {
    pub fn draw(mut thin: Gizmos<Thin>, mut light: Gizmos<Light>, mut medium: Gizmos<Medium>) {
        let color = basic::WHITE.with_alpha(0.1);
        thin.grid(
            Quat::from_rotation_x(PI / 2.0),
            UVec2::splat(1000),
            Vec2::splat(1.0),
            color,
        );
        light.grid(
            Quat::from_rotation_x(PI / 2.0),
            UVec2::splat(100),
            Vec2::splat(10.0),
            color,
        );
        medium.grid(
            Quat::from_rotation_x(PI / 2.0),
            UVec2::splat(10),
            Vec2::splat(100.0),
            color,
        );
    }
}
