use compact_str::CompactString;
use froglight_macros::FrogReadWrite;
use uuid::Uuid;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ResourcePackSendS2CPacket {
    pub id: Uuid,
    pub url: CompactString,
    pub hash: CompactString,
    pub required: bool,
    // TODO: FormattedText
    pub prompt: Option<String>,
}
