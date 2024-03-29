use froglight_macros::FrogReadWrite;

use crate::common::LegacyTradeOffer;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [4, 0, 0, 0, 1, 1])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct SetTradeOffersS2CPacket {
    #[frog(var)]
    pub container_id: u32,
    pub offers: Vec<LegacyTradeOffer>,
    #[frog(var)]
    pub level_progress: u32,
    #[frog(var)]
    pub experience: u32,
    pub leveled: bool,
    pub refreshable: bool,
}
