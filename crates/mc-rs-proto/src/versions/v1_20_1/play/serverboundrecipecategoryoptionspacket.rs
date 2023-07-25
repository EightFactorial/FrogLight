use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundRecipeCategoryOptionsPacket {
    pub a: Enum,
    pub b: bool,
    pub c: bool,
}
