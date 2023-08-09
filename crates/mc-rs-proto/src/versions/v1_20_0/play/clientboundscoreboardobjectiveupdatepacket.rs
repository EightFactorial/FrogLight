use mc_rs_macros::Transcode;

use crate::types::packets::objective::ObjectiveUpdate;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundScoreboardObjectiveUpdatePacket {
    pub name: String,
    pub method: ObjectiveUpdate,
}
