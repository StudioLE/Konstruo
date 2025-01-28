use super::*;
use bevy::prelude::*;

pub struct FlexFactory {
    pub(super) main_axis: Vec3,
    pub(super) cross_axis: Vec3,
    pub(super) flex_wrap: FlexWrap,
    pub(super) justify_content: JustifyContent,
    pub(super) align_content: AlignContent,
    pub(super) align_items: AlignItems,
    pub(super) container: Option<Vec3>,
    pub(super) items: Vec<Vec3>,
}

impl Default for FlexFactory {
    fn default() -> Self {
        FlexFactory {
            main_axis: Vec3::X,
            cross_axis: Vec3::Y,
            flex_wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_content: AlignContent::FlexStart,
            align_items: AlignItems::FlexStart,
            container: None,
            items: Vec::new(),
        }
    }
}

impl FlexFactory {
    #[must_use]
    pub fn execute(self) -> Container {
        let items = self
            .items
            .into_iter()
            .map(|original_size| Item {
                original_size,
                size: original_size,
                translation: Vec3::ZERO,
            })
            .collect();
        let mut container = Container {
            size: self.container.unwrap_or(Vec3::ZERO),
            items,
        };
        let taffy = TaffyFlexFactory {
            main_axis: self.main_axis,
            cross_axis: self.cross_axis,
            flex_wrap: self.flex_wrap,
            justify_content: self.justify_content,
            align_content: self.align_content,
            align_items: self.align_items,
        };
        taffy.execute(&mut container);
        container
    }
}
