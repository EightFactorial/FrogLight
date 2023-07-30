use crate::types::{packets::advancement::AdvancementTabAction, ResourceLocation};
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundAdvancementTabPacket {
    pub action: AdvancementTabAction,
    pub tab: Option<ResourceLocation>,
}
