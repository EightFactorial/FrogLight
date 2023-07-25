use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundClientStatusC2SPacket {
    pub a: Enum,
}
