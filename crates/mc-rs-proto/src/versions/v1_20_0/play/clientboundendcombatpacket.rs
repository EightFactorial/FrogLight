use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEndCombatPacket {
    #[var]
    pub duration: u32,
}
