use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundEntitiesDestroyS2CPacket {
    pub a: Vec<u32>,
}
