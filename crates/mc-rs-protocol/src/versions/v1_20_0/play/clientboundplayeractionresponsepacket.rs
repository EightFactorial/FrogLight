use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayerActionResponsePacket {
    #[var]
    pub sequence_id: i32,
}
