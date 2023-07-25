use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundClearTitleS2CPacket {
    pub a: bool,
}
