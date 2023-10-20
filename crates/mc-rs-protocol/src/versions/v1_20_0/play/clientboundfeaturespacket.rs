use mc_rs_macros::Transcode;

use crate::types::ResourceLocation;

#[derive(Debug, Default, Clone, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0])]
pub struct ClientboundFeaturesPacket {
    pub features: Vec<ResourceLocation>,
}
