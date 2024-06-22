use froglight_macros::FrogReadWrite;

use crate::{common::ResourceKey, packet::SpawnInformation};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct GameJoinPacket {
    pub entity_id: u32,
    pub hardcore: bool,
    pub dimensions: Vec<ResourceKey>,
    #[frog(var)]
    pub max_players: u32,
    #[frog(var)]
    pub view_distance: u32,
    #[frog(var)]
    pub simulation_distance: u32,
    pub reduced_debug_info: bool,
    pub show_death_screen: bool,
    pub limited_crafting: bool,
    pub spawn_info: SpawnInformation,
    pub enforce_secure_chat: bool,
}
