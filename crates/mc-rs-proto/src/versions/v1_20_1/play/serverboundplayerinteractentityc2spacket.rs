use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundPlayerInteractEntityC2SPacket {
    pub a: u32,
    pub b: Enum,
    pub c: bool,
}
