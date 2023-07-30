use mc_rs_macros::Transcode;

use crate::types::packets::abilities::ServerboundPlayerAbilityFlags;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundUpdatePlayerAbilitiesPacket {
    pub flags: ServerboundPlayerAbilityFlags,
}
