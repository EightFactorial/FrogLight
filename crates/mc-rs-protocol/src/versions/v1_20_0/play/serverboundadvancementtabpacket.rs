use crate::types::{packets::advancement::AdvancementTabAction, ResourceLocation};
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [1, 0])]
pub struct ServerboundAdvancementTabPacket {
    pub action: AdvancementTabAction,
    pub tab: Option<ResourceLocation>,
}
