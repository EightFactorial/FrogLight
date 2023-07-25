use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundRenameItemC2SPacket {
    pub a: String,
}
