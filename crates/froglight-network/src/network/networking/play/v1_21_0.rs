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
    async fn play_state_handle(
        packet: &PlayClientboundPackets,
        _: &WriteConnection<Self, Play, Serverbound>,
    ) -> Result<bool, ConnectionError> {
        match packet {
            PlayClientboundPackets::Disconnect(packet) => {
                Err(ConnectionError::ServerError(if let Some(reason) = packet.reason.string() {
                    reason.to_string()
                } else {
                    format!("{:?}", packet.reason)
                }))
            }
            PlayClientboundPackets::EnterReconfiguration(..) => Ok(true),
            _ => Ok(false),
        }
    }

    fn play_ack_handle(packet: &PlayServerboundPackets) -> bool {
        matches!(packet, PlayServerboundPackets::AcknowledgeReconfiguration(..))
    }
}
