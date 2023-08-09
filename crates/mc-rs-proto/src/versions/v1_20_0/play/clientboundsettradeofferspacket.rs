use mc_rs_macros::Transcode;

use crate::types::packets::merchant::MerchantOffer;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundSetTradeOffersPacket {
    #[var]
    pub container_id: u32,
    pub offers: Vec<MerchantOffer>,
    #[var]
    pub merchant_level: u32,
    #[var]
    pub merchant_xp: u32,
    pub show_progress: bool,
    pub can_restock: bool,
}
