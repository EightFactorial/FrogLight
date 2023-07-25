use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundUpdateSelectedSlotS2CPacket {
    pub a: u8,
}
