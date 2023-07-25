use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundCloseHandledScreenC2SPacket {
    pub a: u8,
}
