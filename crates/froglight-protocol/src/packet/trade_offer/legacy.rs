use froglight_macros::FrogReadWrite;

use crate::packet::LegacyItemSlot;

/// A trade offer.
///
/// # Note
/// This is used in versions before `1.20.4 (TODO)`.
#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
pub struct LegacyTradeOffer {
    pub base_cost_a: LegacyItemSlot,
    pub result: LegacyItemSlot,
    pub cost_b: LegacyItemSlot,
    pub out_of_stock: bool,
    pub uses: u32,
    pub max_uses: u32,
    pub xp: u32,
    pub special_price_diff: i32,
    pub price_multiplier: f32,
    pub demand: u32,
}
