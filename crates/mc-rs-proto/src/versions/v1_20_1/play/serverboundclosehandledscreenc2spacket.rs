use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundCloseHandledScreenC2SPacket {
    pub a: u8,
}
