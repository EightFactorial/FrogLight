use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundCloseScreenPacket {
    pub container_id: u8,
}
