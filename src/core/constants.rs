/// Extent of the camera pan and orbit radius.
pub const CAMERA_MIN: f32 = 10.0;

/// Extent of the camera pan and orbit radius.
pub const CAMERA_MAX: f32 = 2_500.0;

/// Extent of the grid.
pub const GRID_MAX: u32 = 10_000;

/// Extent of the ground and sky.
pub const ENVIRONMENT_MAX: f32 = 10_000.0;

/// Elevation of the ground plane
///
/// A lower elevation is used for WebAssembly to prevent z-fighting
#[cfg(not(target_arch = "wasm32"))]
pub const GROUND_ELEVATION: f32 = -0.075;
#[cfg(target_arch = "wasm32")]
pub const GROUND_ELEVATION: f32 = -0.200;

/// Elevation of the grid plane
///
/// Grid will z-fight with ground plane beneath and path surface above.
///
/// This value works for reasonable zoom levels but fighting occurs when zoomed out
/// at which point it's not noticable.
#[cfg(not(target_arch = "wasm32"))]
pub const GRID_ELEVATION: f32 = -0.060;
#[cfg(target_arch = "wasm32")]
pub const GRID_ELEVATION: f32 = -0.100;

/// Elevation of paths
///
/// Paths are extruded upwards so the top elevation is higher.
pub const PATH_ELEVATION: f32 = -0.050;
