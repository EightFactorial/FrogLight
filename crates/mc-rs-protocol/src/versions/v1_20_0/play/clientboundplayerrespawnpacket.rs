use mc_rs_macros::Transcode;

use crate::types::{enums::GameMode, position::GlobalPos, ResourceLocation};

// TODO: Create a test for this packet
#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
pub struct ClientboundPlayerRespawnPacket {
    pub dimension_type: ResourceLocation,
    pub dimension: ResourceLocation,
    pub seed: i64,
    pub game_mode: GameMode,
    pub previous_game_mode: i8,
    pub debug: bool,
    pub flat: bool,
    pub data: u8,
    pub last_death: Option<GlobalPos>,
    #[var]
    pub portal_cooldown: u32,
}
