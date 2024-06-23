use froglight_macros::FrogReadWrite;

use crate::packet::{PlayerRespawnFlags, SpawnInformation};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct PlayerRespawnPacket {
    pub spawn_info: SpawnInformation,
    pub flags: PlayerRespawnFlags,
}
