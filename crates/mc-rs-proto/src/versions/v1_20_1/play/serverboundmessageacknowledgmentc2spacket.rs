use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundMessageAcknowledgmentC2SPacket {
    pub a: u32,
}
