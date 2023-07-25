use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundResourcePackStatusC2SPacket {
    pub a: Enum,
}
