use mc_rs_macros::Transcode;

use crate::types::inventory::ItemSlot;

// TODO: Create a test for this
#[derive(Clone, Debug, PartialEq, Transcode)]
pub struct MerchantOffer {
    pub base_cost_a: ItemSlot,
    pub result: ItemSlot,
    pub cost_b: ItemSlot,
    pub out_of_stock: bool,
    pub uses: u32,
    pub max_uses: u32,
    pub xp: u32,
    pub special_price_diff: i32,
    pub price_multiplier: f32,
    pub demand: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 1, 2])]
pub struct MerchantData {
    #[var]
    pub kind: u32,
    #[var]
    pub profession: u32,
    #[var]
    pub level: u32,
}
