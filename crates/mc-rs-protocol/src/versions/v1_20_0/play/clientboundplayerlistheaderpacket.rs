use azalea_chat::FormattedText;
use mc_rs_macros::Transcode;

// TODO: Create a test for this packet
#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct ClientboundPlayerListHeaderPacket {
    pub header: FormattedText,
    pub footer: FormattedText,
}
