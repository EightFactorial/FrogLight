use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundSetCameraEntityS2CPacket {
    pub a: u32,
}
