use bevy_math::Vec2;
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundWorldBorderCenterChangedPacket {
    pub center: Vec2,
}
