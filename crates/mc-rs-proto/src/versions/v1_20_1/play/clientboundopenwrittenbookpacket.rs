use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundOpenWrittenBookPacket {
    pub a: Enum,
}
