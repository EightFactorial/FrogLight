use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundPlayerInteractItemC2SPacket {
    pub a: Enum,
    pub b: u32,
}
