use mc_rs_macros::Transcode;

use crate::types::{
    packets::look_at::{LookAnchor, LookAtEntity},
    Vec3,
};

#[derive(Debug, Clone, Copy, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 32, 1])]
pub struct ClientboundLookAtPacket {
    pub anchor: LookAnchor,
    pub position: Vec3,
    pub entity: Option<LookAtEntity>,
}
