use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundCloseHandledScreenPacket {
    pub a: u8,
}
