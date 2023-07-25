use mc_rs_macros::Transcode;
use uuid::Uuid;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundSpectatorTeleportC2SPacket {
    pub a: Uuid,
}
