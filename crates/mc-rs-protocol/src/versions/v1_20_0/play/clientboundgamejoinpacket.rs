use std::fmt::Debug;

use azalea_nbt::Nbt;
use mc_rs_macros::Transcode;

use crate::types::{enums::GameMode, position::GlobalPos, ResourceLocation};

// TODO: Create tests for this packet.
#[derive(Clone, PartialEq, Transcode)]
pub struct ClientboundGameJoinPacket {
    pub player_id: u32,
    pub hardcore: bool,
    pub game_mode: GameMode,
    pub previous_game_mode: i8,
    pub worlds: Vec<ResourceLocation>,
    pub registry: Nbt,
    pub world_type: ResourceLocation,
    pub world: ResourceLocation,
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

impl Debug for ClientboundGameJoinPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientboundGameJoinPacket")
            .field("player_id", &self.player_id)
            .field("hardcore", &self.hardcore)
            .field("game_mode", &self.game_mode)
            .field("previous_game_mode", &self.previous_game_mode)
            .field("worlds", &self.worlds)
            .field("world_type", &self.world_type)
            .field("world", &self.world)
            .field("seed", &self.seed)
            .field("max_players", &self.max_players)
            .field("chunk_radius", &self.chunk_radius)
            .field("simulation_distance", &self.simulation_distance)
            .field("reduced_debug_info", &self.reduced_debug_info)
            .field("show_death_screen", &self.show_death_screen)
            .field("debug", &self.debug)
            .field("flat", &self.flat)
            .field("last_death", &self.last_death)
            .field("portal_cooldown", &self.portal_cooldown)
            .finish()
    }
}
