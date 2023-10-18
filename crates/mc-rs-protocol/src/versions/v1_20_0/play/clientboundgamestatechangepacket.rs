use mc_rs_macros::Transcode;

use crate::types::packets::game_event::GameEventType;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundGameStateChangePacket {
    pub event_type: GameEventType,
    pub parameter: f32,
}
