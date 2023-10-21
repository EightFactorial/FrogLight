use mc_rs_macros::Transcode;

use crate::types::ResourceLocation;

#[derive(Debug, Clone, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 21, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 103, 101, 110, 101, 114, 105, 99, 95, 51, 120, 51, 5, 84, 105, 116, 108, 101])]
pub struct ClientboundOpenScreenPacket {
    #[var]
    pub screen_id: u32,
    pub kind: ResourceLocation,
    // TODO: This is actually a FormattedText
    pub title: String,
}
