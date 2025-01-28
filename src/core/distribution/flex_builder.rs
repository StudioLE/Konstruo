use super::*;
use crate::architecture::BuildingModule;
use crate::distribution::distributable::Distributable;
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
