//! [`NetworkVersion`] implementation for [`V26_1`].

use bevy_ecs::world::EntityRef;
use froglight_common::version::V26_1;
use froglight_packet::{
    generated::v26_1::login::ClientboundPackets as LoginClientboundPackets,
    version::{Clientbound, Serverbound, VersionPacket},
};

use super::ConnectionUpdate;
use crate::{bevy::NetworkVersion, connection::ConnectionError, event::EventVersion, prelude::*};

impl NetworkVersion for V26_1 {
    fn update_connection_details(
        packet: &VersionPacket<Self, Clientbound>,
    ) -> Option<ConnectionUpdate> {
        match packet {
            VersionPacket::Login(LoginClientboundPackets::LoginCompression(p)) => {
                Some(ConnectionUpdate {
                    compression_threshold: Some(p.compression_threshold),
                    ..ConnectionUpdate::default()
                })
            }
            VersionPacket::Login(LoginClientboundPackets::Hello(p)) => Some(ConnectionUpdate {
                encrypion_key: Some(p.public_key.clone()),
                ..ConnectionUpdate::default()
            }),
            _ => None,
        }
    }

    #[inline]
    fn event_to_packet(
        event: ServerboundEventEnum,
        _: EntityRef<'_>,
    ) -> Result<Option<VersionPacket<Self, Serverbound>>, ConnectionError> {
        <V26_1 as EventVersion>::server_event_to_packet(event)
    }

    #[inline]
    fn packet_to_event(
        packet: VersionPacket<Self, Clientbound>,
        _: EntityRef<'_>,
    ) -> Result<Option<ClientboundEventEnum>, ConnectionError> {
        <V26_1 as EventVersion>::client_packet_to_event(packet)
    }
}
