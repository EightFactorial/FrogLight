use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundWorldTimeUpdatePacket {
    pub game_time: u64,
    pub day_time: u64,
}
