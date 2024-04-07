use compact_str::CompactString;
use froglight_macros::FrogReadWrite;
use serde_json::Value;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct ResourcePackSendS2CPacket {
    pub url: CompactString,
    pub hash: CompactString,
    pub required: bool,
    // TODO: FormattedText
    pub prompt: Option<Value>,
}
