use crate::beziers::{ControlType, CubicBezier, CubicBezierSpline};
use crate::infrastructure::{Way, WayControl, WayMaterials, WayMeshes};
use crate::ui::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use ControlType::*;

#[derive(Resource, Default)]
pub struct Drawing {
    pub points: Vec<Vec3>,
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
        mut meshes: ResMut<Assets<Mesh>>,
        way_meshes: Res<WayMeshes>,
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
        drawing.points.push(position);
        let len = drawing.points.len();
        let is_start = len == 1;
        let is_end = len >= 3 && (len - 1) % 2 == 0;
        let curve = (len - 1) / 2;
        let control_type = if is_start {
            Start
        } else if is_end {
            End
        } else {
            EndHandle
        };
        let bundle = (
            WayControl::bundle(&way_meshes, &materials, control_type, curve, position),
            Visibility::Visible,
        );
        commands.spawn(bundle);
        if is_end {
            trace!("Segment is complete");
            let way = Way::new(drawing.get_spline());
            way.spawn(&mut commands, &mut meshes, &way_meshes, &materials);
            // TODO: Delete all temporary controls
        }
    }

    #[must_use]
    #[allow(clippy::indexing_slicing)]
    fn get_spline(&self) -> CubicBezierSpline {
        let mut curves = Vec::new();
        for i in 0..self.points.len() - 1 {
            if i % 2 != 0 {
                continue;
            }
            let start = self.points[i];
            let start_handle = self.points[i + 1];
            let end = self.points[i + 2];
            let next_handle = self.points.get(i + 3);
            let end_handle = if let Some(next_handle) = next_handle {
                let translation = end - *next_handle;
                end + translation
            } else {
                start_handle
            };
            curves.push(CubicBezier {
                start,
                start_handle,
                end_handle,
                end,
            });
        }
        CubicBezierSpline { curves }
    }
}
