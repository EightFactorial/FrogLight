use froglight_common::ResourceKey;
use froglight_macros::FrogReadWrite;

use crate::common::{GameMode, GlobalPosition};

/// Information about the player's spawn.
#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct SpawnInformation {
    /// The dimension ID
    #[frog(var)]
    pub dimension_id: u32,
    /// The name of the dimension
    pub dimension_name: ResourceKey,
    /// The seed of the world
    pub seed: i64,
    /// The player's game mode
    pub gamemode: GameMode,
    /// The player's previous game mode
    pub previous_gamemode: i8,
    /// Whether the player is in debug mode
    pub debug: bool,
    /// Whether the world is flat
    pub flat: bool,
    /// The position of the player's last death
    pub last_death: Option<GlobalPosition>,
    /// The cooldown before the player can use a portal
    #[frog(var)]
    pub portal_cooldown: u32,
}
