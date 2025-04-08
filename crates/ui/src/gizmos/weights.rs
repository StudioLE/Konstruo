use bevy::prelude::*;

pub struct GizmoWeights;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct Thin;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct Light;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct Medium;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct Bold;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct Heavy;

impl GizmoWeights {
    /// A system to configure different gizmo weights at startup.
    pub fn startup_system(mut config_store: ResMut<GizmoConfigStore>) {
        let (config, _) = config_store.config_mut::<Thin>();
        config.line.width = 0.3;
        config.depth_bias = -0.1;
        let (config, _) = config_store.config_mut::<Light>();
        config.line.width = 1.0;
        config.depth_bias = -0.3;
        let (config, _) = config_store.config_mut::<Medium>();
        config.line.width = 2.0;
        config.depth_bias = -0.5;
        let (config, _) = config_store.config_mut::<Bold>();
        config.line.width = 3.0;
        config.depth_bias = -0.7;
        let (config, _) = config_store.config_mut::<Heavy>();
        config.line.width = 4.0;
        config.depth_bias = -0.9;
    }
}
