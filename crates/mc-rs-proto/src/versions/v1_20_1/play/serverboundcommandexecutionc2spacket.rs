use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundCommandExecutionC2SPacket {
    pub a: String,
    pub b: u64,
    pub c: u64,
}
