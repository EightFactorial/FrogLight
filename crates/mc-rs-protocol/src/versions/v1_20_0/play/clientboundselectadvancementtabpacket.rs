use mc_rs_macros::Transcode;

use crate::types::ResourceLocation;

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0])]
pub struct ClientboundSelectAdvancementTabPacket {
    pub tab: Option<ResourceLocation>,
}
