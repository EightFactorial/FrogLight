use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundMessageAcknowledgmentPacket {
    pub a: u32,
}
