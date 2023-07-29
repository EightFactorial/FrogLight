use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
pub enum LookAnchor {
    Feet,
    Eyes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
pub struct LookAtEntity {
    pub entity_id: EntityId,
    pub anchor: LookAnchor,
}
