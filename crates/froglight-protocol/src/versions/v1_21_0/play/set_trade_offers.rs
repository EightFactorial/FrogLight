use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct SetTradeOffersPacket {
    #[frog(var)]
    pub container_id: u32,
    // TODO: Implement TradeOffers
    pub trade_offers: UnsizedBuffer,
}
