use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0])]
pub enum LookAnchor {
    Feet,
    Eyes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [24, 1])]
pub struct LookAtEntity {
    pub entity_id: EntityId,
    pub anchor: LookAnchor,
}
