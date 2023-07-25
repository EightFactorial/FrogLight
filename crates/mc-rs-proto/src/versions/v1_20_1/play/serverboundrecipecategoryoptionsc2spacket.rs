use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundRecipeCategoryOptionsC2SPacket {
    pub a: Enum,
    pub b: bool,
    pub c: bool,
}
