//! [`EventVersion`] implementation for [`V26_1`].
#![expect(clippy::too_many_lines, reason = "Huge match statements for packet/event conversion")]
#![expect(unreachable_code, unused_variables, reason = "WIP")]

use froglight_common::version::V26_1;
use froglight_packet::{
    generated::v26_1::{
        config::{
            ClearDialogS2CPacket, ClientboundPackets as ConfigClientboundPackets,
            ConfigDisconnectS2CPacket, FinishConfigurationS2CPacket, KeepAliveC2SPacket,
            KeepAliveS2CPacket, PingS2CPacket, PongC2SPacket,
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
                ClientboundConfigEvent::Disconnect(reason) => Ok(Some(VersionPacket::Config(
                    ConfigClientboundPackets::Disconnect(ConfigDisconnectS2CPacket { reason }),
                ))),
                ClientboundConfigEvent::TransferServer() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Transfer(packet))))
                }
                ClientboundConfigEvent::KeepAlive(packet_id) => {
                    let packet = KeepAliveS2CPacket { packet_id };
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::KeepAlive(packet))))
                }
                ClientboundConfigEvent::Ping(packet_id) => {
                    let packet = PingS2CPacket { packet_id };
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Ping(packet))))
                }
                ClientboundConfigEvent::ResetChat => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::ResetChat(packet))))
                }
                ClientboundConfigEvent::ResourcePackQuery() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::SelectKnownPacks(
                        packet,
                    ))))
                }
                ClientboundConfigEvent::ResourcePackPush() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::ResourcePackPush(
                        packet,
                    ))))
                }
                ClientboundConfigEvent::ResourcePackPop() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::ResourcePackPop(
                        packet,
                    ))))
                }
                ClientboundConfigEvent::UpdateRegistries() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::RegistryData(packet))))
                }
                ClientboundConfigEvent::UpdateFeatures() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(
                        ConfigClientboundPackets::UpdateEnabledFeatures(packet),
                    )))
                }
                ClientboundConfigEvent::UpdateTags() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::UpdateTags(packet))))
                }
                ClientboundConfigEvent::ServerLinks() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::ServerLinks(packet))))
                }
                ClientboundConfigEvent::CodeOfConduct() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::CodeOfConduct(packet))))
                }
                ClientboundConfigEvent::ReportDetails() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::CustomReportDetails(
                        packet,
                    ))))
                }
                ClientboundConfigEvent::QueryRequest() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::QueryRequest(packet))))
                }
                ClientboundConfigEvent::CookieRequest() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::CookieRequest(packet))))
                }
                ClientboundConfigEvent::CookieStore() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::StoreCookie(packet))))
                }
                ClientboundConfigEvent::ShowDialog() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::ShowDialog(packet))))
                }
                ClientboundConfigEvent::ClearDialog => {
                    let packet = ClearDialogS2CPacket;
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::ClearDialog(packet))))
                }
                ClientboundConfigEvent::FinishConfig => Ok(Some(VersionPacket::Config(
                    ConfigClientboundPackets::FinishConfiguration(FinishConfigurationS2CPacket),
                ))),
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

            VersionPacket::Config(config) => match config {
                ConfigClientboundPackets::CookieRequest(_packet) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::CookieRequest())))
                }
                ConfigClientboundPackets::QueryRequest(_packet) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::QueryRequest())))
                }
                ConfigClientboundPackets::Disconnect(packet) => Ok(Some(
                    ClientboundEventEnum::Config(ClientboundConfigEvent::Disconnect(packet.reason)),
                )),
                ConfigClientboundPackets::FinishConfiguration(_) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::FinishConfig)))
                }
                ConfigClientboundPackets::KeepAlive(packet) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::KeepAlive(
                        packet.packet_id,
                    ))))
                }
                ConfigClientboundPackets::Ping(packet) => Ok(Some(ClientboundEventEnum::Config(
                    ClientboundConfigEvent::Ping(packet.packet_id),
                ))),
                ConfigClientboundPackets::ResetChat(_) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::ResetChat)))
                }
                ConfigClientboundPackets::RegistryData(_packet) => Ok(Some(
                    ClientboundEventEnum::Config(ClientboundConfigEvent::UpdateRegistries()),
                )),
                ConfigClientboundPackets::ResourcePackPop(_packet) => Ok(Some(
                    ClientboundEventEnum::Config(ClientboundConfigEvent::ResourcePackPop()),
                )),
                ConfigClientboundPackets::ResourcePackPush(_packet) => Ok(Some(
                    ClientboundEventEnum::Config(ClientboundConfigEvent::ResourcePackPush()),
                )),
                ConfigClientboundPackets::StoreCookie(_packet) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::CookieStore())))
                }
                ConfigClientboundPackets::Transfer(_packet) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::TransferServer())))
                }
                ConfigClientboundPackets::UpdateEnabledFeatures(_packet) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::UpdateFeatures())))
                }
                ConfigClientboundPackets::UpdateTags(_packet) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::UpdateTags())))
                }
                ConfigClientboundPackets::SelectKnownPacks(_packet) => Ok(Some(
                    ClientboundEventEnum::Config(ClientboundConfigEvent::ResourcePackQuery()),
                )),
                ConfigClientboundPackets::CustomReportDetails(_packet) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::ReportDetails())))
                }
                ConfigClientboundPackets::ServerLinks(_packet) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::ServerLinks())))
                }
                ConfigClientboundPackets::ClearDialog(_) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::ClearDialog)))
                }
                ConfigClientboundPackets::ShowDialog(_packet) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::ShowDialog())))
                }
                ConfigClientboundPackets::CodeOfConduct(_packet) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::CodeOfConduct())))
                }
            },

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
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::ClientInformation(
                        packet,
                    ))))
                }
                ServerboundConfigEvent::KeepAlive(packet_id) => {
                    let packet = KeepAliveC2SPacket { packet_id };
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::KeepAlive(packet))))
                }
                ServerboundConfigEvent::Pong(packet_id) => {
                    let packet = PongC2SPacket { packet_id };
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::Pong(packet))))
                }
                ServerboundConfigEvent::ResourcePackResponse() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::SelectKnownPacks(
                        packet,
                    ))))
                }
                ServerboundConfigEvent::ResourcePackUpdate() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::ResourcePack(packet))))
                }
                ServerboundConfigEvent::AcceptCodeOfConduct => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::AcceptCodeOfConduct(
                        packet,
                    ))))
                }
                ServerboundConfigEvent::QueryResponse() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::QueryResponse(packet))))
                }
                ServerboundConfigEvent::CookieResponse() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::CookieResponse(
                        packet,
                    ))))
                }
                ServerboundConfigEvent::DialogAction() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::CustomClickAction(
                        packet,
                    ))))
                }
                ServerboundConfigEvent::AcknowledgeConfig => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::AcceptCodeOfConduct(
                        packet,
                    ))))
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

            VersionPacket::Config(config) => match config {
                ConfigServerboundPackets::ClientInformation(_packet) => Ok(Some(
                    ServerboundEventEnum::Config(ServerboundConfigEvent::ClientInformation()),
                )),
                ConfigServerboundPackets::CookieResponse(_packet) => {
                    Ok(Some(ServerboundEventEnum::Config(ServerboundConfigEvent::CookieResponse())))
                }
                ConfigServerboundPackets::QueryResponse(_packet) => {
                    Ok(Some(ServerboundEventEnum::Config(ServerboundConfigEvent::QueryResponse())))
                }
                ConfigServerboundPackets::FinishConfiguration(_) => Ok(Some(
                    ServerboundEventEnum::Config(ServerboundConfigEvent::AcknowledgeConfig),
                )),
                ConfigServerboundPackets::KeepAlive(packet) => {
                    Ok(Some(ServerboundEventEnum::Config(ServerboundConfigEvent::KeepAlive(
                        packet.packet_id,
                    ))))
                }
                ConfigServerboundPackets::Pong(packet) => Ok(Some(ServerboundEventEnum::Config(
                    ServerboundConfigEvent::Pong(packet.packet_id),
                ))),
                ConfigServerboundPackets::ResourcePack(_packet) => Ok(Some(
                    ServerboundEventEnum::Config(ServerboundConfigEvent::ResourcePackUpdate()),
                )),
                ConfigServerboundPackets::SelectKnownPacks(_packet) => Ok(Some(
                    ServerboundEventEnum::Config(ServerboundConfigEvent::ResourcePackResponse()),
                )),
                ConfigServerboundPackets::CustomClickAction(_packet) => {
                    Ok(Some(ServerboundEventEnum::Config(ServerboundConfigEvent::DialogAction())))
                }
                ConfigServerboundPackets::AcceptCodeOfConduct(_) => Ok(Some(
                    ServerboundEventEnum::Config(ServerboundConfigEvent::AcceptCodeOfConduct),
                )),
            },

            VersionPacket::Play(_) => todo!(),
        }
    }
}
