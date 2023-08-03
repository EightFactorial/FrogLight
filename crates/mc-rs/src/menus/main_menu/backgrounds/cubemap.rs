use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Cube map backgrounds for the main menu
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CubeMapBackground {
    #[default]
    Plains,
    Village,
    Desert,
    DesertVillage,
    Ocean,
    WarmOcean,
    Mountains,
    Cave,
    Cavern,
}

impl CubeMapBackground {
    pub(super) fn create(&self, _parent: Entity, mut _commands: Commands) {
        // TODO: Add backgrounds
    }
}
