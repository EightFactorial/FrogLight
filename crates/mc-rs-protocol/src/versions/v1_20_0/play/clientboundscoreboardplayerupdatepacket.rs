use mc_rs_macros::Transcode;

use crate::types::packets::scoreboard::ScoreboardUpdate;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundScoreboardPlayerUpdatePacket {
    pub entity_name: String,
    pub update_method: ScoreboardUpdate,
    pub objective_name: Option<String>,
}
