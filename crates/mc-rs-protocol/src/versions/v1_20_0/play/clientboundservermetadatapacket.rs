use compact_str::CompactString;
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 1])]
pub struct ClientboundServerMetadataPacket {
    pub motd: CompactString,
    pub icon: Option<Vec<u8>>,
    pub enforce_secure_chat: bool,
}
