use bevy::color::palettes::tailwind;
use bevy::prelude::*;

pub struct ExampleMaterials;

impl ExampleMaterials {
    pub(super) fn blue_face_transparent() -> StandardMaterial {
        StandardMaterial {
            base_color: tailwind::SKY_300.with_alpha(0.05).into(),
            perceptual_roughness: 1.0,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }
    }

    pub(super) fn blue_face() -> StandardMaterial {
        StandardMaterial {
            base_color: tailwind::SKY_300.with_alpha(0.5).into(),
            perceptual_roughness: 1.0,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }
    }

    pub(super) fn blue_edge() -> StandardMaterial {
        StandardMaterial {
            base_color: tailwind::SKY_300.into(),
            perceptual_roughness: 1.0,
            unlit: true,
            ..default()
        }
    }

    pub(super) fn red_face() -> StandardMaterial {
        StandardMaterial {
            base_color: tailwind::RED_600.with_alpha(0.5).into(),
            perceptual_roughness: 1.0,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }
    }

    pub(super) fn red_edge() -> StandardMaterial {
        StandardMaterial {
            base_color: tailwind::RED_600.into(),
            perceptual_roughness: 1.0,
            unlit: true,
            ..default()
        }
    }
}
