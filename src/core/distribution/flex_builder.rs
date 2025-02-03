use super::*;
use crate::geometry::Vec6;
use bevy::prelude::*;

pub struct FlexBuilder {
    factory: FlexFactory,
}

impl FlexBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            factory: FlexFactory::default(),
        }
    }

    #[must_use]
    pub fn with_axis(mut self, main_axis: Vec3, cross_axis: Vec3) -> Self {
        self.factory.main_axis = main_axis;
        self.factory.cross_axis = cross_axis;
        self
    }

    #[must_use]
    pub fn with_flex_wrap(mut self, flex_wrap: FlexWrap) -> Self {
        self.factory.flex_wrap = flex_wrap;
        self
    }

    #[must_use]
    pub fn with_bounds(mut self, bounds: Vec3) -> Self {
        self.factory.bounds = Some(bounds);
        self
    }

    #[must_use]
    pub fn without_bounds(mut self) -> Self {
        self.factory.bounds = None;
        self
    }

    #[must_use]
    pub fn with_justify_content(mut self, justify_content: JustifyContent) -> Self {
        self.factory.justify_content = justify_content;
        self
    }

    #[must_use]
    pub fn with_align_content(mut self, align_content: AlignContent) -> Self {
        self.factory.align_content = align_content;
        self
    }

    #[must_use]
    pub fn with_align_items_cross(mut self, align_items: AlignItems) -> Self {
        self.factory.align_items_cross = align_items;
        self
    }

    #[must_use]
    pub fn with_align_items_normal(mut self, align_items: AlignItems) -> Self {
        self.factory.align_items_normal = align_items;
        self
    }

    #[must_use]
    pub fn with_gap(mut self, gap: Vec3) -> Self {
        self.factory.gap = gap;
        self
    }

    #[must_use]
    pub fn with_items(mut self, sizes: Vec<SourceItem>) -> Self {
        for item in sizes {
            self.factory.items.push(item);
        }
        self
    }

    #[must_use]
    pub fn with_item(mut self, size: Vec3, margin: Vec6) -> Self {
        self.factory.items.push(SourceItem { size, margin });
        self
    }

    #[must_use]
    pub fn build(self) -> FlexFactory {
        self.factory
    }

    #[must_use]
    pub fn execute(self) -> Container {
        self.factory.execute()
    }
}
