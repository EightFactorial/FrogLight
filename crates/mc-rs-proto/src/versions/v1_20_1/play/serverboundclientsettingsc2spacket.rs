use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundClientSettingsC2SPacket {
    pub a: String,
    pub b: u8,
    pub c: Enum,
    pub d: bool,
    pub e: u16,
    pub f: Enum,
    pub g: bool,
    pub h: bool,
}
