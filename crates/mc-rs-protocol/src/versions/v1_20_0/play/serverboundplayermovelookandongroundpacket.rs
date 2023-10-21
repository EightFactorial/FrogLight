use mc_rs_macros::Transcode;

#[derive(Debug, Default, Clone, Copy, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ServerboundPlayerMoveLookAndOnGroundPacket {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}
