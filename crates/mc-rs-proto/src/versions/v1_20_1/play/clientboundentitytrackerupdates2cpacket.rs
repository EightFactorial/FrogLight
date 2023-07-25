use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityTrackerUpdateS2CPacket {
    pub a: u32,
}
