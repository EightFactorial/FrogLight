use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundPickFromInventoryPacket {
    #[var]
    pub slot_id: u32,
}
