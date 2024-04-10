use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 1])]
pub struct ServerMetadataS2CPacket {
    pub message: CompactString,
    pub icon: Option<Vec<u8>>,
    pub enforce_secure_chat: bool,
}
