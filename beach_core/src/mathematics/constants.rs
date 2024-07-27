use std::f32::consts::FRAC_1_SQRT_2;

pub const EPSILON: f32 = 0.000_001;

/// Represents the ratio of the circumference of a circle to its diameter, specified by the constant, π.
/// π radians = 180°
pub const PI: f32 = std::f32::consts::PI;

/// π/180 radians = 1°
pub const ONE_DEGREE_IN_RADIANS: f32 = PI / 180.0;

/// π/8 radians = 22.5°
pub const EIGHTH_PI: f32 = PI / 8.0;

/// π/4 radians = 45°
pub const QUARTER_PI: f32 = PI / 4.0;

/// π/2  radians = 90°
pub const HALF_PI: f32 = PI / 2.0;

/// 2 * π radians = 360°
pub const TWO_PI: f32 = 2.0 * PI;

/// 1/√2
pub const ONE_OVER_ROOT_2: f32 = FRAC_1_SQRT_2;
