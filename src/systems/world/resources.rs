use bevy::prelude::*;
use mc_rs_proto::types::ResourceLocation;

/// The `WorldType` enum represents the type of a world.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum WorldType {
    Nether,
    #[default]
    Overworld,
    End,
    Other(ResourceLocation),
}

impl From<ResourceLocation> for WorldType {
    fn from(value: ResourceLocation) -> Self {
        match value.as_str() {
            "minecraft:the_nether" => WorldType::Nether,
            "minecraft:overworld" => WorldType::Overworld,
            "minecraft:the_end" => WorldType::End,
            _ => WorldType::Other(value),
        }
    }
}

impl From<&str> for WorldType {
    fn from(value: &str) -> Self {
        match value {
            "minecraft:the_nether" | "the_nether" => WorldType::Nether,
            "minecraft:overworld" | "overworld" => WorldType::Overworld,
            "minecraft:the_end" | "the_end" => WorldType::End,
            _ => WorldType::Other(value.into()),
        }
    }
}

/// The current world of the player
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Resource)]
pub struct CurrentWorld {
    #[deref]
    pub name: WorldType,
    pub kind: WorldType,
}

impl CurrentWorld {
    pub fn new(name: impl Into<ResourceLocation>, kind: impl Into<ResourceLocation>) -> Self {
        Self {
            name: WorldType::from(name.into()),
            kind: WorldType::from(kind.into()),
        }
    }
}

/// The current world seed
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Resource)]
pub struct WorldSeed(pub i64);
