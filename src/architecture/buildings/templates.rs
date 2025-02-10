use crate::architecture::{BuildingModule, BuildingModuleStack, ModularBuildingFactory, RoofStyle};
use crate::geometry::Vec6;
use bevy::prelude::default;

pub struct BuildingTemplates;
pub struct StackTemplates;

impl BuildingTemplates {
    /// Beds: 2
    /// Persons: 4
    /// Storeys: 2
    /// Garage bays: 0
    /// NIA: 75 sq m
    /// Orientation: Parallel
    #[must_use]
    pub fn _2b4p1s0g() -> ModularBuildingFactory {
        ModularBuildingFactory {
            stacks: vec![StackTemplates::_2b4p1s()],
        }
    }

    /// Beds: 2
    /// Persons: 4
    /// Storeys: 2
    /// Garage bays: 0
    /// NIA: 75 sq m
    /// Orientation: Perpendicular
    #[must_use]
    pub fn _2b4p1s0g_perp() -> ModularBuildingFactory {
        ModularBuildingFactory {
            stacks: vec![StackTemplates::_2b4p1s_perp()],
        }
    }

    /// Beds: 3
    /// Persons: 6
    /// Storeys: 2
    /// Garage bays: 2
    /// NIA: 104 sq m
    /// Orientation: Parallel
    #[must_use]
    pub fn _3b6p2s2g() -> ModularBuildingFactory {
        ModularBuildingFactory {
            stacks: vec![StackTemplates::_3b6p2s(), StackTemplates::_2g()],
        }
    }

    /// Beds: 3
    /// Persons: 6
    /// Storeys: 2
    /// Garage bays: 2
    /// NIA: 104 sq m
    /// Orientation: Perpendicular
    #[must_use]
    pub fn _3b6p2s2g_perp() -> ModularBuildingFactory {
        ModularBuildingFactory {
            stacks: vec![StackTemplates::_3b6p2s_perp(), StackTemplates::_2g()],
        }
    }

    /// Beds: 4
    /// Persons: 8
    /// Storeys: 2
    /// Garage bays: 2
    /// NIA: 127 sq m
    /// Orientation: Parallel
    #[must_use]
    pub fn _4b8p2s2g() -> ModularBuildingFactory {
        ModularBuildingFactory {
            stacks: vec![StackTemplates::_4b8p2s(), StackTemplates::_2g()],
        }
    }

    /// Beds: 4
    /// Persons: 8
    /// Storeys: 2
    /// Garage bays: 2
    /// NIA: 127 sq m
    /// Orientation: Perpendicular
    #[must_use]
    pub fn _4b8p2s2g_perp() -> ModularBuildingFactory {
        ModularBuildingFactory {
            stacks: vec![StackTemplates::_4b8p2s_perp(), StackTemplates::_2g()],
        }
    }
}

impl StackTemplates {
    /// Beds: 2
    /// Persons: 4
    /// Storeys: 2
    /// NIA: 75 sq m
    /// Orientation: Parallel
    #[must_use]
    fn _2b4p1s() -> BuildingModuleStack {
        BuildingModuleStack {
            definition: BuildingModule {
                width: 12.000,
                length: 7.200,
                ..default()
            },
            levels: 1,
            level_height: 2.400,
            roof_height: 1.800,
            roof_style: Some(RoofStyle::PitchFrontToBack),
        }
    }

    /// Beds: 2
    /// Persons: 4
    /// Storeys: 2
    /// NIA: 75 sq m
    /// Orientation: Perpendicular
    #[must_use]
    fn _2b4p1s_perp() -> BuildingModuleStack {
        BuildingModuleStack {
            definition: BuildingModule {
                width: 7.200,
                length: 12.000,
                ..default()
            },
            levels: 1,
            level_height: 2.400,
            roof_height: 1.800,
            roof_style: Some(RoofStyle::PitchLeftToRight),
        }
    }

    /// Beds: 3
    /// Persons: 6
    /// Storeys: 2
    /// NIA: 104 sq m
    /// Orientation: Parallel
    #[must_use]
    fn _3b6p2s() -> BuildingModuleStack {
        BuildingModuleStack {
            definition: BuildingModule {
                width: 10.200,
                length: 6.000,
                ..default()
            },
            levels: 2,
            level_height: 2.400,
            roof_height: 1.800,
            roof_style: Some(RoofStyle::PitchFrontToBack),
        }
    }

    /// Beds: 3
    /// Persons: 6
    /// Storeys: 2
    /// NIA: 104 sq m
    /// Orientation: Perpendicular
    #[must_use]
    fn _3b6p2s_perp() -> BuildingModuleStack {
        BuildingModuleStack {
            definition: BuildingModule {
                width: 6.000,
                length: 10.200,
                ..default()
            },
            levels: 2,
            level_height: 2.400,
            roof_height: 1.800,
            roof_style: Some(RoofStyle::PitchLeftToRight),
        }
    }

    /// Beds: 4
    /// Persons: 8
    /// Storeys: 2
    /// NIA: 127 sq m
    /// Orientation: Parallel
    #[must_use]
    fn _4b8p2s() -> BuildingModuleStack {
        BuildingModuleStack {
            definition: BuildingModule {
                width: 10.200,
                length: 7.200,
                ..default()
            },
            levels: 2,
            level_height: 2.400,
            roof_height: 2.400,
            roof_style: Some(RoofStyle::PitchFrontToBack),
        }
    }

    /// Beds: 4
    /// Persons: 8
    /// Storeys: 2
    /// NIA: 127 sq m
    /// Orientation: Perpendicular
    #[must_use]
    fn _4b8p2s_perp() -> BuildingModuleStack {
        BuildingModuleStack {
            definition: BuildingModule {
                width: 7.200,
                length: 10.200,
                ..default()
            },
            levels: 2,
            level_height: 2.400,
            roof_height: 2.400,
            roof_style: Some(RoofStyle::PitchLeftToRight),
        }
    }

    /// Garage bays: 2
    /// NIA: 26 sq m
    /// Orientation: Parallel
    #[must_use]
    fn _2g() -> BuildingModuleStack {
        BuildingModuleStack {
            definition: BuildingModule {
                width: 6.000,
                length: 5.400,
                margin: Some(Vec6::default().with_back(0.600)),
                ..default()
            },
            levels: 1,
            level_height: 2.400,
            roof_height: 1.800,
            roof_style: Some(RoofStyle::PitchFrontToBack),
        }
    }
}
