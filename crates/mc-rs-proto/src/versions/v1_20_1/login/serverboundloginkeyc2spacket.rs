use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundLoginKeyC2SPacket {
    pub key: Vec<u8>,
    pub challenge: Vec<u8>,
}
