use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundClientStatusC2SPacket {
    pub a: Enum,
}
