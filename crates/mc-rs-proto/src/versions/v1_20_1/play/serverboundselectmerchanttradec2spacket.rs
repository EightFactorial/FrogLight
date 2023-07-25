use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundSelectMerchantTradeC2SPacket {
    pub a: u32,
}
