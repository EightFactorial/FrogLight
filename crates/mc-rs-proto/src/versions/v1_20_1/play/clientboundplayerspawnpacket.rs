use mc_rs_macros::Transcode;
use uuid::Uuid;

use crate::types::{
    position::{AngleData, Vec3},
    EntityId,
};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayerSpawnPacket {
    pub entity_id: EntityId,
    pub uuid: Uuid,
    pub position: Vec3,
    pub angle: AngleData,
}
