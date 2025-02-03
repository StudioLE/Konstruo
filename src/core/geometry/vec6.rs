use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec6 {
    /// Right
    pub x_pos: f32,
    /// Left
    pub x_neg: f32,
    /// Back
    pub y_pos: f32,
    /// Front
    pub y_neg: f32,
    /// Top
    pub z_pos: f32,
    /// Bottom
    pub z_neg: f32,
}

impl Vec6 {
    pub const ZERO: Self = Self {
        x_pos: 0.0,
        x_neg: 0.0,
        y_pos: 0.0,
        y_neg: 0.0,
        z_pos: 0.0,
        z_neg: 0.0,
    };

    #[must_use]
    pub fn new(x_pos: f32, x_neg: f32, y_pos: f32, y_neg: f32, z_pos: f32, z_neg: f32) -> Self {
        Self {
            x_pos,
            x_neg,
            y_pos,
            y_neg,
            z_pos,
            z_neg,
        }
    }

    #[must_use]
    pub fn splat(value: f32) -> Self {
        Self {
            x_pos: value,
            x_neg: value,
            y_pos: value,
            y_neg: value,
            z_pos: value,
            z_neg: value,
        }
    }

    #[must_use]
    pub fn get_pos(&self) -> Vec3 {
        Vec3::new(self.x_pos, self.y_pos, self.z_pos)
    }

    #[must_use]
    pub fn get_neg(&self) -> Vec3 {
        Vec3::new(self.x_neg, self.y_neg, self.z_neg)
    }
}
