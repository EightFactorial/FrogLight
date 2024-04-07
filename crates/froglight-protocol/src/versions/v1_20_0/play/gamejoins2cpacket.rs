use froglight_macros::FrogReadWrite;
use simdnbt::owned::Nbt;

use crate::common::{GameMode, GlobalPosition, ResourceKey};

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct GameJoinS2CPacket {
    pub player_id: u32,
    pub hardcore: bool,
    pub game_mode: GameMode,
    pub previous_game_mode: i8,
    pub worlds: Vec<ResourceKey>,
    pub registry: Nbt,
    pub world_type: ResourceKey,
    pub world: ResourceKey,
    pub seed: i64,
    #[frog(var)]
    pub max_players: i32,
    #[frog(var)]
    pub chunk_radius: u32,
    #[frog(var)]
    pub simulation_distance: u32,
    pub reduced_debug_info: bool,
    pub show_death_screen: bool,
    pub debug: bool,
    pub flat: bool,
    pub last_death: Option<GlobalPosition>,
    #[frog(var)]
    pub portal_cooldown: u32,
}
