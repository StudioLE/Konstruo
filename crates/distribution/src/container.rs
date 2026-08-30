use super::*;
use bevy::prelude::*;
use serde::Serialize;

/// Container of distributed items
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Container {
    /// Size
    pub size: Vec3,
    /// Items
    pub items: Vec<Distributed>,
}

/// An entity with a transform scaled to the size of [`Container`].
#[derive(Clone, Component, Debug, Default)]
pub struct DiagnosticContainer;
