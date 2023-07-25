use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundResourcePackStatusC2SPacket {
    pub a: Enum,
}
