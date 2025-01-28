use bevy::prelude::*;

pub trait Distributable {
    fn get_size(&self) -> Vec3;
}
