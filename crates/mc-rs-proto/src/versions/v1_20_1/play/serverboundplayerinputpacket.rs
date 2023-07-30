use mc_rs_macros::Transcode;

use crate::types::packets::input::InputFlags;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundPlayerInputPacket {
    pub h_vel: f32,
    pub f_vel: f32,
    pub flags: InputFlags,
}
