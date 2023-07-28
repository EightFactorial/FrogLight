use mc_rs_macros::Transcode;
use uuid::Uuid;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayerRemovePacket {
    pub player_uuids: Vec<Uuid>,
}
