use super::*;
use bevy::prelude::*;

pub struct FlexFactory {
    pub(super) main_axis: Vec3,
    pub(super) cross_axis: Vec3,
    pub(super) flex_wrap: FlexWrap,
    pub(super) justify_content: JustifyContent,
    pub(super) align_content: AlignContent,
    pub(super) align_items_cross: AlignItems,
    pub(super) align_items_normal: AlignItems,
    pub(super) bounds: Option<Vec3>,
    pub(super) items: Vec<SourceItem>,
}

impl Default for FlexFactory {
    fn default() -> Self {
        FlexFactory {
            main_axis: Vec3::X,
            cross_axis: Vec3::Y,
            flex_wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_content: AlignContent::FlexStart,
            align_items_cross: AlignItems::FlexStart,
            align_items_normal: AlignItems::FlexStart,
            bounds: None,
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
            .map(Item::from)
            .collect();
        let mut container = Container {
            size: Vec3::ZERO,
            items,
        };
        let normal = self.main_axis.cross(self.cross_axis);
        let taffy = TaffyFlexFactory {
            main_axis: Vec3::ZERO,
            cross_axis: normal,
            flex_wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::Start,
            align_content: AlignContent::Start,
            align_items: self.align_items_normal,
            bounds: None,
        };
        taffy.execute(&mut container);
        let taffy = TaffyFlexFactory {
            main_axis: self.main_axis,
            cross_axis: self.cross_axis,
            flex_wrap: self.flex_wrap,
            justify_content: self.justify_content,
            align_content: self.align_content,
            align_items: self.align_items_cross,
            bounds: self.bounds,
        };
        taffy.execute(&mut container);
        container
    }
}
