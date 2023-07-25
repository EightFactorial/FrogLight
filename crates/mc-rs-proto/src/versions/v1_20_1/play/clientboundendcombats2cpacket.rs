use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundEndCombatS2CPacket {
    pub a: u32,
}
