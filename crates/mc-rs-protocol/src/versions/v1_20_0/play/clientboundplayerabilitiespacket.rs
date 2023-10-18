use mc_rs_macros::Transcode;

use crate::types::packets::abilities::ClientboundPlayerAbilityFlags;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayerAbilitiesPacket {
    pub flags: ClientboundPlayerAbilityFlags,
    pub flying_speed: f32,
    pub walking_speed: f32,
}
