use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundOpenWrittenBookS2CPacket {
    pub a: Enum,
}
