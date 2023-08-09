use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundOpenWrittenBookPacket {
    // TODO: Hand enum
    pub hand: u8,
}
