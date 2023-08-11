use bevy::prelude::*;
use mc_rs_proto::types::ResourceLocation;

use super::WorldType;

/// The `CurrentWorld` resource contains the current world of the player
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Resource)]
pub struct CurrentWorld {
    #[deref]
    pub name: WorldType,
    pub kind: WorldType,
}

impl CurrentWorld {
    pub fn new(name: ResourceLocation, kind: ResourceLocation) -> Self {
        Self {
            name: name.into(),
            kind: kind.into(),
        }
    }
}

/// The current world seed
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Resource)]
pub struct WorldSeed(pub i64);
