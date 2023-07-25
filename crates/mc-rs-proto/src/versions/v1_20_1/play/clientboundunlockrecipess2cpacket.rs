use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundUnlockRecipesS2CPacket {
    pub a: Enum,
    pub b: Vec,
    pub c: Vec,
}
