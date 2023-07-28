use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayerAbilitiesPacket {
    // TODO: Bitfield macro attribute
    pub flags: u8,
    pub flying_speed: f32,
    pub walking_speed: f32,
}
