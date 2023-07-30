use crate::types::ResourceLocation;
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundRecipeBookDataPacket {
    pub recipe: ResourceLocation,
}
