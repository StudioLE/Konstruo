//! Easting and northing of the world origin.

/// Easting and northing of world `(0, 0)` in EPSG:27700.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Origin {
    /// Easting of world `(0, 0)`, in meters.
    pub easting: f64,
    /// Northing of world `(0, 0)`, in meters.
    pub northing: f64,
}

impl Origin {
    /// Origin every chunk written by the preprocessor is relative to.
    ///
    /// - Sits at the center of Kielder Water
    /// - Lands on a 10 km grid intersection, so Ordnance Survey tile corners
    ///   fall on round world coordinates
    /// - Duplicates `NationalGrid::ORIGIN` in `crates/gis/src/national_grid.rs`
    ///   until a follow-up makes `konstruo_gis` depend on this crate
    pub const DEFAULT: Self = Self {
        easting: 370_000.0,
        northing: 590_000.0,
    };
}
