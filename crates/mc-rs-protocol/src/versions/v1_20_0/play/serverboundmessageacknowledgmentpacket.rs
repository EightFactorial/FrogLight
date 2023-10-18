use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundMessageAcknowledgmentPacket {
    #[var]
    pub message: u32,
}
