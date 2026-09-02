use bevy::color::palettes::*;
use bevy::prelude::*;
use bevy::render::render_resource::Face;
use konstruo_core::constants::WATER_ELEVATION;
use konstruo_gis::{GeoJsonPlugin, GeoJsonPolygons};

/// Path of the surface water asset, relative to `assets/`.
const SOURCE: &str = "gis/kielder-water.geojson";

/// A graphical representation of surface water as a flat filled surface.
#[derive(Component)]
pub struct SurfaceWater;

/// A graphical representation of surface water as a flat filled surface.
pub struct SurfaceWaterPlugin;

/// Handle to the loading surface water asset.
///
/// Held so the asset is not dropped before it finishes loading.
#[derive(Resource)]
struct SurfaceWaterAsset(Handle<GeoJsonPolygons>);

impl SurfaceWater {
    /// System to start loading the surface water asset on startup.
    fn startup_system(mut commands: Commands, assets: Res<AssetServer>) {
        commands.insert_resource(SurfaceWaterAsset(assets.load(SOURCE)));
    }

    /// System to spawn [`SurfaceWater`] when the asset finishes loading.
    ///
    /// - Spawns again on every load, so a hot reload would duplicate the water
    /// - Loads once at startup in practice, the `file_watcher` feature being off
    fn asset_system(
        mut commands: Commands,
        mut events: MessageReader<AssetEvent<GeoJsonPolygons>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        polygons: Res<Assets<GeoJsonPolygons>>,
        source: Res<SurfaceWaterAsset>,
    ) {
        for event in events.read() {
            let AssetEvent::LoadedWithDependencies { id } = *event else {
                continue;
            };
            if id != source.0.id() {
                continue;
            }
            let Some(loaded) = polygons.get(id) else {
                warn!("Failed to spawn surface water: asset was unavailable");
                continue;
            };
            let material = materials.add(SurfaceWater::material());
            for polygon in &loaded.polygons {
                let triangles = match polygon.triangulate() {
                    Ok(triangles) => triangles,
                    Err(error) => {
                        warn!("Skipped a surface water polygon: {error}");
                        continue;
                    }
                };
                debug!(
                    "Spawning surface water of {} vertices and {} triangles",
                    triangles.get_positions().len(),
                    triangles.get_triangle_count()
                );
                let mesh = meshes.add(triangles.to_mesh(Vec3::Z));
                let bundle = SurfaceWater::bundle(mesh, material.clone());
                commands.spawn(bundle);
            }
        }
    }

    /// Create a bundle for [`SurfaceWater`].
    fn bundle(mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> impl Bundle {
        (
            SurfaceWater,
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_xyz(0.0, 0.0, WATER_ELEVATION),
        )
    }

    /// Create the [`SurfaceWater`] material.
    ///
    /// - Mirrors [`crate::Ground`] but smoother, so the two read as a pair
    fn material() -> StandardMaterial {
        StandardMaterial {
            base_color: tailwind::SKY_700.into(),
            perceptual_roughness: 0.2,
            double_sided: true,
            cull_mode: Some(Face::Back),
            ..default()
        }
    }
}

impl Plugin for SurfaceWaterPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<GeoJsonPlugin>() {
            app.add_plugins(GeoJsonPlugin);
        }
        app.add_systems(Startup, SurfaceWater::startup_system)
            .add_systems(Update, SurfaceWater::asset_system);
    }
}
