//! [`EventVersion`] implementation for [`V26_1`].
#![expect(clippy::too_many_lines, reason = "Huge match statements for packet/event conversion")]
#![expect(unreachable_code, unused_variables, reason = "WIP")]

use froglight_common::version::V26_1;
use froglight_packet::{
    generated::v26_1::{
        configuration::{
            ClearDialogS2CPacket, ClientInformationC2SPacket,
            ClientboundPackets as ConfigClientboundPackets,
            CustomPayloadC2SPacket as ConfigCustomPayloadC2SPacket,
            CustomPayloadS2CPacket as ConfigCustomPayloadS2CPacket,
            DisconnectS2CPacket as ConfigDisconnectS2CPacket, FinishConfigurationC2SPacket,
            FinishConfigurationS2CPacket, KeepAliveC2SPacket, KeepAliveS2CPacket, PingS2CPacket,
            PongC2SPacket, SelectKnownPacksC2SPacket, SelectKnownPacksS2CPacket,
            ServerboundPackets as ConfigServerboundPackets,
        },
        handshake::{IntentionC2SPacket, ServerboundPackets as HandshakeServerboundPackets},
        login::{
            ClientboundPackets as LoginClientboundPackets, HelloC2SPacket,
            LoginAcknowledgedC2SPacket, LoginDisconnectS2CPacket, LoginFinishedS2CPacket,
            ServerboundPackets as LoginServerboundPackets,
        },
        // play::ClientboundPackets as PlayClientboundPackets,
    },
    version::{Clientbound, Serverbound, VersionPacket},
};

use super::enums::ServerboundConfigEvent;
use crate::{
    connection::ConnectionError,
    event::{
        EventVersion,
        enums::{
            ClientboundConfigEvent, ClientboundLoginEvent, ClientboundPlayEvent,
            ServerboundHandshakeEvent, ServerboundLoginEvent,
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
                    Ok(Some(VersionPacket::Login(LoginClientboundPackets::Hello(packet))))
                }
                ClientboundLoginEvent::QueryRequest() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Login(LoginClientboundPackets::CustomQuery(packet))))
                }
                ClientboundLoginEvent::CookieRequest() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Login(LoginClientboundPackets::CookieRequest(packet))))
                }
                ClientboundLoginEvent::Profile(event) => {
                    let packet = LoginFinishedS2CPacket::new(event);
                    Ok(Some(VersionPacket::Login(LoginClientboundPackets::LoginFinished(packet))))
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
                ClientboundConfigEvent::KeepAlive(id) => {
                    let packet = KeepAliveS2CPacket { id };
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::KeepAlive(packet))))
                }
                ClientboundConfigEvent::Ping(id) => {
                    let packet = PingS2CPacket { id };
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Ping(packet))))
                }
                ClientboundConfigEvent::ResetChat => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::ResetChat(packet))))
                }
                ClientboundConfigEvent::ResourcePackQuery(known) => {
                    let packet = SelectKnownPacksS2CPacket { known };
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
                ClientboundConfigEvent::CustomQuery(identifier, buffer) => {
                    let packet = ConfigCustomPayloadS2CPacket { identifier, buffer };
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::CustomPayload(packet))))
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
                    let packet = ClearDialogS2CPacket {};
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::ClearDialog(packet))))
                }
                ClientboundConfigEvent::FinishConfig => Ok(Some(VersionPacket::Config(
                    ConfigClientboundPackets::FinishConfiguration(FinishConfigurationS2CPacket {}),
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
                LoginClientboundPackets::Hello(_packet) => Ok(Some(ClientboundEventEnum::Login(
                    ClientboundLoginEvent::EncryptionRequest(),
                ))),
                LoginClientboundPackets::LoginFinished(packet) => Ok(Some(
                    ClientboundEventEnum::Login(ClientboundLoginEvent::Profile(packet.profile)),
                )),
                LoginClientboundPackets::CustomQuery(_packet) => {
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
                ConfigClientboundPackets::CustomPayload(packet) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::CustomQuery(
                        packet.identifier,
                        packet.buffer,
                    ))))
                }
                ConfigClientboundPackets::Disconnect(packet) => Ok(Some(
                    ClientboundEventEnum::Config(ClientboundConfigEvent::Disconnect(packet.reason)),
                )),
                ConfigClientboundPackets::FinishConfiguration(_) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::FinishConfig)))
                }
                ConfigClientboundPackets::KeepAlive(packet) => Ok(Some(
                    ClientboundEventEnum::Config(ClientboundConfigEvent::KeepAlive(packet.id)),
                )),
                ConfigClientboundPackets::Ping(packet) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::Ping(packet.id))))
                }
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
                ConfigClientboundPackets::SelectKnownPacks(packet) => {
                    Ok(Some(ClientboundEventEnum::Config(
                        ClientboundConfigEvent::ResourcePackQuery(packet.known),
                    )))
                }
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

            #[expect(clippy::match_single_binding, reason = "WIP")]
            VersionPacket::Play(play) => match play {
                // PlayClientboundPackets::BundleDelimiter(_packet) => todo!(),
                // PlayClientboundPackets::AddEntity(_packet) => todo!(),
                // PlayClientboundPackets::Animate(_packet) => todo!(),
                // PlayClientboundPackets::AwardStats(_packet) => todo!(),
                // PlayClientboundPackets::BlockChangedAck(_packet) => todo!(),
                // PlayClientboundPackets::BlockDestruction(_packet) => todo!(),
                // PlayClientboundPackets::BlockEntityData(_packet) => todo!(),
                // PlayClientboundPackets::BlockEvent(_packet) => todo!(),
                // PlayClientboundPackets::BlockUpdate(_packet) => todo!(),
                // PlayClientboundPackets::BossEvent(_packet) => todo!(),
                // PlayClientboundPackets::ChangeDifficulty(_packet) => todo!(),
                // PlayClientboundPackets::ChunkBatchFinished(_packet) => todo!(),
                // PlayClientboundPackets::ChunkBatchStart(_packet) => todo!(),
                // PlayClientboundPackets::ChunksBiomes(_packet) => todo!(),
                // PlayClientboundPackets::ClearTitles(_packet) => todo!(),
                // PlayClientboundPackets::CommandSuggestions(_packet) => todo!(),
                // PlayClientboundPackets::Commands(_packet) => todo!(),
                // PlayClientboundPackets::ContainerClose(_packet) => todo!(),
                // PlayClientboundPackets::ContainerSetContent(_packet) => todo!(),
                // PlayClientboundPackets::ContainerSetData(_packet) => todo!(),
                // PlayClientboundPackets::ContainerSetSlot(_packet) => todo!(),
                // PlayClientboundPackets::CookieRequest(_packet) => todo!(),
                // PlayClientboundPackets::Cooldown(_packet) => todo!(),
                // PlayClientboundPackets::CustomChatCompletions(_packet) => todo!(),
                // PlayClientboundPackets::CustomPayload(_packet) => todo!(),
                // PlayClientboundPackets::DamageEvent(_packet) => todo!(),
                // PlayClientboundPackets::DebugBlockValue(_packet) => todo!(),
                // PlayClientboundPackets::DebugChunkValue(_packet) => todo!(),
                // PlayClientboundPackets::DebugEntityValue(_packet) => todo!(),
                // PlayClientboundPackets::DebugEvent(_packet) => todo!(),
                // PlayClientboundPackets::DebugSample(_packet) => todo!(),
                // PlayClientboundPackets::DeleteChat(_packet) => todo!(),
                // PlayClientboundPackets::Disconnect(_packet) => todo!(),
                // PlayClientboundPackets::DisguisedChat(_packet) => todo!(),
                // PlayClientboundPackets::EntityEvent(_packet) => todo!(),
                // PlayClientboundPackets::EntityPositionSync(_packet) => todo!(),
                // PlayClientboundPackets::Explode(_packet) => todo!(),
                // PlayClientboundPackets::ForgetLevelChunk(_packet) => todo!(),
                // PlayClientboundPackets::GameEvent(_packet) => todo!(),
                // PlayClientboundPackets::GameRuleValues(_packet) => todo!(),
                // PlayClientboundPackets::GameTestHighlightPos(_packet) => todo!(),
                // PlayClientboundPackets::MountScreenOpen(_packet) => todo!(),
                // PlayClientboundPackets::HurtAnimation(_packet) => todo!(),
                // PlayClientboundPackets::InitializeBorder(_packet) => todo!(),
                // PlayClientboundPackets::KeepAlive(_packet) => todo!(),
                // PlayClientboundPackets::LevelChunkWithLight(_packet) => todo!(),
                // PlayClientboundPackets::LevelEvent(_packet) => todo!(),
                // PlayClientboundPackets::LevelParticles(_packet) => todo!(),
                // PlayClientboundPackets::LightUpdate(_packet) => todo!(),
                // PlayClientboundPackets::Login(_packet) => todo!(),
                // PlayClientboundPackets::LowDiskSpaceWarning(_packet) => todo!(),
                // PlayClientboundPackets::MapItemData(_packet) => todo!(),
                // PlayClientboundPackets::MerchantOffers(_packet) => todo!(),
                // PlayClientboundPackets::MoveEntityPos(_packet) => todo!(),
                // PlayClientboundPackets::MoveEntityPosRot(_packet) => todo!(),
                // PlayClientboundPackets::MoveMinecartAlongTrack(_packet) => todo!(),
                // PlayClientboundPackets::MoveEntityRot(_packet) => todo!(),
                // PlayClientboundPackets::MoveVehicle(_packet) => todo!(),
                // PlayClientboundPackets::OpenBook(_packet) => todo!(),
                // PlayClientboundPackets::OpenScreen(_packet) => todo!(),
                // PlayClientboundPackets::OpenSignEditor(_packet) => todo!(),
                // PlayClientboundPackets::Ping(_packet) => todo!(),
                // PlayClientboundPackets::PongResponse(_packet) => todo!(),
                // PlayClientboundPackets::PlaceGhostRecipe(_packet) => todo!(),
                // PlayClientboundPackets::PlayerAbilities(_packet) => todo!(),
                // PlayClientboundPackets::PlayerChat(_packet) => todo!(),
                // PlayClientboundPackets::PlayerCombatEnd(_packet) => todo!(),
                // PlayClientboundPackets::PlayerCombatEnter(_packet) => todo!(),
                // PlayClientboundPackets::PlayerCombatKill(_packet) => todo!(),
                // PlayClientboundPackets::PlayerInfoRemove(_packet) => todo!(),
                // PlayClientboundPackets::PlayerInfoUpdate(_packet) => todo!(),
                // PlayClientboundPackets::PlayerLookAt(_packet) => todo!(),
                // PlayClientboundPackets::PlayerPosition(_packet) => todo!(),
                // PlayClientboundPackets::PlayerRotation(_packet) => todo!(),
                // PlayClientboundPackets::RecipeBookAdd(_packet) => todo!(),
                // PlayClientboundPackets::RecipeBookRemove(_packet) => todo!(),
                // PlayClientboundPackets::RecipeBookSettings(_packet) => todo!(),
                // PlayClientboundPackets::RemoveEntities(_packet) => todo!(),
                // PlayClientboundPackets::RemoveMobEffect(_packet) => todo!(),
                // PlayClientboundPackets::ResetScore(_packet) => todo!(),
                // PlayClientboundPackets::ResourcePackPop(_packet) => todo!(),
                // PlayClientboundPackets::ResourcePackPush(_packet) => todo!(),
                // PlayClientboundPackets::Respawn(_packet) => todo!(),
                // PlayClientboundPackets::RotateHead(_packet) => todo!(),
                // PlayClientboundPackets::SectionBlocksUpdate(_packet) => todo!(),
                // PlayClientboundPackets::SelectAdvancementsTab(_packet) => todo!(),
                // PlayClientboundPackets::ServerData(_packet) => todo!(),
                // PlayClientboundPackets::SetActionBarText(_packet) => todo!(),
                // PlayClientboundPackets::SetBorderCenter(_packet) => todo!(),
                // PlayClientboundPackets::SetBorderLerpSize(_packet) => todo!(),
                // PlayClientboundPackets::SetBorderSize(_packet) => todo!(),
                // PlayClientboundPackets::SetBorderWarningDelay(_packet) => todo!(),
                // PlayClientboundPackets::SetBorderWarningDistance(_packet) => todo!(),
                // PlayClientboundPackets::SetCamera(_packet) => todo!(),
                // PlayClientboundPackets::SetChunkCacheCenter(_packet) => todo!(),
                // PlayClientboundPackets::SetChunkCacheRadius(_packet) => todo!(),
                // PlayClientboundPackets::SetCursorItem(_packet) => todo!(),
                // PlayClientboundPackets::SetDefaultSpawnPosition(_packet) => todo!(),
                // PlayClientboundPackets::SetDisplayObjective(_packet) => todo!(),
                // PlayClientboundPackets::SetEntityData(_packet) => todo!(),
                // PlayClientboundPackets::SetEntityLink(_packet) => todo!(),
                // PlayClientboundPackets::SetEntityMotion(_packet) => todo!(),
                // PlayClientboundPackets::SetEquipment(_packet) => todo!(),
                // PlayClientboundPackets::SetExperience(_packet) => todo!(),
                // PlayClientboundPackets::SetHealth(_packet) => todo!(),
                // PlayClientboundPackets::SetHeldSlot(_packet) => todo!(),
                // PlayClientboundPackets::SetObjective(_packet) => todo!(),
                // PlayClientboundPackets::SetPassengers(_packet) => todo!(),
                // PlayClientboundPackets::SetPlayerInventory(_packet) => todo!(),
                // PlayClientboundPackets::SetPlayerTeam(_packet) => todo!(),
                // PlayClientboundPackets::SetScore(_packet) => todo!(),
                // PlayClientboundPackets::SetSimulationDistance(_packet) => todo!(),
                // PlayClientboundPackets::SetSubtitleText(_packet) => todo!(),
                // PlayClientboundPackets::SetTime(_packet) => todo!(),
                // PlayClientboundPackets::SetTitleText(_packet) => todo!(),
                // PlayClientboundPackets::SetTitlesAnimation(_packet) => todo!(),
                // PlayClientboundPackets::SoundEntity(_packet) => todo!(),
                // PlayClientboundPackets::Sound(_packet) => todo!(),
                // PlayClientboundPackets::StartConfiguration(_packet) => todo!(),
                // PlayClientboundPackets::StopSound(_packet) => todo!(),
                // PlayClientboundPackets::StoreCookie(_packet) => todo!(),
                // PlayClientboundPackets::SystemChat(_packet) => todo!(),
                // PlayClientboundPackets::TabList(_packet) => todo!(),
                // PlayClientboundPackets::TagQuery(_packet) => todo!(),
                // PlayClientboundPackets::TakeItemEntity(_packet) => todo!(),
                // PlayClientboundPackets::TeleportEntity(_packet) => todo!(),
                // PlayClientboundPackets::TestInstanceBlockStatus(_packet) => todo!(),
                // PlayClientboundPackets::TickingState(_packet) => todo!(),
                // PlayClientboundPackets::TickingStep(_packet) => todo!(),
                // PlayClientboundPackets::Transfer(_packet) => todo!(),
                // PlayClientboundPackets::UpdateAdvancements(_packet) => todo!(),
                // PlayClientboundPackets::UpdateAttributes(_packet) => todo!(),
                // PlayClientboundPackets::UpdateMobEffect(_packet) => todo!(),
                // PlayClientboundPackets::UpdateRecipes(_packet) => todo!(),
                // PlayClientboundPackets::UpdateTags(_packet) => todo!(),
                // PlayClientboundPackets::ProjectilePower(_packet) => todo!(),
                // PlayClientboundPackets::CustomReportDetails(_packet) => todo!(),
                // PlayClientboundPackets::ServerLinks(_packet) => todo!(),
                // PlayClientboundPackets::Waypoint(_packet) => todo!(),
                // PlayClientboundPackets::ClearDialog(_packet) => todo!(),
                // PlayClientboundPackets::ShowDialog(_packet) => todo!(),
                _ => Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::Placeholder))),
            },
        }
    }

    fn server_event_to_packet(
        event: ServerboundEventEnum,
    ) -> Result<Option<VersionPacket<Self, Serverbound>>, ConnectionError> {
        match event {
            ServerboundEventEnum::Handshake(handshake) => {
                let ServerboundHandshakeEvent::Handshake(event) = handshake;
                let packet = IntentionC2SPacket::new(event);
                Ok(Some(VersionPacket::Handshake(HandshakeServerboundPackets::Intention(packet))))
            }

            ServerboundEventEnum::Status(_status) => {
                todo!()
            }

            ServerboundEventEnum::Login(login) => match login {
                ServerboundLoginEvent::Hello(event) => {
                    let packet = HelloC2SPacket::new(event);
                    Ok(Some(VersionPacket::Login(LoginServerboundPackets::Hello(packet))))
                }
                ServerboundLoginEvent::EncryptionResponse() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Login(LoginServerboundPackets::Key(packet))))
                }
                ServerboundLoginEvent::QueryResponse() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Login(LoginServerboundPackets::CustomQueryAnswer(
                        packet,
                    ))))
                }
                ServerboundLoginEvent::Cookieresponse() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Login(LoginServerboundPackets::CookieResponse(packet))))
                }
                ServerboundLoginEvent::AcknowledgeLogin => {
                    let packet = LoginAcknowledgedC2SPacket;
                    Ok(Some(VersionPacket::Login(LoginServerboundPackets::LoginAcknowledged(
                        packet,
                    ))))
                }
            },

            ServerboundEventEnum::Config(config) => match config {
                ServerboundConfigEvent::ClientInformation(information) => {
                    let packet = ClientInformationC2SPacket { information };
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::ClientInformation(
                        packet,
                    ))))
                }
                ServerboundConfigEvent::KeepAlive(id) => {
                    let packet = KeepAliveC2SPacket { id };
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::KeepAlive(packet))))
                }
                ServerboundConfigEvent::Pong(id) => {
                    let packet = PongC2SPacket { id };
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::Pong(packet))))
                }
                ServerboundConfigEvent::ResourcePackResponse(selected) => {
                    let packet = SelectKnownPacksC2SPacket { selected };
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
                ServerboundConfigEvent::CustomQuery(identifier, buffer) => {
                    let packet = ConfigCustomPayloadC2SPacket { identifier, buffer };
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::CustomPayload(packet))))
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
                ServerboundConfigEvent::AcknowledgeConfig => Ok(Some(VersionPacket::Config(
                    ConfigServerboundPackets::FinishConfiguration(FinishConfigurationC2SPacket),
                ))),
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
                HandshakeServerboundPackets::Intention(packet) => Ok(Some(
                    ServerboundEventEnum::Handshake(ServerboundHandshakeEvent::Handshake(packet.0)),
                )),
            },

            VersionPacket::Status(_) => todo!(),

            VersionPacket::Login(login) => match login {
                LoginServerboundPackets::Hello(packet) => {
                    Ok(Some(ServerboundEventEnum::Login(ServerboundLoginEvent::Hello(packet.0))))
                }
                LoginServerboundPackets::Key(_packet) => Ok(Some(ServerboundEventEnum::Login(
                    ServerboundLoginEvent::EncryptionResponse(),
                ))),
                LoginServerboundPackets::CustomQueryAnswer(_packet) => {
                    Ok(Some(ServerboundEventEnum::Login(ServerboundLoginEvent::QueryResponse())))
                }
                LoginServerboundPackets::LoginAcknowledged(_) => {
                    Ok(Some(ServerboundEventEnum::Login(ServerboundLoginEvent::AcknowledgeLogin)))
                }
                LoginServerboundPackets::CookieResponse(_packet) => {
                    Ok(Some(ServerboundEventEnum::Login(ServerboundLoginEvent::Cookieresponse())))
                }
            },

            VersionPacket::Config(config) => match config {
                ConfigServerboundPackets::ClientInformation(packet) => {
                    Ok(Some(ServerboundEventEnum::Config(
                        ServerboundConfigEvent::ClientInformation(packet.information),
                    )))
                }
                ConfigServerboundPackets::CookieResponse(_packet) => {
                    Ok(Some(ServerboundEventEnum::Config(ServerboundConfigEvent::CookieResponse())))
                }
                ConfigServerboundPackets::CustomPayload(packet) => {
                    Ok(Some(ServerboundEventEnum::Config(ServerboundConfigEvent::CustomQuery(
                        packet.identifier,
                        packet.buffer,
                    ))))
                }
                ConfigServerboundPackets::FinishConfiguration(_) => Ok(Some(
                    ServerboundEventEnum::Config(ServerboundConfigEvent::AcknowledgeConfig),
                )),
                ConfigServerboundPackets::KeepAlive(packet) => Ok(Some(
                    ServerboundEventEnum::Config(ServerboundConfigEvent::KeepAlive(packet.id)),
                )),
                ConfigServerboundPackets::Pong(packet) => {
                    Ok(Some(ServerboundEventEnum::Config(ServerboundConfigEvent::Pong(packet.id))))
                }
                ConfigServerboundPackets::ResourcePack(_packet) => Ok(Some(
                    ServerboundEventEnum::Config(ServerboundConfigEvent::ResourcePackUpdate()),
                )),
                ConfigServerboundPackets::SelectKnownPacks(packet) => {
                    Ok(Some(ServerboundEventEnum::Config(
                        ServerboundConfigEvent::ResourcePackResponse(packet.selected),
                    )))
                }
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
