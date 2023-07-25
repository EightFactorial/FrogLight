use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundHandshakeC2SPacket {
    #[var]
    pub protocol_version: u32,
    pub hostname: String,
    pub port: u16,
    #[var]
    pub intention: u32,
}
