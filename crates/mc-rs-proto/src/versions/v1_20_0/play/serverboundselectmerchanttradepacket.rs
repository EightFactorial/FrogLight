use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundSelectMerchantTradePacket {
    #[var]
    pub item: u32,
}
