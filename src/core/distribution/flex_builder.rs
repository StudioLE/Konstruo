use super::*;
use crate::architecture::BuildingModule;
use crate::distribution::distributable::Distributable;
use bevy::prelude::*;
use bevy::text::cosmic_text::Align;

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
    pub fn with_align_items(mut self, align_items: AlignItems) -> Self {
        self.factory.align_items = align_items;
        self
    }

    #[must_use]
    pub fn with_items<T: Distributable + 'static>(mut self, items: Vec<T>) -> Self {
        for item in items {
            self.factory.items.push(Box::new(item));
        }
        self
    }

    #[must_use]
    pub fn with_item<T: Distributable + 'static>(mut self, item: T) -> Self {
        // TODO: Instead of using a FlexItem pass the transform  to be set automatically.
        self.factory.items.push(Box::new(item));
        self
    }

    #[must_use]
    pub fn execute(self) -> Container {
        self.factory.execute()
    }
}
