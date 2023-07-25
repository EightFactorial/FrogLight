use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundCommandSuggestionsS2CPacket {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: Vec,
}
