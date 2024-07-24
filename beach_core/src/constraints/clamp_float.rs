use crate::mathematics::floats::modulo;

pub enum ClampFloat {
    Fixed(f32, f32),
    Wrapped(f32),
}

impl ClampFloat {
    pub fn clamp(&self, number: f32) -> f32 {
        match self {
            ClampFloat::Fixed(min, max) => number.clamp(*min, *max),
            ClampFloat::Wrapped(max) => modulo(number, *max),
        }
    }
}
