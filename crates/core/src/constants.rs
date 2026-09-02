/// Minimum camera orbit radius.
pub const CAMERA_MIN: f32 = 10.0;

/// Extent of the camera pan and orbit radius.
pub const CAMERA_MAX: f32 = 20_000.0;

/// Extent of the directional light shadow cascades.
///
/// - Stays well below [`CAMERA_MAX`], so the cascades keep a usable texel density
pub const SHADOW_MAX: f32 = 2_500.0;

/// Extent of the grid.
pub const GRID_MAX: u32 = 40_000;

/// Extent of the ground and sky.
pub const ENVIRONMENT_MAX: f32 = 40_000.0;

/// Elevation of the ground plane.
#[cfg(not(target_arch = "wasm32"))]
pub const GROUND_ELEVATION: f32 = -0.075;

/// Elevation of the ground plane.
///
/// - Sits lower than on native to prevent z-fighting
#[cfg(target_arch = "wasm32")]
pub const GROUND_ELEVATION: f32 = -0.200;

/// Elevation of the grid plane.
///
/// - Z-fights with the ground plane beneath and the path surface above
/// - Holds up at reasonable zoom levels
/// - Fights when zoomed out, where it is not noticeable
#[cfg(not(target_arch = "wasm32"))]
pub const GRID_ELEVATION: f32 = -0.060;

/// Elevation of the grid plane.
///
/// - Sits lower than on native to prevent z-fighting
#[cfg(target_arch = "wasm32")]
pub const GRID_ELEVATION: f32 = -0.100;

/// Elevation of paths.
///
/// - Extrudes upwards, so the top elevation is higher
pub const PATH_ELEVATION: f32 = -0.050;

/// Elevation of surface water.
///
/// - Sits above [`PATH_ELEVATION`], so water reads as the topmost ground layer
pub const WATER_ELEVATION: f32 = -0.040;
