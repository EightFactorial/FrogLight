//! [`NetworkVersion`] implementation for [`V26_1`].

use bevy_ecs::world::EntityRef;
use froglight_common::version::V26_1;
use froglight_packet::{
    generated::v26_1::{
        handshake::{HandshakeC2SPacket, ServerboundPackets as HandshakeServerboundPackets},
        login::{
            ClientboundPackets as LoginClientboundPackets, EnterConfigurationC2SPacket,
            LoginHelloC2SPacket, ServerboundPackets as LoginServerboundPackets,
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
    ) -> Result<Option<VersionPacket<Self, Serverbound>>, ConnectionError> {
        match event {
            ServerboundEventEnum::Handshake(handshake) => {
                let ServerboundHandshakeEvent::Handshake(event) = handshake;
                let packet = HandshakeC2SPacket::new(event);
                Ok(Some(VersionPacket::Handshake(HandshakeServerboundPackets::Handshake(packet))))
            }

            ServerboundEventEnum::Status(_status) => {
                todo!()
            }

            ServerboundEventEnum::Login(login) => match login {
                ServerboundLoginEvent::Hello(event) => {
                    let packet = LoginHelloC2SPacket { name: event.username, uuid: event.uuid };
                    Ok(Some(VersionPacket::Login(LoginServerboundPackets::LoginHello(packet))))
                }
                ServerboundLoginEvent::EncryptionResponse() => todo!(),
                ServerboundLoginEvent::QueryResponse() => todo!(),
                ServerboundLoginEvent::Cookieresponse() => todo!(),
                ServerboundLoginEvent::AcknowledgeLogin => Ok(Some(VersionPacket::Login(
                    LoginServerboundPackets::EnterConfiguration(EnterConfigurationC2SPacket),
                ))),
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
    ) -> Result<Option<ClientboundEventEnum>, ConnectionError> {
        match packet {
            VersionPacket::Status(_status) => {
                todo!()
            }

            VersionPacket::Login(login) => match login {
                LoginClientboundPackets::LoginDisconnect(packet) => Ok(Some(
                    ClientboundEventEnum::Login(ClientboundLoginEvent::Disconnect(packet.reason)),
                )),
                LoginClientboundPackets::LoginHello(_packet) => Ok(Some(
                    ClientboundEventEnum::Login(ClientboundLoginEvent::EncryptionRequest()),
                )),
                #[expect(unreachable_code, reason = "WIP")]
                LoginClientboundPackets::LoginSuccess(_packet) => {
                    Ok(Some(ClientboundEventEnum::Login(ClientboundLoginEvent::Profile(todo!()))))
                }
                LoginClientboundPackets::LoginQueryRequest(_packet) => {
                    Ok(Some(ClientboundEventEnum::Login(ClientboundLoginEvent::QueryRequest())))
                }
                LoginClientboundPackets::CookieRequest(_packet) => {
                    Ok(Some(ClientboundEventEnum::Login(ClientboundLoginEvent::CookieRequest())))
                }

                LoginClientboundPackets::LoginCompression(_) => Ok(None),
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
