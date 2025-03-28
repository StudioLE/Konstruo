use crate::architecture::{
    BuildingModule, BuildingModuleStackFactory, ModularBuildingFactory, Pitch,
};
use crate::geometry::Vec6;

const LEVEL_HEIGHT: f32 = 2.400;
const ROOF_HEIGHT: f32 = 1.800;

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
    /// Storeys: 1
    /// NIA: 75 sq m
    /// Orientation: Parallel
    #[must_use]
    fn _2b4p1s() -> BuildingModuleStackFactory {
        BuildingModuleStackFactory {
            modules: vec![
                BuildingModule {
                    level: 0,
                    width: 12.000,
                    length: 7.200,
                    height: LEVEL_HEIGHT,
                    margin: None,
                    pitch: None,
                },
                BuildingModule {
                    level: 1,
                    width: 12.000,
                    length: 7.200,
                    height: ROOF_HEIGHT,
                    margin: None,
                    pitch: Some(Pitch::FrontToBack),
                },
            ],
        }
    }

    /// Beds: 2
    /// Persons: 4
    /// Storeys: 1
    /// NIA: 75 sq m
    /// Orientation: Perpendicular
    #[must_use]
    fn _2b4p1s_perp() -> BuildingModuleStackFactory {
        BuildingModuleStackFactory {
            modules: vec![
                BuildingModule {
                    level: 0,
                    width: 7.200,
                    length: 12.000,
                    height: LEVEL_HEIGHT,
                    margin: None,
                    pitch: None,
                },
                BuildingModule {
                    level: 1,
                    width: 7.200,
                    length: 12.000,
                    height: ROOF_HEIGHT,
                    margin: None,
                    pitch: Some(Pitch::LeftToRight),
                },
            ],
        }
    }

    /// Beds: 3
    /// Persons: 6
    /// Storeys: 2
    /// NIA: 104 sq m
    /// Orientation: Parallel
    #[must_use]
    fn _3b6p2s() -> BuildingModuleStackFactory {
        BuildingModuleStackFactory {
            modules: vec![
                BuildingModule {
                    level: 0,
                    width: 10.200,
                    length: 6.000,
                    height: LEVEL_HEIGHT,
                    margin: None,
                    pitch: None,
                },
                BuildingModule {
                    level: 1,
                    width: 10.200,
                    length: 6.000,
                    height: LEVEL_HEIGHT,
                    margin: None,
                    pitch: None,
                },
                BuildingModule {
                    level: 2,
                    width: 10.200,
                    length: 6.000,
                    height: ROOF_HEIGHT,
                    margin: None,
                    pitch: Some(Pitch::FrontToBack),
                },
            ],
        }
    }

    /// Beds: 3
    /// Persons: 6
    /// Storeys: 2
    /// NIA: 104 sq m
    /// Orientation: Perpendicular
    #[must_use]
    fn _3b6p2s_perp() -> BuildingModuleStackFactory {
        BuildingModuleStackFactory {
            modules: vec![
                BuildingModule {
                    level: 0,
                    width: 6.000,
                    length: 10.200,
                    height: LEVEL_HEIGHT,
                    margin: None,
                    pitch: None,
                },
                BuildingModule {
                    level: 1,
                    width: 6.000,
                    length: 10.200,
                    height: LEVEL_HEIGHT,
                    margin: None,
                    pitch: None,
                },
                BuildingModule {
                    level: 2,
                    width: 6.000,
                    length: 10.200,
                    height: ROOF_HEIGHT,
                    margin: None,
                    pitch: Some(Pitch::LeftToRight),
                },
            ],
        }
    }

    /// Beds: 4
    /// Persons: 8
    /// Storeys: 2
    /// NIA: 127 sq m
    /// Orientation: Parallel
    #[must_use]
    fn _4b8p2s() -> BuildingModuleStackFactory {
        BuildingModuleStackFactory {
            modules: vec![
                BuildingModule {
                    level: 0,
                    width: 10.200,
                    length: 7.200,
                    height: LEVEL_HEIGHT,
                    margin: None,
                    pitch: None,
                },
                BuildingModule {
                    level: 1,
                    width: 10.200,
                    length: 7.200,
                    height: LEVEL_HEIGHT,
                    margin: None,
                    pitch: None,
                },
                BuildingModule {
                    level: 2,
                    width: 10.200,
                    length: 7.200,
                    height: LEVEL_HEIGHT,
                    margin: None,
                    pitch: Some(Pitch::FrontToBack),
                },
            ],
        }
    }

    /// Beds: 4
    /// Persons: 8
    /// Storeys: 2
    /// NIA: 127 sq m
    /// Orientation: Perpendicular
    #[must_use]
    fn _4b8p2s_perp() -> BuildingModuleStackFactory {
        BuildingModuleStackFactory {
            modules: vec![
                BuildingModule {
                    level: 0,
                    width: 7.200,
                    length: 10.200,
                    height: LEVEL_HEIGHT,
                    margin: None,
                    pitch: None,
                },
                BuildingModule {
                    level: 1,
                    width: 7.200,
                    length: 10.200,
                    height: LEVEL_HEIGHT,
                    margin: None,
                    pitch: None,
                },
                BuildingModule {
                    level: 2,
                    width: 7.200,
                    length: 10.200,
                    height: LEVEL_HEIGHT,
                    margin: None,
                    pitch: Some(Pitch::LeftToRight),
                },
            ],
        }
    }

    /// Garage bays: 2
    /// NIA: 26 sq m
    /// Orientation: Parallel
    #[must_use]
    fn _2g() -> BuildingModuleStackFactory {
        BuildingModuleStackFactory {
            modules: vec![
                BuildingModule {
                    level: 0,
                    width: 6.000,
                    length: 5.400,
                    height: LEVEL_HEIGHT,
                    margin: Some(Vec6::default().with_back(0.600)),
                    pitch: None,
                },
                BuildingModule {
                    level: 1,
                    width: 6.000,
                    length: 5.400,
                    height: ROOF_HEIGHT,
                    margin: Some(Vec6::default().with_back(0.600)),
                    pitch: Some(Pitch::FrontToBack),
                },
            ],
        }
    }
}
