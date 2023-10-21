use crate::types::ResourceLocation;
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 10, 109, 99, 45, 114, 115, 58, 116, 101, 115, 116, 1])]
pub struct ServerboundCraftRequestPacket {
    pub container_id: u8,
    pub recipe: ResourceLocation,
    pub shift: bool,
}
