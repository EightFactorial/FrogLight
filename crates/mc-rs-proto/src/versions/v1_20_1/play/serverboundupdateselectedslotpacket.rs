use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundUpdateSelectedSlotPacket {
    pub a: u16,
}
