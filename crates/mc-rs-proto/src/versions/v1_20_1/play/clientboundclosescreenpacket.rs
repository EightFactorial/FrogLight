use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundCloseScreenPacket {
    pub a: u16,
}
