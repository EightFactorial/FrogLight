use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundHandSwingC2SPacket {
    pub a: Enum,
}
