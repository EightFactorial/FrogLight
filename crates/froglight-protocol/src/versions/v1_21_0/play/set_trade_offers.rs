use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct SetTradeOffersPacket {
    #[frog(var)]
    pub container_id: u32,
    // TODO: Implement TradeOffers
    pub trade_offers: UnsizedBuffer,
    // #[frog(var)]
    // pub field_1: u32,
    // #[frog(var)]
    // pub field_2: u32,
    // pub field_3: bool,
    // pub field_4: bool,
}
