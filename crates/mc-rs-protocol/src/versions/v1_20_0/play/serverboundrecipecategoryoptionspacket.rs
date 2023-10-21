use mc_rs_macros::Transcode;

use crate::types::packets::recipe::RecipeBook;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0])]
pub struct ServerboundRecipeCategoryOptionsPacket {
    pub book: RecipeBook,
    pub open: bool,
    pub filter: bool,
}
