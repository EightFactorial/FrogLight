use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundDeathMessagePacket {
    #[var]
    pub player_id: u32,
    pub message: String,
}
