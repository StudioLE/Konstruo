use bevy::app::PluginGroupBuilder;
use bevy::asset::AssetMetaCheck;
use bevy::log::Level;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use konstruo::architecture::*;
use konstruo::distribution::DistributionPlugin;
use konstruo::environment::*;
use konstruo::examples::*;
use konstruo::infrastructure::*;
use konstruo::ui::InteractionPlugin;
use konstruo::ui::*;
use std::collections::HashMap;

fn main() {
    App::new()
        .add_plugins(configure_default_plugins())
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

fn configure_default_plugins() -> PluginGroupBuilder {
    let plugins = DefaultPlugins.set(configure_log_plugin());
    if cfg!(target_arch = "wasm32") {
        configure_webassembly(plugins)
    } else if cfg!(windows) {
        configure_windows(plugins)
    } else {
        plugins
    }
}

#[allow(suspicious_double_ref_op)]
fn configure_log_plugin() -> LogPlugin {
    let mut filter: HashMap<&str, Vec<&str>> = HashMap::new();
    filter.insert(
        "info",
        vec![
            "bevy_render",
            "calloop",
            "cosmic_text",
            "naga",
            "offset_allocator",
            "polling",
            "wgpu_core",
            "wgpu_hal",
            "winit",
        ],
    );
    filter.insert("warn", vec!["wgpu_hal"]);
    LogPlugin {
        level: Level::TRACE,
        filter: create_log_filter(filter),
        ..default()
    }
}

#[allow(suspicious_double_ref_op)]
fn create_log_filter(filter: HashMap<&str, Vec<&str>>) -> String {
    filter
        .iter()
        .flat_map(|(level, names)| names.iter().map(|name| (level.clone(), name)))
        .fold(String::new(), |mut acc, (level, name)| {
            acc.push_str(name);
            acc.push('=');
            acc.push_str(level);
            acc.push(',');
            acc
        })
}

#[allow(suspicious_double_ref_op)]
fn configure_webassembly(plugins: PluginGroupBuilder) -> PluginGroupBuilder {
    trace!("Configuring DefaultPlugins for WebAssembly");
    plugins
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
}

fn configure_windows(plugins: PluginGroupBuilder) -> PluginGroupBuilder {
    trace!("Configuring DefaultPlugins for Windows");
    plugins.set(RenderPlugin {
        render_creation: RenderCreation::Automatic(WgpuSettings {
            backends: Some(Backends::VULKAN),
            ..default()
        }),
        ..default()
    })
}
