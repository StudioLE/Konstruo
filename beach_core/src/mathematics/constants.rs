/// Represents the ratio of the circumference of a circle to its diameter, specified by the constant, π.
/// π radians = 180°
pub const PI: f32 = std::f32::consts::PI;

/// π / 180  radians = 1°
pub const ONE_DEGREE_IN_RADIANS: f32 = 1.0 / 180.0 * PI;

/// 0.125 * π  radians = 22.5°
pub const EIGHTH_PI: f32 = 0.25 * PI;

/// 0.25 * π  radians = 45°
pub const QUARTER_PI: f32 = 0.25 * PI;

/// 0.5 * π  radians = 90°
pub const HALF_PI: f32 = 0.5 * PI;

/// 2 * π radians = 360°
pub const TWO_PI: f32 = 2.0 * PI;
