use crate::beziers::{ControlType, CubicBezier, CubicBezierSpline};
use crate::infrastructure::{Way, WayControl, WayMaterials, WayMeshes};
use crate::ui::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use ControlType::*;

#[derive(Resource, Default)]
pub struct Drawing {
    pub controls: Vec<Vec3>,
}

impl Drawing {
    /// System to draw a [`Way`]
    #[allow(clippy::too_many_arguments, clippy::integer_division)]
    pub(super) fn update_system(
        mut commands: Commands,
        interface: Res<InterfaceState>,
        mut drawing: ResMut<Drawing>,
        buttons: Res<ButtonInput<MouseButton>>,
        window: Query<&Window, With<PrimaryWindow>>,
        camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
        meshes: Res<WayMeshes>,
        materials: Res<WayMaterials>,
    ) {
        if *interface != InterfaceState::DrawWay {
            return;
        }
        if !buttons.just_pressed(MouseButton::Left) {
            return;
        }
        let Ok(position) = Cursor::on_ground(&window, &camera) else {
            warn!("Failed to get Cursor position");
            return;
        };
        drawing.controls.push(position);
        let index = drawing.controls.len() - 1;
        let curve = index / 4;
        let control_type = ControlType::by_index(index);
        if control_type == End {
            let way = Way::new(drawing.get_spline());
            commands.spawn(way);
            drawing.controls.push(position);
            // TODO: Delete all temporary controls
        } else {
            let bundle = (
                WayControl::bundle(&meshes, &materials, control_type, curve, position),
                Visibility::Visible,
            );
            commands.spawn(bundle);
        }
    }

    #[must_use]
    #[allow(clippy::indexing_slicing)]
    fn get_spline(&self) -> CubicBezierSpline {
        let curves = self
            .controls
            .chunks(4)
            .filter(|chunk| chunk.len() == 4)
            .map(|chunk| CubicBezier {
                start: chunk[0],
                start_handle: chunk[1],
                end_handle: chunk[2],
                end: chunk[3],
            })
            .collect();
        CubicBezierSpline { curves }
    }
}
