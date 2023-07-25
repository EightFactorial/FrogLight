use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundMessageAcknowledgmentC2SPacket {
    pub a: u32,
}
