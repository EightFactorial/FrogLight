use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundLoginHelloPacket {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub key_nonce: Vec<u8>,
}