use froglight_macros::FrogReadWrite;

use crate::common::{GameMode, GlobalPosition, ResourceKey};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct PlayerRespawnS2CPacket {
    pub dimension_type: ResourceKey,
    pub dimension: ResourceKey,
    pub seed: i64,
    pub game_mode: GameMode,
    pub previous_game_mode: i8,
    pub debug: bool,
    pub flat: bool,
    pub data: u8,
    pub last_death: Option<GlobalPosition>,
    #[frog(var)]
    pub portal_cooldown: u32,
}
