use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityTrackerUpdatePacket {
    pub a: u32,
}
