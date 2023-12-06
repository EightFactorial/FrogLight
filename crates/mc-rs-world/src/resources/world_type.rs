use std::str::FromStr;

use mc_rs_protocol::types::ResourceLocation;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WorldType {
    Nether,
    Overworld,
    End,
    Other(ResourceLocation),
}

impl WorldType {
    /// Returns the name of the world type.
    pub fn as_str(&self) -> &str { self.as_ref() }
}

impl From<ResourceLocation> for WorldType {
    fn from(value: ResourceLocation) -> Self {
        match value.as_str() {
            "minecraft:the_nether" => Self::Nether,
            "minecraft:overworld" => Self::Overworld,
            "minecraft:the_end" => Self::End,
            _ => Self::Other(value),
        }
    }
}

impl From<WorldType> for ResourceLocation {
    fn from(value: WorldType) -> Self {
        match value {
            WorldType::Nether => Self::from("minecraft:the_nether"),
            WorldType::Overworld => Self::from("minecraft:overworld"),
            WorldType::End => Self::from("minecraft:the_end"),
            WorldType::Other(value) => value,
        }
    }
}

impl FromStr for WorldType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "minecraft:the_nether" => Self::Nether,
            "minecraft:overworld" => Self::Overworld,
            "minecraft:the_end" => Self::End,
            _ => Self::Other(ResourceLocation::try_from(s).ok_or("Invalid World name")?),
        })
    }
}

impl AsRef<str> for WorldType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Nether => "minecraft:the_nether",
            Self::Overworld => "minecraft:overworld",
            Self::End => "minecraft:the_end",
            Self::Other(value) => value.as_ref(),
        }
    }
}
