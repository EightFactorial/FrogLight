use mc_rs_macros::Transcode;
use crate::types::ResourceLocation;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundRecipeBookDataPacket {
    pub a: ResourceLocation,
}
