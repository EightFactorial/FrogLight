use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundCloseScreenS2CPacket {
    pub a: u16,
}
