use froglight_protocol::{
    states::Play,
    versions::v1_21_0::{
        play::{PlayClientboundPackets, PlayServerboundPackets},
        V1_21_0,
    },
};

use super::PlayState;
use crate::connection::{ConnectionError, Serverbound, WriteConnection};

impl PlayState for V1_21_0 {
    async fn end_play(
        packet: &PlayClientboundPackets,
        _: &WriteConnection<Self, Play, Serverbound>,
    ) -> Result<bool, ConnectionError> {
        match packet {
            PlayClientboundPackets::Disconnect(packet) => {
                Err(ConnectionError::ServerError(serde_json::to_string(&packet.reason).unwrap()))
            }
            PlayClientboundPackets::EnterReconfiguration(..) => Ok(true),
            _ => Ok(false),
        }
    }

    fn play_acknowledged(packet: &PlayServerboundPackets) -> bool {
        matches!(packet, PlayServerboundPackets::AcknowledgeReconfiguration(..))
    }
}
