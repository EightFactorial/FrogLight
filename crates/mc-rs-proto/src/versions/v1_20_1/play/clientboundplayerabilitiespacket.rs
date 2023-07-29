use mc_rs_macros::Transcode;

use crate::types::packets::abilities::PlayerAbilityFlags;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayerAbilitiesPacket {
    pub flags: PlayerAbilityFlags,
    pub flying_speed: f32,
    pub walking_speed: f32,
}
