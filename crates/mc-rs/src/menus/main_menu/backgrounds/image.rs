use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Image backgrounds for the main menu
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImageBackground {
    #[default]
    Plains,
}

impl ImageBackground {
    pub(super) fn create(&self, _parent: Entity, mut _commands: Commands) {
        // TODO: Add backgrounds
    }
}
