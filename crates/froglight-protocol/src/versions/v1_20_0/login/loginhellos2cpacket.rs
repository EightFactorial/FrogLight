use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct LoginHelloS2CPacket {
    pub server_id: CompactString,
    pub public_key: Vec<u8>,
    pub nonce: Vec<u8>,
}
