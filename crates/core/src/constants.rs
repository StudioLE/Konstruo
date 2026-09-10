use bevy::render::view::Msaa;

/// Minimum camera orbit radius.
pub const CAMERA_MIN: f32 = 10.0;

/// Extent of the camera pan and orbit radius.
pub const CAMERA_MAX: f32 = 20_000.0;

/// Extent of the grid.
pub const GRID_MAX: u32 = 40_000;

/// Multisample anti-aliasing of every camera.
///
/// - `Off`, which [`ScreenSpaceAmbientOcclusion`](bevy::pbr::ScreenSpaceAmbientOcclusion)
///   requires, `Smaa` standing in as the anti-aliasing
/// - MUST match across cameras drawing to one target; differing samples give each its own
///   intermediate texture, so they stop compositing with one another
#[cfg(not(target_arch = "wasm32"))]
pub const CAMERA_MSAA: Msaa = Msaa::Off;

/// Multisample anti-aliasing of every camera.
///
/// - `Sample4`, no web backend supporting screen space ambient occlusion
#[cfg(target_arch = "wasm32")]
pub const CAMERA_MSAA: Msaa = Msaa::Sample4;

/// Extent of the ground and sky.
///
/// - The camera `far` of `1000.0` does not clip this; it only culls objects wholly beyond it
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
