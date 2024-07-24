use crate::constraints::clamp_float::ClampFloat;
use bevy::prelude::*;

pub struct ClampVec3 {
    pub x: ClampFloat,
    pub y: ClampFloat,
    pub z: ClampFloat,
}

impl ClampVec3 {
    pub fn clamp(&self, position: Vec3) -> Vec3 {
        Vec3::new(
            self.x.clamp(position.x),
            self.y.clamp(position.y),
            self.z.clamp(position.z),
        )
    }
}
