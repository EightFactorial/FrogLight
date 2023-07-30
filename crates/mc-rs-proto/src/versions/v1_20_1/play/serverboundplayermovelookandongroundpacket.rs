use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundPlayerMoveLookAndOnGroundPacket {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}
