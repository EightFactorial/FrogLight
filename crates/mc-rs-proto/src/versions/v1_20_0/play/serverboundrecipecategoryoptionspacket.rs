use mc_rs_macros::Transcode;

use crate::types::packets::recipe::RecipeBook;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundRecipeCategoryOptionsPacket {
    pub book: RecipeBook,
    pub open: bool,
    pub filter: bool,
}
