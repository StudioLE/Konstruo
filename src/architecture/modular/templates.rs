use crate::architecture::*;
use crate::geometry::{Orientation, Vec6};
use bevy::prelude::{default, JustifyContent};

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
    pub fn _2b4p1s0g() -> ModularBuildingInfo {
        ModularBuildingInfo {
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
    pub fn _2b4p1s0g_perp() -> ModularBuildingInfo {
        ModularBuildingInfo {
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
    pub fn _3b6p2s2g() -> ModularBuildingInfo {
        ModularBuildingInfo {
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
    pub fn _3b6p2s2g_perp() -> ModularBuildingInfo {
        ModularBuildingInfo {
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
    pub fn _4b8p2s2g() -> ModularBuildingInfo {
        ModularBuildingInfo {
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
    pub fn _4b8p2s2g_perp() -> ModularBuildingInfo {
        ModularBuildingInfo {
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
    fn _2b4p1s() -> BuildingModuleStackInfo {
        BuildingModuleStackInfo {
            modules: vec![
                BuildingModuleInfo {
                    level: 0,
                    width: 10.200,
                    length: 8.400,
                    openings: Some(vec![
                        OpeningDistribution {
                            side: Orientation::Front,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.800,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 0.900,
                                    height: 2.100,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 1.800,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                            ],
                        },
                        OpeningDistribution {
                            side: Orientation::Back,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 2.600,
                                    height: 2.100,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                            ],
                        },
                    ]),
                    ..default()
                },
                BuildingModuleInfo {
                    level: 1,
                    width: 10.200,
                    length: 8.400,
                    height: ROOF_HEIGHT,
                    pitch: Some(Pitch::FrontToBack),
                    ..default()
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
    fn _2b4p1s_perp() -> BuildingModuleStackInfo {
        BuildingModuleStackInfo {
            modules: vec![
                BuildingModuleInfo {
                    level: 0,
                    width: 8.400,
                    length: 10.200,
                    openings: Some(vec![
                        OpeningDistribution {
                            side: Orientation::Front,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 0.900,
                                    height: 2.100,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                            ],
                        },
                        OpeningDistribution {
                            side: Orientation::Back,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 2.600,
                                    height: 2.100,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                            ],
                        },
                    ]),
                    ..default()
                },
                BuildingModuleInfo {
                    level: 1,
                    width: 8.400,
                    length: 10.200,
                    height: ROOF_HEIGHT,
                    pitch: Some(Pitch::LeftToRight),
                    ..default()
                },
            ],
        }
    }
    /// Beds: 2
    /// Persons: 4
    /// Storeys: 1
    /// NIA: 75 sq m
    /// Orientation: Parallel
    #[must_use]
    fn _2b4p1s_long() -> BuildingModuleStackInfo {
        BuildingModuleStackInfo {
            modules: vec![
                BuildingModuleInfo {
                    level: 0,
                    width: 12.000,
                    length: 7.200,
                    openings: Some(vec![
                        OpeningDistribution {
                            side: Orientation::Front,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.800,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 0.900,
                                    height: 2.100,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 1.800,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                            ],
                        },
                        OpeningDistribution {
                            side: Orientation::Back,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 2.600,
                                    height: 2.100,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                            ],
                        },
                    ]),
                    ..default()
                },
                BuildingModuleInfo {
                    level: 1,
                    width: 12.000,
                    length: 7.200,
                    height: ROOF_HEIGHT,
                    pitch: Some(Pitch::FrontToBack),
                    ..default()
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
    fn _2b4p1s_long_perp() -> BuildingModuleStackInfo {
        BuildingModuleStackInfo {
            modules: vec![
                BuildingModuleInfo {
                    level: 0,
                    width: 7.200,
                    length: 12.000,
                    openings: Some(vec![
                        OpeningDistribution {
                            side: Orientation::Front,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 0.900,
                                    height: 2.100,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                            ],
                        },
                        OpeningDistribution {
                            side: Orientation::Back,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 2.600,
                                    height: 2.100,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                            ],
                        },
                    ]),
                    ..default()
                },
                BuildingModuleInfo {
                    level: 1,
                    width: 7.200,
                    length: 12.000,
                    height: ROOF_HEIGHT,
                    pitch: Some(Pitch::LeftToRight),
                    ..default()
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
    fn _3b6p2s() -> BuildingModuleStackInfo {
        BuildingModuleStackInfo {
            modules: vec![
                BuildingModuleInfo {
                    level: 0,
                    width: 10.200,
                    length: 6.000,
                    openings: Some(vec![
                        OpeningDistribution {
                            side: Orientation::Front,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 0.900,
                                    height: 2.100,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                            ],
                        },
                        OpeningDistribution {
                            side: Orientation::Back,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300).with_left(0.800)),
                                },
                                OpeningInfo {
                                    width: 2.700,
                                    height: 2.100,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300).with_right(0.800)),
                                },
                            ],
                        },
                    ]),
                    ..default()
                },
                BuildingModuleInfo {
                    level: 1,
                    width: 10.200,
                    length: 6.000,
                    openings: Some(vec![
                        OpeningDistribution {
                            side: Orientation::Front,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 0.600,
                                    height: 1.000,
                                    margin: Some(Vec6::new(0.150, 0.150, 0.0, 0.0, 0.0, 0.300)),
                                },
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                            ],
                        },
                        OpeningDistribution {
                            side: Orientation::Back,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 0.600,
                                    height: 1.000,
                                    margin: Some(Vec6::new(0.150, 0.150, 0.0, 0.0, 0.0, 0.300)),
                                },
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                            ],
                        },
                    ]),
                    ..default()
                },
                BuildingModuleInfo {
                    level: 2,
                    width: 10.200,
                    length: 6.000,
                    height: ROOF_HEIGHT,
                    pitch: Some(Pitch::FrontToBack),
                    ..default()
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
    fn _3b6p2s_perp() -> BuildingModuleStackInfo {
        BuildingModuleStackInfo {
            modules: vec![
                BuildingModuleInfo {
                    level: 0,
                    width: 6.000,
                    length: 10.200,
                    openings: Some(vec![
                        OpeningDistribution {
                            side: Orientation::Front,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 0.900,
                                    height: 2.100,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                            ],
                        },
                        OpeningDistribution {
                            side: Orientation::Back,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 2.600,
                                    height: 2.100,
                                    margin: Some(Vec6::default().with_top(0.300).with_left(0.100)),
                                },
                            ],
                        },
                    ]),
                    ..default()
                },
                BuildingModuleInfo {
                    level: 1,
                    width: 6.000,
                    length: 10.200,
                    openings: Some(vec![
                        OpeningDistribution {
                            side: Orientation::Front,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 0.600,
                                    height: 1.000,
                                    margin: Some(Vec6::new(0.150, 0.150, 0.0, 0.0, 0.0, 0.300)),
                                },
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                            ],
                        },
                        OpeningDistribution {
                            side: Orientation::Back,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 0.600,
                                    height: 1.000,
                                    margin: Some(Vec6::new(0.150, 0.150, 0.0, 0.0, 0.0, 0.300)),
                                },
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                            ],
                        },
                    ]),
                    ..default()
                },
                BuildingModuleInfo {
                    level: 2,
                    width: 6.000,
                    length: 10.200,
                    height: ROOF_HEIGHT,
                    pitch: Some(Pitch::LeftToRight),
                    ..default()
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
    fn _4b8p2s() -> BuildingModuleStackInfo {
        BuildingModuleStackInfo {
            modules: vec![
                BuildingModuleInfo {
                    level: 0,
                    width: 10.200,
                    length: 7.200,
                    openings: Some(vec![
                        OpeningDistribution {
                            side: Orientation::Front,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 0.900,
                                    height: 2.100,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                            ],
                        },
                        OpeningDistribution {
                            side: Orientation::Back,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300).with_left(0.800)),
                                },
                                OpeningInfo {
                                    width: 2.700,
                                    height: 2.100,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300).with_right(0.800)),
                                },
                            ],
                        },
                    ]),
                    ..default()
                },
                BuildingModuleInfo {
                    level: 1,
                    width: 10.200,
                    length: 7.200,
                    openings: Some(vec![
                        OpeningDistribution {
                            side: Orientation::Front,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 0.600,
                                    height: 1.000,
                                    margin: Some(Vec6::new(0.150, 0.150, 0.0, 0.0, 0.0, 0.300)),
                                },
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                            ],
                        },
                        OpeningDistribution {
                            side: Orientation::Back,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 0.600,
                                    height: 1.000,
                                    margin: Some(Vec6::new(0.150, 0.150, 0.0, 0.0, 0.0, 0.300)),
                                },
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                            ],
                        },
                    ]),
                    ..default()
                },
                BuildingModuleInfo {
                    level: 2,
                    width: 10.200,
                    length: 7.200,
                    pitch: Some(Pitch::FrontToBack),
                    ..default()
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
    fn _4b8p2s_perp() -> BuildingModuleStackInfo {
        BuildingModuleStackInfo {
            modules: vec![
                BuildingModuleInfo {
                    level: 0,
                    width: 7.200,
                    length: 10.200,
                    openings: Some(vec![
                        OpeningDistribution {
                            side: Orientation::Front,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 0.900,
                                    height: 2.100,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                            ],
                        },
                        OpeningDistribution {
                            side: Orientation::Back,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 2.900,
                                    height: 2.100,
                                    margin: Some(Vec6::default().with_top(0.300).with_left(0.100)),
                                },
                            ],
                        },
                    ]),
                    ..default()
                },
                BuildingModuleInfo {
                    level: 1,
                    width: 7.200,
                    length: 10.200,
                    openings: Some(vec![
                        OpeningDistribution {
                            side: Orientation::Front,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 0.600,
                                    height: 1.000,
                                    margin: Some(Vec6::new(0.150, 0.150, 0.0, 0.0, 0.0, 0.300)),
                                },
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                            ],
                        },
                        OpeningDistribution {
                            side: Orientation::Back,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 0.600,
                                    height: 1.000,
                                    margin: Some(Vec6::new(0.150, 0.150, 0.0, 0.0, 0.0, 0.300)),
                                },
                                OpeningInfo {
                                    width: 1.200,
                                    height: 1.500,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                            ],
                        },
                    ]),
                    ..default()
                },
                BuildingModuleInfo {
                    level: 2,
                    width: 7.200,
                    length: 10.200,
                    pitch: Some(Pitch::LeftToRight),
                    ..default()
                },
            ],
        }
    }

    /// Garage bays: 2
    /// NIA: 26 sq m
    /// Orientation: Parallel
    #[must_use]
    fn _2g() -> BuildingModuleStackInfo {
        BuildingModuleStackInfo {
            modules: vec![
                BuildingModuleInfo {
                    level: 0,
                    width: 6.000,
                    length: 5.400,
                    margin: Some(Vec6::default().with_back(0.600)),
                    openings: Some(vec![
                        OpeningDistribution {
                            side: Orientation::Front,
                            justify_content: JustifyContent::Center,
                            openings: vec![OpeningInfo {
                                width: 4.800,
                                height: 2.100,
                                margin: Some(Vec6::default().with_top(0.300)),
                            }],
                        },
                        OpeningDistribution {
                            side: Orientation::Back,
                            justify_content: JustifyContent::SpaceEvenly,
                            openings: vec![
                                OpeningInfo {
                                    width: 0.900,
                                    height: 2.100,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                                OpeningInfo {
                                    width: 1.800,
                                    height: 1.200,
                                    margin: Some(Vec6::default().with_top(0.300)),
                                },
                            ],
                        },
                    ]),
                    ..default()
                },
                BuildingModuleInfo {
                    level: 1,
                    width: 6.000,
                    length: 5.400,
                    height: ROOF_HEIGHT,
                    margin: Some(Vec6::default().with_back(0.600)),
                    pitch: Some(Pitch::FrontToBack),
                    ..default()
                },
            ],
        }
    }
}
