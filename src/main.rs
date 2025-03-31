use bevy::app::PluginGroupBuilder;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use konstruo::architecture::*;
use konstruo::distribution::DistributionPlugin;
use konstruo::environment::*;
use konstruo::examples::*;
use konstruo::infrastructure::*;
use konstruo::ui::InteractionPlugin;
use konstruo::ui::*;

fn main() {
    App::new()
        .add_plugins(default_plugins())
        .add_plugins(MeshPickingPlugin)
        .add_plugins(AxisMarkerPlugin)
        .add_plugins(ModularBuildingsPlugin)
        // .add_plugins(CursorGizmoPlugin)
        .add_plugins(DistributionPlugin)
        .add_plugins(InterfacePlugin)
        .add_plugins(InteractionPlugin)
        .add_plugins(GizmoPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(GroundPlugin)
        .add_plugins(OriginMarkerPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(SkyPlugin)
        .add_plugins(SunPlugin)
        .add_plugins(ViewCubePlugin)
        // .add_plugins(debug_plugin)
        .add_plugins(PathPlugin)
        // .add_plugins(AxisMarkerExample)
        // .add_plugins(BuildingsExample)
        // .add_plugins(FlexAlongBezierExample)
        .add_plugins(FlexBuildingsAlongPathExample)
        // .add_plugins(FlexNestedExample)
        // .add_plugins(FlexVerticallyExample)
        // .add_plugins(FlexWrappingExample)
        .add_plugins(PathExample)
        // .add_plugins(Shapes3DExample)
        // .add_plugins(SubdivisionExample)
        .run();
}

fn default_plugins() -> PluginGroupBuilder {
    if cfg!(target_arch = "wasm32") {
        trace!("Configuring DefaultPlugins for WebAssembly");
        DefaultPlugins
            .set(AssetPlugin {
                file_path: "/assets".to_owned(),
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some("#primary-window".into()),
                    ..default()
                }),
                ..default()
            })
    } else {
        DefaultPlugins.build()
    }
}
