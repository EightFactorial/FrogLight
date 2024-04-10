use froglight_macros::FrogReadWrite;

use crate::packet::ItemSlot;

/// A trade offer.
#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
pub struct TradeOffer {
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
