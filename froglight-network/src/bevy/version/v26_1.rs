//! [`NetworkVersion`] implementation for [`V26_1`].

use bevy_ecs::world::EntityRef;
use froglight_common::version::V26_1;
use froglight_packet::{
    generated::v26_1::{
        handshake::{HandshakeC2SPacket, ServerboundPackets as HandshakeServerboundPackets},
        login::ClientboundPackets as LoginClientboundPackets,
    },
    version::{Clientbound, Serverbound, VersionPacket},
};

use super::ConnectionUpdate;
use crate::{
    bevy::NetworkVersion, connection::ConnectionError, event::ServerboundHandshakeEvent, prelude::*,
};

impl NetworkVersion for V26_1 {
    fn update_connection_details(
        packet: &VersionPacket<Self, Clientbound>,
    ) -> Option<ConnectionUpdate> {
        match packet {
            VersionPacket::Login(LoginClientboundPackets::Placeholder) => todo!(),
            _ => None,
        }
    }

    fn event_to_packet(
        event: ServerboundEventEnum,
        _entity: EntityRef<'_>,
    ) -> Result<VersionPacket<Self, Serverbound>, ConnectionError> {
        match event {
            ServerboundEventEnum::Handshake(handshake) => {
                let ServerboundHandshakeEvent::Handshake(content) = handshake;

                Ok(VersionPacket::Handshake(HandshakeServerboundPackets::Handshake(
                    HandshakeC2SPacket::new(content),
                )))
            }

            ServerboundEventEnum::Status(_status) => {
                todo!()
            }

            ServerboundEventEnum::Login(_login) => {
                todo!()
            }

            ServerboundEventEnum::Config(_config) => {
                todo!()
            }

            ServerboundEventEnum::Play(_play) => {
                todo!()
            }
        }
    }

    fn packet_to_event(
        packet: VersionPacket<Self, Clientbound>,
        _entity: EntityRef<'_>,
    ) -> Result<ClientboundEventEnum, ConnectionError> {
        match packet {
            VersionPacket::Status(_status) => {
                todo!()
            }

            VersionPacket::Login(_login) => {
                todo!()
            }

            VersionPacket::Config(_config) => {
                todo!()
            }

            VersionPacket::Play(_play) => {
                todo!()
            }
        }
    }
}
