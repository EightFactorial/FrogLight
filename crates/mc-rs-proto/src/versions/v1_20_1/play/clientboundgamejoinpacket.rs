use fastnbt::Value;
use mc_rs_macros::Transcode;

use crate::types::{enums::GameMode, position::GlobalPos, ResourceLocation};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundGameJoinPacket {
    pub player_id: u32,
    pub hardcore: bool,
    pub game_mode: GameMode,
    pub previous_game_mode: i8,
    pub levels: Vec<ResourceLocation>,
    pub registry: Value,
    pub dimension_type: ResourceLocation,
    pub dimension: ResourceLocation,
    pub seed: i64,
    #[var]
    pub max_players: i32,
    #[var]
    pub chunk_radius: u32,
    #[var]
    pub simulation_distance: u32,
    pub reduced_debug_info: bool,
    pub show_death_screen: bool,
    pub debug: bool,
    pub flat: bool,
    pub last_death: Option<GlobalPos>,
    #[var]
    pub portal_cooldown: u32,
}
