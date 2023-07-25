use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundUpdateSelectedSlotS2CPacket {
    pub a: u8,
}
