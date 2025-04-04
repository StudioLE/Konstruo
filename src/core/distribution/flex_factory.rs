use super::*;
use bevy::prelude::*;

#[derive(Clone, Debug)]
pub struct FlexFactory {
    pub main_axis: Vec3,
    pub cross_axis: Vec3,
    pub flex_wrap: FlexWrap,
    pub justify_content: JustifyContent,
    pub align_content: AlignContent,
    pub align_items_cross: AlignItems,
    pub align_items_normal: AlignItems,
    pub gap: Vec3,
    pub bounds: Option<Vec3>,
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
            gap: Vec3::ZERO,
            bounds: None,
        }
    }
}

impl FlexFactory {
    #[must_use]
    pub fn execute(&self, items: Vec<Distributable>) -> Container {
        let items = items
            .into_iter()
            .map(Distributable::to_distributed)
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
            gap: self.gap,
            bounds: self.bounds,
        };
        taffy.execute(&mut container);
        let taffy = TaffyFlexFactory {
            main_axis: self.main_axis,
            cross_axis: self.cross_axis,
            flex_wrap: self.flex_wrap,
            justify_content: self.justify_content,
            align_content: self.align_content,
            align_items: self.align_items_cross,
            gap: self.gap,
            bounds: self.bounds,
        };
        taffy.execute(&mut container);
        container
    }
}
