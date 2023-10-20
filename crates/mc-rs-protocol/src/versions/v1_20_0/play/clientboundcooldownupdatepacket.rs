use mc_rs_macros::Transcode;

use crate::types::ResourceLocation;

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0])]
pub struct ClientboundCooldownUpdatePacket {
    pub item: ResourceLocation,
    #[var]
    pub duration: u32,
}
