use crate::types::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LookAnchor {
    Feet,
    Eyes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LookAtEntity {
    pub entity_id: EntityId,
    pub anchor: LookAnchor,
}
