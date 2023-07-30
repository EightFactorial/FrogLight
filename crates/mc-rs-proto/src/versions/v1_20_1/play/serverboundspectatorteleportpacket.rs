use mc_rs_macros::Transcode;
use uuid::Uuid;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundSpectatorTeleportPacket {
    pub uuid: Uuid,
}
