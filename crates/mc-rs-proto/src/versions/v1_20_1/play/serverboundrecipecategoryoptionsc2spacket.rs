use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundRecipeCategoryOptionsC2SPacket {
    pub a: Enum,
    pub b: bool,
    pub c: bool,
}
