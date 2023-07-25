use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundUpdateSelectedSlotPacket {
    pub a: u8,
}
