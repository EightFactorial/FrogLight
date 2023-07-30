use bevy_math::IVec2;
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundWorldBorderCenterChangedPacket {
    pub center: IVec2,
}
