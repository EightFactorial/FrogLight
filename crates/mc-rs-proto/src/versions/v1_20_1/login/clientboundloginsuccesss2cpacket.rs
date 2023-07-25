use mc_rs_macros::Transcode;

use crate::types::GameProfile;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundLoginSuccessS2CPacket {
    pub profile: GameProfile,
}
