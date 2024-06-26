use froglight_protocol::{
    states::Configuration,
    versions::v1_21_0::{
        configuration::{ConfigurationClientboundPackets, ConfigurationServerboundPackets},
        V1_21_0,
    },
};

use super::ConfigurationState;
use crate::connection::{ConnectionError, Serverbound, WriteConnection};

impl ConfigurationState for V1_21_0 {
    async fn config_state_handle<'a, 'b>(
        packet: &'a ConfigurationClientboundPackets,
        _: &'b WriteConnection<Self, Configuration, Serverbound>,
    ) -> Result<bool, ConnectionError> {
        match packet {
            ConfigurationClientboundPackets::Disconnect(packet) => {
                Err(ConnectionError::ServerError(serde_json::to_string(&packet.reason).unwrap()))
            }
            ConfigurationClientboundPackets::Ready(..) => Ok(true),
            _ => Ok(false),
        }
    }

    fn config_ack_handle(packet: &ConfigurationServerboundPackets) -> bool {
        matches!(packet, ConfigurationServerboundPackets::Ready(..))
    }
}
