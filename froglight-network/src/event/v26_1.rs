//! [`EventVersion`] implementation for [`V26_1`].
#![expect(clippy::too_many_lines, reason = "Huge match statements for packet/event conversion")]
#![expect(unreachable_code, unused_variables, reason = "WIP")]

use froglight_common::version::V26_1;
use froglight_packet::{
    generated::v26_1::{
        config::{
            ClientboundPackets as ConfigClientboundPackets,
            ServerboundPackets as ConfigServerboundPackets,
        },
        handshake::{HandshakeC2SPacket, ServerboundPackets as HandshakeServerboundPackets},
        login::{
            ClientboundPackets as LoginClientboundPackets, EnterConfigurationC2SPacket,
            LoginDisconnectS2CPacket, LoginHelloC2SPacket, LoginSuccessS2CPacket,
            ServerboundPackets as LoginServerboundPackets,
        },
    },
    version::{Clientbound, Serverbound, VersionPacket},
};

use super::enums::ServerboundConfigEvent;
use crate::{
    connection::ConnectionError,
    event::{
        EventVersion,
        enums::{
            ClientboundConfigEvent, ClientboundLoginEvent, ServerboundHandshakeEvent,
            ServerboundLoginEvent,
        },
    },
    prelude::*,
};

impl EventVersion for V26_1 {
    fn client_event_to_packet(
        event: ClientboundEventEnum,
    ) -> Result<Option<VersionPacket<Self, Clientbound>>, ConnectionError> {
        match event {
            ClientboundEventEnum::Status(_status) => todo!(),

            ClientboundEventEnum::Login(login) => match login {
                ClientboundLoginEvent::Disconnect(event) => {
                    let packet = LoginDisconnectS2CPacket::new(event);
                    Ok(Some(VersionPacket::Login(LoginClientboundPackets::LoginDisconnect(packet))))
                }
                ClientboundLoginEvent::EncryptionRequest() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Login(LoginClientboundPackets::LoginHello(packet))))
                }
                ClientboundLoginEvent::QueryRequest() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Login(LoginClientboundPackets::LoginQueryRequest(
                        packet,
                    ))))
                }
                ClientboundLoginEvent::CookieRequest() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Login(LoginClientboundPackets::CookieRequest(packet))))
                }
                ClientboundLoginEvent::Profile(event) => {
                    let packet = LoginSuccessS2CPacket::new(event);
                    Ok(Some(VersionPacket::Login(LoginClientboundPackets::LoginSuccess(packet))))
                }
            },

            ClientboundEventEnum::Config(config) => match config {
                ClientboundConfigEvent::Disconnect(_) => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Placeholder)))
                }
                ClientboundConfigEvent::TransferServer() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Placeholder)))
                }
                ClientboundConfigEvent::KeepAlive() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Placeholder)))
                }
                ClientboundConfigEvent::Ping() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Placeholder)))
                }
                ClientboundConfigEvent::ResetChat => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Placeholder)))
                }
                ClientboundConfigEvent::ResourcePackQuery() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Placeholder)))
                }
                ClientboundConfigEvent::ResourcePackPush() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Placeholder)))
                }
                ClientboundConfigEvent::ResourcePackPop() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Placeholder)))
                }
                ClientboundConfigEvent::UpdateRegistries() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Placeholder)))
                }
                ClientboundConfigEvent::UpdateFeatures() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Placeholder)))
                }
                ClientboundConfigEvent::UpdateTags() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Placeholder)))
                }
                ClientboundConfigEvent::ServerLinks() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Placeholder)))
                }
                ClientboundConfigEvent::CodeOfConduct() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Placeholder)))
                }
                ClientboundConfigEvent::ReportDetails() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Placeholder)))
                }
                ClientboundConfigEvent::QueryRequest() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Placeholder)))
                }
                ClientboundConfigEvent::CookieRequest() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Placeholder)))
                }
                ClientboundConfigEvent::CookieStore() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Placeholder)))
                }
                ClientboundConfigEvent::ShowDialog() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Placeholder)))
                }
                ClientboundConfigEvent::ClearDialog() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Placeholder)))
                }
                ClientboundConfigEvent::FinishConfig => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Placeholder)))
                }
            },

            ClientboundEventEnum::Play(_play) => todo!(),
        }
    }

    fn client_packet_to_event(
        packet: VersionPacket<Self, Clientbound>,
    ) -> Result<Option<ClientboundEventEnum>, ConnectionError> {
        match packet {
            // Technically unreachable as there are no clientbound handshake packets
            VersionPacket::Handshake(_) => Ok(None),

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
                LoginClientboundPackets::LoginSuccess(packet) => Ok(Some(
                    ClientboundEventEnum::Login(ClientboundLoginEvent::Profile(packet.profile)),
                )),
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

    fn server_event_to_packet(
        event: ServerboundEventEnum,
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
                    let packet = LoginHelloC2SPacket::new(event);
                    Ok(Some(VersionPacket::Login(LoginServerboundPackets::LoginHello(packet))))
                }
                ServerboundLoginEvent::EncryptionResponse() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Login(LoginServerboundPackets::LoginKey(packet))))
                }
                ServerboundLoginEvent::QueryResponse() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Login(LoginServerboundPackets::LoginQueryResponse(
                        packet,
                    ))))
                }
                ServerboundLoginEvent::Cookieresponse() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Login(LoginServerboundPackets::CookieResponse(packet))))
                }
                ServerboundLoginEvent::AcknowledgeLogin => {
                    let packet = EnterConfigurationC2SPacket;
                    Ok(Some(VersionPacket::Login(LoginServerboundPackets::EnterConfiguration(
                        packet,
                    ))))
                }
            },

            ServerboundEventEnum::Config(config) => match config {
                ServerboundConfigEvent::ClientInformation() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::Placeholder)))
                }
                ServerboundConfigEvent::KeepAlive() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::Placeholder)))
                }
                ServerboundConfigEvent::Pong() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::Placeholder)))
                }
                ServerboundConfigEvent::ResourcePackResponse() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::Placeholder)))
                }
                ServerboundConfigEvent::ResourcePackUpdate() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::Placeholder)))
                }
                ServerboundConfigEvent::AcceptCodeOfConduct => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::Placeholder)))
                }
                ServerboundConfigEvent::QueryResponse() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::Placeholder)))
                }
                ServerboundConfigEvent::CookieResponse() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::Placeholder)))
                }
                ServerboundConfigEvent::DialogAction() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::Placeholder)))
                }
                ServerboundConfigEvent::AcknowledgeConfig => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::Placeholder)))
                }
            },

            ServerboundEventEnum::Play(_play) => {
                todo!()
            }
        }
    }

    fn server_packet_to_event(
        packet: VersionPacket<Self, Serverbound>,
    ) -> Result<Option<ServerboundEventEnum>, ConnectionError> {
        match packet {
            VersionPacket::Handshake(handshake) => match handshake {
                HandshakeServerboundPackets::Handshake(packet) => Ok(Some(
                    ServerboundEventEnum::Handshake(ServerboundHandshakeEvent::Handshake(packet.0)),
                )),
            },

            VersionPacket::Status(_) => todo!(),

            VersionPacket::Login(login) => match login {
                LoginServerboundPackets::LoginHello(packet) => {
                    Ok(Some(ServerboundEventEnum::Login(ServerboundLoginEvent::Hello(packet.0))))
                }
                LoginServerboundPackets::LoginKey(_packet) => Ok(Some(
                    ServerboundEventEnum::Login(ServerboundLoginEvent::EncryptionResponse()),
                )),
                LoginServerboundPackets::LoginQueryResponse(_packet) => {
                    Ok(Some(ServerboundEventEnum::Login(ServerboundLoginEvent::QueryResponse())))
                }
                LoginServerboundPackets::EnterConfiguration(_) => {
                    Ok(Some(ServerboundEventEnum::Login(ServerboundLoginEvent::AcknowledgeLogin)))
                }
                LoginServerboundPackets::CookieResponse(_packet) => {
                    Ok(Some(ServerboundEventEnum::Login(ServerboundLoginEvent::Cookieresponse())))
                }
            },

            VersionPacket::Config(_) => todo!(),

            VersionPacket::Play(_) => todo!(),
        }
    }
}
