use mc_rs_macros::Transcode;

use crate::types::packets::input::InputFlags;

#[derive(Debug, Default, Clone, Copy, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ServerboundPlayerInputPacket {
    pub h_vel: f32,
    pub f_vel: f32,
    pub flags: InputFlags,
}
