use mc_rs_macros::Transcode;

use crate::types::packets::game_event::GameEventType;

#[derive(Debug, Clone, Copy, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [3, 0, 0, 0, 0])]
pub struct ClientboundGameStateChangePacket {
    pub event_type: GameEventType,
    pub parameter: f32,
}
