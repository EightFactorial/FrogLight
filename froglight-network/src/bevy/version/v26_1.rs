//! [`NetworkVersion`] implementation for [`V26_1`].

use bevy_ecs::world::EntityRef;
use froglight_common::version::V26_1;
use froglight_packet::{
    generated::v26_1::{
        handshake::{HandshakeC2SPacket, ServerboundPackets as HandshakeServerboundPackets},
        login::{
            ClientboundPackets as LoginClientboundPackets, LoginHelloC2SPacket,
            ServerboundPackets as LoginServerboundPackets,
        },
    },
    version::{Clientbound, Serverbound, VersionPacket},
};

use super::ConnectionUpdate;
use crate::{
    bevy::NetworkVersion,
    connection::ConnectionError,
    event::{ClientboundLoginEvent, ServerboundHandshakeEvent, ServerboundLoginEvent},
    prelude::*,
};

impl NetworkVersion for V26_1 {
    fn update_connection_details(
        packet: &VersionPacket<Self, Clientbound>,
    ) -> Option<ConnectionUpdate> {
        match packet {
            VersionPacket::Login(LoginClientboundPackets::LoginCompression(p)) => {
                Some(ConnectionUpdate {
                    compression_threshold: Some(p.compression_threshold),
                    encrypion_key: None,
                })
            }
            VersionPacket::Login(LoginClientboundPackets::LoginHello(p)) => {
                Some(ConnectionUpdate {
                    compression_threshold: None,
                    encrypion_key: Some(p.public_key.clone()),
                })
            }
            _ => None,
        }
    }

    fn event_to_packet(
        event: ServerboundEventEnum,
        _entity: EntityRef<'_>,
    ) -> Result<VersionPacket<Self, Serverbound>, ConnectionError> {
        match event {
            ServerboundEventEnum::Handshake(handshake) => {
                let ServerboundHandshakeEvent::Handshake(c) = handshake;
                let packet = HandshakeC2SPacket::new(c);
                Ok(VersionPacket::Handshake(HandshakeServerboundPackets::Handshake(packet)))
            }

            ServerboundEventEnum::Status(_status) => {
                todo!()
            }

            ServerboundEventEnum::Login(login) => match login {
                ServerboundLoginEvent::Hello(c) => {
                    let packet =
                        LoginHelloC2SPacket { name: c.username, uuid: c.uuid.into_bytes() };
                    Ok(VersionPacket::Login(LoginServerboundPackets::LoginHello(packet)))
                }
            },

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

            VersionPacket::Login(login) => match login {
                LoginClientboundPackets::LoginDisconnect(disconnect) => {
                    Ok(ClientboundEventEnum::Login(ClientboundLoginEvent::Disconnect(
                        disconnect.reason,
                    )))
                }
                _ => todo!(),
            },

            VersionPacket::Config(_config) => {
                todo!()
            }

            VersionPacket::Play(_play) => {
                todo!()
            }
        }
    }
}
