use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundSetTradeOffersS2CPacket {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: bool,
    pub e: bool,
}
