use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundCommandSuggestionsS2CPacket {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: Vec,
}
