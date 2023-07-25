use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundSynchronizeRecipesS2CPacket {
    pub a: Vec,
}
