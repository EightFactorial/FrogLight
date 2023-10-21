use mc_rs_macros::Transcode;

use crate::types::packets::objective::ObjectiveUpdate;

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [4, 84, 101, 115, 116, 1])]
pub struct ClientboundScoreboardObjectiveUpdatePacket {
    pub name: String,
    pub method: ObjectiveUpdate,
}
