use belly::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Image backgrounds for the main menu
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImageBackground {
    #[default]
    Plains,
}

impl ImageBackground {
    #[allow(dead_code)]
    pub(super) fn create(&self, _parent: Entity, mut _elements: Elements, mut _commands: Commands) {
        // TODO: Add backgrounds
    }
}
