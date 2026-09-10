use crate::{HeightChunkAsset, HeightChunkLoader, HeightChunkMesh, HeightChunkSettings, Terrain};
use bevy::prelude::*;
use konstruo_geography_core::{ChunkIndex, ChunkPath, HeightSpacing};

/// Directory holding the height chunks, relative to `assets/`.
const DIRECTORY: &str = "terrain";

/// Index of the chunk loaded at startup.
const INDEX: ChunkIndex = ChunkIndex::new(0, 0);

/// Spacing of the chunk loaded at startup.
const SPACING: HeightSpacing = HeightSpacing::Sixteen;

/// Registers the [`HeightChunkAsset`] asset with its [`HeightChunkLoader`], and
/// spawns [`Terrain`] for one chunk.
pub struct TerrainPlugin;

/// Handle to the loading height chunk asset.
///
/// Held so the asset is not dropped before it finishes loading.
#[derive(Resource)]
struct TerrainAsset(Handle<HeightChunkAsset>);

impl TerrainPlugin {
    /// System to start loading the height chunk asset on startup.
    ///
    /// - Derives the path and the settings from [`INDEX`] and [`SPACING`], so
    ///   the bytes cannot be decoded against another spacing
    /// - Builds the load with [`AssetServer::load_builder`], the settings
    ///   overload of [`AssetServer::load`] being deprecated
    fn startup_system(mut commands: Commands, assets: Res<AssetServer>) {
        let path = ChunkPath::new(DIRECTORY).get(SPACING, INDEX);
        let handle = assets
            .load_builder()
            .with_settings(|settings: &mut HeightChunkSettings| {
                settings.spacing = Some(SPACING);
                settings.index = Some(INDEX);
            })
            .load(path);
        commands.insert_resource(TerrainAsset(handle));
    }

    /// System to spawn [`Terrain`] when the asset finishes loading.
    ///
    /// - Spawns again on every load, so a hot reload would duplicate the terrain
    /// - Loads once at startup in practice, the `file_watcher` feature being off
    fn asset_system(
        mut commands: Commands,
        mut events: MessageReader<AssetEvent<HeightChunkAsset>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        chunks: Res<Assets<HeightChunkAsset>>,
        source: Res<TerrainAsset>,
    ) {
        for event in events.read() {
            let AssetEvent::LoadedWithDependencies { id } = *event else {
                continue;
            };
            if id != source.0.id() {
                continue;
            }
            let Some(loaded) = chunks.get(id) else {
                warn!("Failed to spawn terrain: asset was unavailable");
                continue;
            };
            let triangles = HeightChunkMesh::from_chunk(&loaded.chunk);
            debug!(
                "Spawning terrain of {} vertices and {} triangles",
                triangles.get_triangles().len() * 3,
                triangles.get_triangles().len()
            );
            let mesh = meshes.add(triangles.to_mesh());
            let material = materials.add(Terrain::material());
            let bundle = Terrain::bundle(mesh, material);
            commands.spawn(bundle);
        }
    }
}

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<HeightChunkAsset>()
            .init_asset_loader::<HeightChunkLoader>()
            .add_systems(Startup, TerrainPlugin::startup_system)
            .add_systems(Update, TerrainPlugin::asset_system);
    }
}
