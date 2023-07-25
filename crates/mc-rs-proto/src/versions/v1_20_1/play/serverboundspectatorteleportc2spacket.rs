use mc_rs_macros::Packet;
use uuid::Uuid;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundSpectatorTeleportC2SPacket {
    pub a: Uuid,
}
