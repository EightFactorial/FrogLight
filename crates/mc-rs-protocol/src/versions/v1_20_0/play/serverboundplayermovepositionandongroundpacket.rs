use crate::types::Vec3;
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundPlayerMovePositionAndOnGroundPacket {
    pub position: Vec3,
    pub on_ground: bool,
}
