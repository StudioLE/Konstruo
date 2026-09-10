//! One source layer of polygons and the file it exports to.

/// One source layer of polygons and the file it exports to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PolygonLayer {
    /// Path of the `GeoPackage` file, relative to the source directory.
    pub source: &'static str,
    /// Name of the layer table within the file.
    pub layer: &'static str,
    /// Stem of the file the layer is written to.
    pub output: &'static str,
}

impl PolygonLayer {
    /// Every layer the preprocessor exports.
    // TODO: Move to a manifest describing the source dataset
    pub const ALL: [PolygonLayer; 6] = [
        PolygonLayer {
            source: "ordnance-survey/vmdvec_gpkg_gb/Data/vmdvec_gb.gpkg",
            layer: "surface_water_area",
            output: "surface-water",
        },
        PolygonLayer {
            source: "ordnance-survey/vmdvec_gpkg_gb/Data/vmdvec_gb.gpkg",
            layer: "woodland",
            output: "woodland",
        },
        PolygonLayer {
            source: "ordnance-survey/OS_Open_Zoomstack.gpkg",
            layer: "local_buildings",
            output: "buildings",
        },
        PolygonLayer {
            source: "forestry-england/Forestry_England_Recreation_Areas_4445405460243066176.gpkg",
            layer: "FE_RecAreas_170226",
            output: "recreation-areas",
        },
        PolygonLayer {
            source: "forestry-england/Forestry_England_Quarries_-8301489482564464389.gpkg",
            layer: "FE_Quarries_160226",
            output: "quarries",
        },
        PolygonLayer {
            source: "forestry-england/Forestry_England_Management_Coupes_8744773793166487482.gpkg",
            layer: "FE_ManagementCoupes_130226",
            output: "management-coupes",
        },
    ];
}
