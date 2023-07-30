use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundCloseHandledScreenPacket {
    pub container_id: u8,
}
