use mc_rs_macros::Transcode;

use crate::types::{
    packets::look_at::{LookAnchor, LookAtEntity},
    Vec3,
};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundLookAtPacket {
    pub anchor: LookAnchor,
    pub position: Vec3,
    pub entity: Option<LookAtEntity>,
}
