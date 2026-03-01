//! [`EventVersion`] implementation for [`V26_1`].
#![expect(clippy::too_many_lines, reason = "Huge match statements for packet/event conversion")]
#![expect(unreachable_code, unused_variables, reason = "WIP")]

use froglight_common::version::V26_1;
use froglight_packet::{
    common::entity_id::VarEntityId,
    generated::v26_1::{
        configuration::{
            ClearDialogS2CPacket as LoginClearDialogS2CPacket, ClientInformationC2SPacket,
            ClientboundPackets as ConfigClientboundPackets,
            CookieResponseC2SPacket as ConfigCookieResponseC2SPacket,
            CustomPayloadC2SPacket as ConfigCustomPayloadC2SPacket,
            CustomPayloadS2CPacket as ConfigCustomPayloadS2CPacket,
            DisconnectS2CPacket as ConfigDisconnectS2CPacket, FinishConfigurationC2SPacket,
            FinishConfigurationS2CPacket, KeepAliveC2SPacket,
            KeepAliveS2CPacket as ConfigKeepAliveS2CPacket, PingS2CPacket,
            PongC2SPacket as ConfigPongC2SPacket, SelectKnownPacksC2SPacket,
            SelectKnownPacksS2CPacket, ServerboundPackets as ConfigServerboundPackets,
            StoreCookieS2CPacket as ConfigStoreCookieS2CPacket,
        },
        handshake::{IntentionC2SPacket, ServerboundPackets as HandshakeServerboundPackets},
        login::{
            ClientboundPackets as LoginClientboundPackets,
            CookieRequestS2CPacket as LoginCookieRequestS2CPacket,
            CookieResponseC2SPacket as LoginCookieResponseC2SPacket, CustomQueryAnswerC2SPacket,
            CustomQueryS2CPacket, HelloC2SPacket, LoginAcknowledgedC2SPacket,
            LoginDisconnectS2CPacket, LoginFinishedS2CPacket,
            ServerboundPackets as LoginServerboundPackets,
        },
        play::{
            AddEntityS2CPacket, BundleDelimiterS2CPacket, ChunkBatchFinishedS2CPacket,
            ChunkBatchReceivedC2SPacket, ChunkBatchStartS2CPacket,
            ClearDialogS2CPacket as PlayClearDialogS2CPacket,
            ClientboundPackets as PlayClientboundPackets,
            CustomPayloadS2CPacket as PlayCustomPayloadS2CPacket,
            DisconnectS2CPacket as PlayDisconnectS2CPacket,
            KeepAliveC2SPacket as PlayKeepAliveC2SPacket,
            KeepAliveS2CPacket as PlayKeepAliveS2CPacket, LevelChunkWithLightS2CPacket,
            LoginS2CPacket, PingRequestC2SPacket as PlayPingRequestC2SPacket,
            PongC2SPacket as PlayPongC2SPacket, PongResponseS2CPacket as PlayPongResponseS2CPacket,
            RemoveEntitiesS2CPacket, ServerboundPackets as PlayServerboundPackets,
        },
    },
    version::{Clientbound, Serverbound, VersionPacket},
};
use froglight_world::prelude::ChunkPos;

use crate::{
    connection::ConnectionError,
    event::{
        EventVersion,
        enums::{
            ClientboundConfigEvent, ClientboundLoginEvent, ClientboundPlayEvent,
            ServerboundConfigEvent, ServerboundHandshakeEvent, ServerboundLoginEvent,
            ServerboundPlayEvent,
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
                ClientboundLoginEvent::CompressionThreshold(_) => Ok(None),
                ClientboundLoginEvent::CookieRequest(identifier) => {
                    let packet = LoginCookieRequestS2CPacket { cookie: identifier };
                    Ok(Some(VersionPacket::Login(LoginClientboundPackets::CookieRequest(packet))))
                }
                ClientboundLoginEvent::CustomPayload(query_id, identifier, payload) => {
                    let packet = CustomQueryS2CPacket { query_id, identifier, payload };
                    Ok(Some(VersionPacket::Login(LoginClientboundPackets::CustomQuery(packet))))
                }
                ClientboundLoginEvent::Disconnect(event) => {
                    let packet = LoginDisconnectS2CPacket::new(event);
                    Ok(Some(VersionPacket::Login(LoginClientboundPackets::LoginDisconnect(packet))))
                }
                ClientboundLoginEvent::EncryptionRequest() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Login(LoginClientboundPackets::Hello(packet))))
                }
                ClientboundLoginEvent::LoginFinished(event) => {
                    let packet = LoginFinishedS2CPacket::new(event);
                    Ok(Some(VersionPacket::Login(LoginClientboundPackets::LoginFinished(packet))))
                }
            },

            ClientboundEventEnum::Config(config) => match config {
                ClientboundConfigEvent::Disconnect(reason) => Ok(Some(VersionPacket::Config(
                    ConfigClientboundPackets::Disconnect(ConfigDisconnectS2CPacket { reason }),
                ))),
                ClientboundConfigEvent::Transfer() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::Transfer(packet))))
                }
                ClientboundConfigEvent::KeepAlive(id) => {
                    let packet = ConfigKeepAliveS2CPacket { id };
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
                ClientboundConfigEvent::KnownResourcePacks(known) => {
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
                ClientboundConfigEvent::RegistryData() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::RegistryData(packet))))
                }
                ClientboundConfigEvent::EnabledFeatures() => {
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
                ClientboundConfigEvent::CustomReportDetails() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::CustomReportDetails(
                        packet,
                    ))))
                }
                ClientboundConfigEvent::CustomPayload(identifier, buffer) => {
                    let packet = ConfigCustomPayloadS2CPacket { identifier, buffer };
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::CustomPayload(packet))))
                }
                ClientboundConfigEvent::CookieRequest(identifier) => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::CookieRequest(packet))))
                }
                ClientboundConfigEvent::StoreCookie(cookie, payload) => {
                    let packet = ConfigStoreCookieS2CPacket { cookie, payload };
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::StoreCookie(packet))))
                }
                ClientboundConfigEvent::ShowDialog() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Config(ConfigClientboundPackets::ShowDialog(packet))))
                }
                ClientboundConfigEvent::ClearDialog => Ok(Some(VersionPacket::Config(
                    ConfigClientboundPackets::ClearDialog(LoginClearDialogS2CPacket),
                ))),
                ClientboundConfigEvent::FinishConfig => Ok(Some(VersionPacket::Config(
                    ConfigClientboundPackets::FinishConfiguration(FinishConfigurationS2CPacket {}),
                ))),
            },

            ClientboundEventEnum::Play(play) => match play {
                ClientboundPlayEvent::ActionBarText() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetActionBarText(packet))))
                }
                ClientboundPlayEvent::AddEntity(entity_data) => {
                    let packet = AddEntityS2CPacket(entity_data);
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::AddEntity(packet))))
                }
                ClientboundPlayEvent::Animate() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::Animate(packet))))
                }
                ClientboundPlayEvent::AwardStats() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::AwardStats(packet))))
                }
                ClientboundPlayEvent::BlockChangedAck() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::BlockChangedAck(packet))))
                }
                ClientboundPlayEvent::BlockDestruction() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::BlockDestruction(packet))))
                }
                ClientboundPlayEvent::BlockEntityData() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::BlockEntityData(packet))))
                }
                ClientboundPlayEvent::BlockEvent() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::BlockEvent(packet))))
                }
                ClientboundPlayEvent::BlockUpdate() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::BlockUpdate(packet))))
                }
                ClientboundPlayEvent::BossEvent() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::BossEvent(packet))))
                }
                ClientboundPlayEvent::BundleDelimiter => Ok(Some(VersionPacket::Play(
                    PlayClientboundPackets::BundleDelimiter(BundleDelimiterS2CPacket),
                ))),
                ClientboundPlayEvent::ChangeDifficulty() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::ChangeDifficulty(packet))))
                }
                ClientboundPlayEvent::ChatSuggestions() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::CustomChatCompletions(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::ChunkBatchFinished(batch_size) => {
                    let packet = ChunkBatchFinishedS2CPacket { batch_size };
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::ChunkBatchFinished(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::ChunkBatchStart => Ok(Some(VersionPacket::Play(
                    PlayClientboundPackets::ChunkBatchStart(ChunkBatchStartS2CPacket),
                ))),
                ClientboundPlayEvent::ChunkBiomes() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::ChunksBiomes(packet))))
                }
                ClientboundPlayEvent::ChunkCacheCenter() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetChunkCacheCenter(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::ChunkCacheRadius() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetChunkCacheRadius(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::ChunkSectionUpdate() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SectionBlocksUpdate(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::ChunkWithLight(pos, chunk, light) => {
                    let packet = LevelChunkWithLightS2CPacket {
                        chunk_x: pos.x(),
                        chunk_z: pos.z(),
                        chunk_data: chunk,
                        light_data: light,
                    };
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::LevelChunkWithLight(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::ClearDialog => Ok(Some(VersionPacket::Play(
                    PlayClientboundPackets::ClearDialog(PlayClearDialogS2CPacket),
                ))),
                ClientboundPlayEvent::ClearTitles() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::ClearTitles(packet))))
                }
                ClientboundPlayEvent::CommandSuggestions() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::CommandSuggestions(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::Commands() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::Commands(packet))))
                }
                ClientboundPlayEvent::ContainerClose() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::ContainerClose(packet))))
                }
                ClientboundPlayEvent::ContainerContent() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::ContainerSetContent(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::ContainerData() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::ContainerSetData(packet))))
                }
                ClientboundPlayEvent::ContainerSlot() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::ContainerSetSlot(packet))))
                }
                ClientboundPlayEvent::CookieRequest() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::CookieRequest(packet))))
                }
                ClientboundPlayEvent::Cooldown() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::Cooldown(packet))))
                }
                ClientboundPlayEvent::CustomPayload(identifier, buffer) => {
                    let packet = PlayCustomPayloadS2CPacket { identifier, buffer };
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::CustomPayload(packet))))
                }
                ClientboundPlayEvent::CustomReportDetails() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::CustomReportDetails(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::DamageEvent() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::DamageEvent(packet))))
                }
                ClientboundPlayEvent::DebugBlock() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::DebugBlockValue(packet))))
                }
                ClientboundPlayEvent::DebugChunk() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::DebugChunkValue(packet))))
                }
                ClientboundPlayEvent::DebugEntity() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::DebugEntityValue(packet))))
                }
                ClientboundPlayEvent::DebugEvent() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::DebugEvent(packet))))
                }
                ClientboundPlayEvent::DebugSample() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::DebugSample(packet))))
                }
                ClientboundPlayEvent::DeleteChat() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::DeleteChat(packet))))
                }
                ClientboundPlayEvent::Disconnect(reason) => {
                    let packet = PlayDisconnectS2CPacket { reason };
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::Disconnect(packet))))
                }
                ClientboundPlayEvent::DisguisedChat() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::DisguisedChat(packet))))
                }
                ClientboundPlayEvent::DiskSpaceWarning() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::LowDiskSpaceWarning(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::EntityEvent() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::EntityEvent(packet))))
                }
                ClientboundPlayEvent::EntityPosition() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::EntityPositionSync(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::Explode() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::Explode(packet))))
                }
                ClientboundPlayEvent::ForgetChunk() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::ForgetLevelChunk(packet))))
                }
                ClientboundPlayEvent::GameEvent() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::GameEvent(packet))))
                }
                ClientboundPlayEvent::GameRule() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::GameRuleValues(packet))))
                }
                ClientboundPlayEvent::GameTestHighlight() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::GameTestHighlightPos(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::GhostRecipe() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::PlaceGhostRecipe(packet))))
                }
                ClientboundPlayEvent::HurtAnimation() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::HurtAnimation(packet))))
                }
                ClientboundPlayEvent::InitializeBorder() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::InitializeBorder(packet))))
                }
                ClientboundPlayEvent::KeepAlive(id) => {
                    let packet = PlayKeepAliveS2CPacket { id };
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::KeepAlive(packet))))
                }
                ClientboundPlayEvent::LevelEvent() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::LevelEvent(packet))))
                }
                ClientboundPlayEvent::LevelParticles() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::LevelParticles(packet))))
                }
                ClientboundPlayEvent::LightUpdate() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::LightUpdate(packet))))
                }
                ClientboundPlayEvent::Login(content) => {
                    let packet = LoginS2CPacket(content);
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::Login(packet))))
                }
                ClientboundPlayEvent::MapItemData() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::MapItemData(packet))))
                }
                ClientboundPlayEvent::MerchantOffers() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::MerchantOffers(packet))))
                }
                ClientboundPlayEvent::MountScreen() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::MountScreenOpen(packet))))
                }
                ClientboundPlayEvent::MoveEntityPos() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::MoveEntityPos(packet))))
                }
                ClientboundPlayEvent::MoveEntityPosRot() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::MoveEntityPosRot(packet))))
                }
                ClientboundPlayEvent::MoveEntityRot() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::MoveEntityRot(packet))))
                }
                ClientboundPlayEvent::MoveMinecartTrack() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::MoveMinecartAlongTrack(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::MoveVehicle() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::MoveVehicle(packet))))
                }
                ClientboundPlayEvent::OpenBook() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::OpenBook(packet))))
                }
                ClientboundPlayEvent::OpenScreen() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::OpenScreen(packet))))
                }
                ClientboundPlayEvent::OpenSignEditor() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::OpenSignEditor(packet))))
                }
                ClientboundPlayEvent::Ping(id) => {
                    let packet = PlayPongResponseS2CPacket { id };
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::PongResponse(packet))))
                }
                ClientboundPlayEvent::PlayerAbilities() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::PlayerAbilities(packet))))
                }
                ClientboundPlayEvent::PlayerChat() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::PlayerChat(packet))))
                }
                ClientboundPlayEvent::PlayerCombatEnd() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::PlayerCombatEnd(packet))))
                }
                ClientboundPlayEvent::PlayerCombatEnter() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::PlayerCombatEnter(packet))))
                }
                ClientboundPlayEvent::PlayerCombatKill() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::PlayerCombatKill(packet))))
                }
                ClientboundPlayEvent::PlayerInfoRemove() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::PlayerInfoRemove(packet))))
                }
                ClientboundPlayEvent::PlayerInfoUpdate() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::PlayerInfoUpdate(packet))))
                }
                ClientboundPlayEvent::PlayerLookAt() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::PlayerLookAt(packet))))
                }
                ClientboundPlayEvent::PlayerPosition() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::PlayerPosition(packet))))
                }
                ClientboundPlayEvent::PlayerRotation() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::PlayerRotation(packet))))
                }
                ClientboundPlayEvent::Pong(id) => Ok(Some(VersionPacket::Play(
                    PlayClientboundPackets::PongResponse(PlayPongResponseS2CPacket { id }),
                ))),
                ClientboundPlayEvent::ProjectilePower() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::ProjectilePower(packet))))
                }
                ClientboundPlayEvent::RecipeBookAdd() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::RecipeBookAdd(packet))))
                }
                ClientboundPlayEvent::RecipeBookRemove() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::RecipeBookRemove(packet))))
                }
                ClientboundPlayEvent::RecipeBookSettings() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::RecipeBookSettings(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::RemoveEntities(entities) => {
                    let entities = entities.into_iter().map(VarEntityId).collect();
                    let packet = RemoveEntitiesS2CPacket { entities };
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::RemoveEntities(packet))))
                }
                ClientboundPlayEvent::RemoveMobEffect() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::RemoveMobEffect(packet))))
                }
                ClientboundPlayEvent::ResetScore() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::ResetScore(packet))))
                }
                ClientboundPlayEvent::ResourcePackPop() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::ResourcePackPop(packet))))
                }
                ClientboundPlayEvent::ResourcePackPush() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::ResourcePackPush(packet))))
                }
                ClientboundPlayEvent::Respawn() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::Respawn(packet))))
                }
                ClientboundPlayEvent::RotateHead() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::RotateHead(packet))))
                }
                ClientboundPlayEvent::SelectAdvancementTab() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SelectAdvancementsTab(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::ServerData() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::ServerData(packet))))
                }
                ClientboundPlayEvent::ServerLinks() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::ServerLinks(packet))))
                }
                ClientboundPlayEvent::SetBorderCenter() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetBorderCenter(packet))))
                }
                ClientboundPlayEvent::SetBorderLerpSize() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetBorderLerpSize(packet))))
                }
                ClientboundPlayEvent::SetBorderSize() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetBorderSize(packet))))
                }
                ClientboundPlayEvent::SetBorderWarningDelay() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetBorderWarningDelay(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::SetBorderWarningDistance() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetBorderWarningDistance(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::SetCamera() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetCamera(packet))))
                }
                ClientboundPlayEvent::SetCursorItem() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetCursorItem(packet))))
                }
                ClientboundPlayEvent::SetDefaultSpawn() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetDefaultSpawnPosition(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::SetDisplayObjective() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetDisplayObjective(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::SetEntityData() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetEntityData(packet))))
                }
                ClientboundPlayEvent::SetEntityLink() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetEntityLink(packet))))
                }
                ClientboundPlayEvent::SetEntityMotion() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetEntityMotion(packet))))
                }
                ClientboundPlayEvent::SetEquipment() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetEquipment(packet))))
                }
                ClientboundPlayEvent::SetExperience() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetExperience(packet))))
                }
                ClientboundPlayEvent::SetHealth() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetHealth(packet))))
                }
                ClientboundPlayEvent::SetHeldSlot() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetHeldSlot(packet))))
                }
                ClientboundPlayEvent::SetObjective() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetObjective(packet))))
                }
                ClientboundPlayEvent::SetPassengers() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetPassengers(packet))))
                }
                ClientboundPlayEvent::SetPlayerInventory() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetPlayerInventory(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::SetPlayerTeam() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetPlayerTeam(packet))))
                }
                ClientboundPlayEvent::SetScore() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetScore(packet))))
                }
                ClientboundPlayEvent::SetSimulationDistance() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetSimulationDistance(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::SetSubtitleText() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetSubtitleText(packet))))
                }
                ClientboundPlayEvent::SetTime() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetTime(packet))))
                }
                ClientboundPlayEvent::SetTitleAnimation() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetTitlesAnimation(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::SetTitleText() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SetTitleText(packet))))
                }
                ClientboundPlayEvent::ShowDialog() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::ShowDialog(packet))))
                }
                ClientboundPlayEvent::Sound() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::Sound(packet))))
                }
                ClientboundPlayEvent::SoundEntity() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SoundEntity(packet))))
                }
                ClientboundPlayEvent::StartConfiguration => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::StartConfiguration(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::StopSound() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::StopSound(packet))))
                }
                ClientboundPlayEvent::StoreCookie() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::StoreCookie(packet))))
                }
                ClientboundPlayEvent::SystemChat() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::SystemChat(packet))))
                }
                ClientboundPlayEvent::TabList() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::TabList(packet))))
                }
                ClientboundPlayEvent::TagQuery() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::TagQuery(packet))))
                }
                ClientboundPlayEvent::TakeItemEntity() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::TakeItemEntity(packet))))
                }
                ClientboundPlayEvent::TeleportEntity() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::TeleportEntity(packet))))
                }
                ClientboundPlayEvent::TestBlockStatus() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::TestInstanceBlockStatus(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::TickingState() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::TickingState(packet))))
                }
                ClientboundPlayEvent::TickingStep() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::TickingStep(packet))))
                }
                ClientboundPlayEvent::Transfer() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::Transfer(packet))))
                }
                ClientboundPlayEvent::UpdateAdvancements() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::UpdateAdvancements(
                        packet,
                    ))))
                }
                ClientboundPlayEvent::UpdateAttributes() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::UpdateAttributes(packet))))
                }
                ClientboundPlayEvent::UpdateMobEffect() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::UpdateMobEffect(packet))))
                }
                ClientboundPlayEvent::UpdateRecipes() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::UpdateRecipes(packet))))
                }
                ClientboundPlayEvent::UpdateTags() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::UpdateTags(packet))))
                }
                ClientboundPlayEvent::Waypoint() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Play(PlayClientboundPackets::Waypoint(packet))))
                }
            },
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
                LoginClientboundPackets::LoginFinished(packet) => {
                    Ok(Some(ClientboundEventEnum::Login(ClientboundLoginEvent::LoginFinished(
                        packet.profile,
                    ))))
                }
                LoginClientboundPackets::LoginCompression(_) => Ok(None),
                LoginClientboundPackets::CustomQuery(packet) => {
                    Ok(Some(ClientboundEventEnum::Login(ClientboundLoginEvent::CustomPayload(
                        packet.query_id,
                        packet.identifier,
                        packet.payload,
                    ))))
                }
                LoginClientboundPackets::CookieRequest(packet) => {
                    Ok(Some(ClientboundEventEnum::Login(ClientboundLoginEvent::CookieRequest(
                        packet.cookie,
                    ))))
                }
            },

            VersionPacket::Config(config) => match config {
                ConfigClientboundPackets::CookieRequest(packet) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::CookieRequest(
                        packet.cookie,
                    ))))
                }
                ConfigClientboundPackets::CustomPayload(packet) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::CustomPayload(
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
                ConfigClientboundPackets::RegistryData(_packet) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::RegistryData())))
                }
                ConfigClientboundPackets::ResourcePackPop(_packet) => Ok(Some(
                    ClientboundEventEnum::Config(ClientboundConfigEvent::ResourcePackPop()),
                )),
                ConfigClientboundPackets::ResourcePackPush(_packet) => Ok(Some(
                    ClientboundEventEnum::Config(ClientboundConfigEvent::ResourcePackPush()),
                )),
                ConfigClientboundPackets::StoreCookie(packet) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::StoreCookie(
                        packet.cookie,
                        packet.payload,
                    ))))
                }
                ConfigClientboundPackets::Transfer(_packet) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::Transfer())))
                }
                ConfigClientboundPackets::UpdateEnabledFeatures(_packet) => Ok(Some(
                    ClientboundEventEnum::Config(ClientboundConfigEvent::EnabledFeatures()),
                )),
                ConfigClientboundPackets::UpdateTags(_packet) => {
                    Ok(Some(ClientboundEventEnum::Config(ClientboundConfigEvent::UpdateTags())))
                }
                ConfigClientboundPackets::SelectKnownPacks(packet) => {
                    Ok(Some(ClientboundEventEnum::Config(
                        ClientboundConfigEvent::KnownResourcePacks(packet.known),
                    )))
                }
                ConfigClientboundPackets::CustomReportDetails(_packet) => Ok(Some(
                    ClientboundEventEnum::Config(ClientboundConfigEvent::CustomReportDetails()),
                )),
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

            VersionPacket::Play(play) => match play {
                PlayClientboundPackets::BundleDelimiter(_) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::BundleDelimiter)))
                }
                PlayClientboundPackets::AddEntity(packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::AddEntity(packet.0))))
                }
                PlayClientboundPackets::Animate(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::Animate())))
                }
                PlayClientboundPackets::AwardStats(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::AwardStats())))
                }
                PlayClientboundPackets::BlockChangedAck(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::BlockChangedAck())))
                }
                PlayClientboundPackets::BlockDestruction(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::BlockDestruction())))
                }
                PlayClientboundPackets::BlockEntityData(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::BlockEntityData())))
                }
                PlayClientboundPackets::BlockEvent(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::BlockEvent())))
                }
                PlayClientboundPackets::BlockUpdate(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::BlockUpdate())))
                }
                PlayClientboundPackets::BossEvent(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::BossEvent())))
                }
                PlayClientboundPackets::ChangeDifficulty(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ChangeDifficulty())))
                }
                PlayClientboundPackets::ChunkBatchFinished(packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ChunkBatchFinished(
                        packet.batch_size,
                    ))))
                }
                PlayClientboundPackets::ChunkBatchStart(_) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ChunkBatchStart)))
                }
                PlayClientboundPackets::ChunksBiomes(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ChunkBiomes())))
                }
                PlayClientboundPackets::ClearTitles(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ClearTitles())))
                }
                PlayClientboundPackets::CommandSuggestions(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::CommandSuggestions())))
                }
                PlayClientboundPackets::Commands(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::Commands())))
                }
                PlayClientboundPackets::ContainerClose(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ContainerClose())))
                }
                PlayClientboundPackets::ContainerSetContent(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ContainerContent())))
                }
                PlayClientboundPackets::ContainerSetData(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ContainerData())))
                }
                PlayClientboundPackets::ContainerSetSlot(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ContainerSlot())))
                }
                PlayClientboundPackets::CookieRequest(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::CookieRequest())))
                }
                PlayClientboundPackets::Cooldown(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::Cooldown())))
                }
                PlayClientboundPackets::CustomChatCompletions(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ChatSuggestions())))
                }
                PlayClientboundPackets::CustomPayload(packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::CustomPayload(
                        packet.identifier,
                        packet.buffer,
                    ))))
                }
                PlayClientboundPackets::DamageEvent(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::DamageEvent())))
                }
                PlayClientboundPackets::DebugBlockValue(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::DebugBlock())))
                }
                PlayClientboundPackets::DebugChunkValue(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::DebugChunk())))
                }
                PlayClientboundPackets::DebugEntityValue(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::DebugEntity())))
                }
                PlayClientboundPackets::DebugEvent(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::DebugEvent())))
                }
                PlayClientboundPackets::DebugSample(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::DebugSample())))
                }
                PlayClientboundPackets::DeleteChat(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::DeleteChat())))
                }
                PlayClientboundPackets::Disconnect(packet) => Ok(Some(ClientboundEventEnum::Play(
                    ClientboundPlayEvent::Disconnect(packet.reason),
                ))),
                PlayClientboundPackets::DisguisedChat(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::DisguisedChat())))
                }
                PlayClientboundPackets::EntityEvent(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::EntityEvent())))
                }
                PlayClientboundPackets::EntityPositionSync(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::EntityPosition())))
                }
                PlayClientboundPackets::Explode(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::Explode())))
                }
                PlayClientboundPackets::ForgetLevelChunk(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ForgetChunk())))
                }
                PlayClientboundPackets::GameEvent(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::GameEvent())))
                }
                PlayClientboundPackets::GameRuleValues(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::GameRule())))
                }
                PlayClientboundPackets::GameTestHighlightPos(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::GameTestHighlight())))
                }
                PlayClientboundPackets::MountScreenOpen(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::MountScreen())))
                }
                PlayClientboundPackets::HurtAnimation(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::HurtAnimation())))
                }
                PlayClientboundPackets::InitializeBorder(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::InitializeBorder())))
                }
                PlayClientboundPackets::KeepAlive(packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::KeepAlive(packet.id))))
                }
                PlayClientboundPackets::LevelChunkWithLight(packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ChunkWithLight(
                        ChunkPos::new_xz(packet.chunk_x, packet.chunk_z),
                        packet.chunk_data,
                        packet.light_data,
                    ))))
                }
                PlayClientboundPackets::LevelEvent(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::LevelEvent())))
                }
                PlayClientboundPackets::LevelParticles(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::LevelParticles())))
                }
                PlayClientboundPackets::LightUpdate(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::LightUpdate())))
                }
                PlayClientboundPackets::Login(packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::Login(packet.0))))
                }
                PlayClientboundPackets::LowDiskSpaceWarning(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::DiskSpaceWarning())))
                }
                PlayClientboundPackets::MapItemData(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::MapItemData())))
                }
                PlayClientboundPackets::MerchantOffers(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::MerchantOffers())))
                }
                PlayClientboundPackets::MoveEntityPos(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::MoveEntityPos())))
                }
                PlayClientboundPackets::MoveEntityPosRot(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::MoveEntityPosRot())))
                }
                PlayClientboundPackets::MoveMinecartAlongTrack(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::MoveEntityPosRot())))
                }
                PlayClientboundPackets::MoveEntityRot(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::MoveEntityRot())))
                }
                PlayClientboundPackets::MoveVehicle(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::MoveVehicle())))
                }
                PlayClientboundPackets::OpenBook(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::OpenBook())))
                }
                PlayClientboundPackets::OpenScreen(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::OpenScreen())))
                }
                PlayClientboundPackets::OpenSignEditor(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::OpenSignEditor())))
                }
                PlayClientboundPackets::Ping(packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::Ping(packet.id))))
                }
                PlayClientboundPackets::PongResponse(packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::Pong(packet.id))))
                }
                PlayClientboundPackets::PlaceGhostRecipe(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::GhostRecipe())))
                }
                PlayClientboundPackets::PlayerAbilities(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::PlayerAbilities())))
                }
                PlayClientboundPackets::PlayerChat(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::PlayerChat())))
                }
                PlayClientboundPackets::PlayerCombatEnd(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::PlayerCombatEnd())))
                }
                PlayClientboundPackets::PlayerCombatEnter(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::PlayerCombatEnter())))
                }
                PlayClientboundPackets::PlayerCombatKill(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::PlayerCombatKill())))
                }
                PlayClientboundPackets::PlayerInfoRemove(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::PlayerInfoRemove())))
                }
                PlayClientboundPackets::PlayerInfoUpdate(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::PlayerInfoUpdate())))
                }
                PlayClientboundPackets::PlayerLookAt(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::PlayerLookAt())))
                }
                PlayClientboundPackets::PlayerPosition(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::PlayerPosition())))
                }
                PlayClientboundPackets::PlayerRotation(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::PlayerRotation())))
                }
                PlayClientboundPackets::RecipeBookAdd(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::RecipeBookAdd())))
                }
                PlayClientboundPackets::RecipeBookRemove(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::RecipeBookRemove())))
                }
                PlayClientboundPackets::RecipeBookSettings(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::RecipeBookSettings())))
                }
                PlayClientboundPackets::RemoveEntities(packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::RemoveEntities(
                        packet.entities.into_iter().map(|entity| entity.0).collect(),
                    ))))
                }
                PlayClientboundPackets::RemoveMobEffect(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::RemoveMobEffect())))
                }
                PlayClientboundPackets::ResetScore(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ResetScore())))
                }
                PlayClientboundPackets::ResourcePackPop(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ResourcePackPop())))
                }
                PlayClientboundPackets::ResourcePackPush(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ResourcePackPush())))
                }
                PlayClientboundPackets::Respawn(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::Respawn())))
                }
                PlayClientboundPackets::RotateHead(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::RotateHead())))
                }
                PlayClientboundPackets::SectionBlocksUpdate(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ChunkSectionUpdate())))
                }
                PlayClientboundPackets::SelectAdvancementsTab(_packet) => Ok(Some(
                    ClientboundEventEnum::Play(ClientboundPlayEvent::SelectAdvancementTab()),
                )),
                PlayClientboundPackets::ServerData(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ServerData())))
                }
                PlayClientboundPackets::SetActionBarText(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ActionBarText())))
                }
                PlayClientboundPackets::SetBorderCenter(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SetBorderCenter())))
                }
                PlayClientboundPackets::SetBorderLerpSize(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SetBorderLerpSize())))
                }
                PlayClientboundPackets::SetBorderSize(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SetBorderSize())))
                }
                PlayClientboundPackets::SetBorderWarningDelay(_packet) => Ok(Some(
                    ClientboundEventEnum::Play(ClientboundPlayEvent::SetBorderWarningDelay()),
                )),
                PlayClientboundPackets::SetBorderWarningDistance(_packet) => Ok(Some(
                    ClientboundEventEnum::Play(ClientboundPlayEvent::SetBorderWarningDistance()),
                )),
                PlayClientboundPackets::SetCamera(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SetCamera())))
                }
                PlayClientboundPackets::SetChunkCacheCenter(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ChunkCacheCenter())))
                }
                PlayClientboundPackets::SetChunkCacheRadius(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ChunkCacheRadius())))
                }
                PlayClientboundPackets::SetCursorItem(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SetCursorItem())))
                }
                PlayClientboundPackets::SetDefaultSpawnPosition(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SetDefaultSpawn())))
                }
                PlayClientboundPackets::SetDisplayObjective(_packet) => Ok(Some(
                    ClientboundEventEnum::Play(ClientboundPlayEvent::SetDisplayObjective()),
                )),
                PlayClientboundPackets::SetEntityData(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SetEntityData())))
                }
                PlayClientboundPackets::SetEntityLink(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SetEntityLink())))
                }
                PlayClientboundPackets::SetEntityMotion(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SetEntityMotion())))
                }
                PlayClientboundPackets::SetEquipment(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SetEquipment())))
                }
                PlayClientboundPackets::SetExperience(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SetExperience())))
                }
                PlayClientboundPackets::SetHealth(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SetHealth())))
                }
                PlayClientboundPackets::SetHeldSlot(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SetHeldSlot())))
                }
                PlayClientboundPackets::SetObjective(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SetObjective())))
                }
                PlayClientboundPackets::SetPassengers(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SetPassengers())))
                }
                PlayClientboundPackets::SetPlayerInventory(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SetPlayerInventory())))
                }
                PlayClientboundPackets::SetPlayerTeam(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SetPlayerTeam())))
                }
                PlayClientboundPackets::SetScore(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SetScore())))
                }
                PlayClientboundPackets::SetSimulationDistance(_packet) => Ok(Some(
                    ClientboundEventEnum::Play(ClientboundPlayEvent::SetSimulationDistance()),
                )),
                PlayClientboundPackets::SetSubtitleText(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SetSubtitleText())))
                }
                PlayClientboundPackets::SetTime(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SetTime())))
                }
                PlayClientboundPackets::SetTitleText(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SetTitleText())))
                }
                PlayClientboundPackets::SetTitlesAnimation(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SetTitleAnimation())))
                }
                PlayClientboundPackets::SoundEntity(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SoundEntity())))
                }
                PlayClientboundPackets::Sound(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::Sound())))
                }
                PlayClientboundPackets::StartConfiguration(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::StartConfiguration)))
                }
                PlayClientboundPackets::StopSound(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::StopSound())))
                }
                PlayClientboundPackets::StoreCookie(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::StoreCookie())))
                }
                PlayClientboundPackets::SystemChat(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::SystemChat())))
                }
                PlayClientboundPackets::TabList(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::TabList())))
                }
                PlayClientboundPackets::TagQuery(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::TagQuery())))
                }
                PlayClientboundPackets::TakeItemEntity(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::TakeItemEntity())))
                }
                PlayClientboundPackets::TeleportEntity(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::TeleportEntity())))
                }
                PlayClientboundPackets::TestInstanceBlockStatus(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::TestBlockStatus())))
                }
                PlayClientboundPackets::TickingState(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::TickingState())))
                }
                PlayClientboundPackets::TickingStep(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::TickingStep())))
                }
                PlayClientboundPackets::Transfer(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::Transfer())))
                }
                PlayClientboundPackets::UpdateAdvancements(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::UpdateAdvancements())))
                }
                PlayClientboundPackets::UpdateAttributes(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::UpdateAttributes())))
                }
                PlayClientboundPackets::UpdateMobEffect(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::UpdateMobEffect())))
                }
                PlayClientboundPackets::UpdateRecipes(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::UpdateRecipes())))
                }
                PlayClientboundPackets::UpdateTags(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::UpdateTags())))
                }
                PlayClientboundPackets::ProjectilePower(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ProjectilePower())))
                }
                PlayClientboundPackets::CustomReportDetails(_packet) => Ok(Some(
                    ClientboundEventEnum::Play(ClientboundPlayEvent::CustomReportDetails()),
                )),
                PlayClientboundPackets::ServerLinks(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ServerLinks())))
                }
                PlayClientboundPackets::Waypoint(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::Waypoint())))
                }
                PlayClientboundPackets::ClearDialog(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ClearDialog)))
                }
                PlayClientboundPackets::ShowDialog(_packet) => {
                    Ok(Some(ClientboundEventEnum::Play(ClientboundPlayEvent::ShowDialog())))
                }
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
                ServerboundLoginEvent::AcknowledgeLogin => Ok(Some(VersionPacket::Login(
                    LoginServerboundPackets::LoginAcknowledged(LoginAcknowledgedC2SPacket),
                ))),
                ServerboundLoginEvent::CookieResponse(cookie, payload) => {
                    let packet = LoginCookieResponseC2SPacket { cookie, payload };
                    Ok(Some(VersionPacket::Login(LoginServerboundPackets::CookieResponse(packet))))
                }
                ServerboundLoginEvent::CustomPayload(query_id, payload) => {
                    let packet = CustomQueryAnswerC2SPacket { query_id, payload };
                    Ok(Some(VersionPacket::Login(LoginServerboundPackets::CustomQueryAnswer(
                        packet,
                    ))))
                }
                ServerboundLoginEvent::EncryptionResponse() => {
                    let packet = todo!();
                    Ok(Some(VersionPacket::Login(LoginServerboundPackets::Key(packet))))
                }
                ServerboundLoginEvent::Hello(event) => {
                    let packet = HelloC2SPacket::new(event);
                    Ok(Some(VersionPacket::Login(LoginServerboundPackets::Hello(packet))))
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
                    let packet = ConfigPongC2SPacket { id };
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
                ServerboundConfigEvent::CustomPayload(identifier, buffer) => {
                    let packet = ConfigCustomPayloadC2SPacket { identifier, buffer };
                    Ok(Some(VersionPacket::Config(ConfigServerboundPackets::CustomPayload(packet))))
                }
                ServerboundConfigEvent::CookieResponse(cookie, payload) => {
                    let packet = ConfigCookieResponseC2SPacket { cookie, payload };
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

            ServerboundEventEnum::Play(play) => match play {
                ServerboundPlayEvent::ChunkBatchReceived(rate) => {
                    let packet = ChunkBatchReceivedC2SPacket { rate };
                    Ok(Some(VersionPacket::Play(PlayServerboundPackets::ChunkBatchReceived(
                        packet,
                    ))))
                }
                ServerboundPlayEvent::KeepAlive(id) => {
                    let packet = PlayKeepAliveC2SPacket { id };
                    Ok(Some(VersionPacket::Play(PlayServerboundPackets::KeepAlive(packet))))
                }
                ServerboundPlayEvent::PingRequest(id) => {
                    let packet = PlayPingRequestC2SPacket { id };
                    Ok(Some(VersionPacket::Play(PlayServerboundPackets::PingRequest(packet))))
                }
                ServerboundPlayEvent::Pong(id) => {
                    let packet = PlayPongC2SPacket { id };
                    Ok(Some(VersionPacket::Play(PlayServerboundPackets::Pong(packet))))
                }
            },
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
                LoginServerboundPackets::CustomQueryAnswer(packet) => {
                    Ok(Some(ServerboundEventEnum::Login(ServerboundLoginEvent::CustomPayload(
                        packet.query_id,
                        packet.payload,
                    ))))
                }
                LoginServerboundPackets::LoginAcknowledged(_) => {
                    Ok(Some(ServerboundEventEnum::Login(ServerboundLoginEvent::AcknowledgeLogin)))
                }
                LoginServerboundPackets::CookieResponse(packet) => {
                    Ok(Some(ServerboundEventEnum::Login(ServerboundLoginEvent::CookieResponse(
                        packet.cookie,
                        packet.payload,
                    ))))
                }
            },

            VersionPacket::Config(config) => {
                match config {
                    ConfigServerboundPackets::ClientInformation(packet) => {
                        Ok(Some(ServerboundEventEnum::Config(
                            ServerboundConfigEvent::ClientInformation(packet.information),
                        )))
                    }
                    ConfigServerboundPackets::CookieResponse(packet) => {
                        Ok(Some(ServerboundEventEnum::Config(
                            ServerboundConfigEvent::CookieResponse(packet.cookie, packet.payload),
                        )))
                    }
                    ConfigServerboundPackets::CustomPayload(packet) => {
                        Ok(Some(ServerboundEventEnum::Config(
                            ServerboundConfigEvent::CustomPayload(packet.identifier, packet.buffer),
                        )))
                    }
                    ConfigServerboundPackets::FinishConfiguration(_) => Ok(Some(
                        ServerboundEventEnum::Config(ServerboundConfigEvent::AcknowledgeConfig),
                    )),
                    ConfigServerboundPackets::KeepAlive(packet) => Ok(Some(
                        ServerboundEventEnum::Config(ServerboundConfigEvent::KeepAlive(packet.id)),
                    )),
                    ConfigServerboundPackets::Pong(packet) => Ok(Some(
                        ServerboundEventEnum::Config(ServerboundConfigEvent::Pong(packet.id)),
                    )),
                    ConfigServerboundPackets::ResourcePack(_packet) => Ok(Some(
                        ServerboundEventEnum::Config(ServerboundConfigEvent::ResourcePackUpdate()),
                    )),
                    ConfigServerboundPackets::SelectKnownPacks(packet) => {
                        Ok(Some(ServerboundEventEnum::Config(
                            ServerboundConfigEvent::ResourcePackResponse(packet.selected),
                        )))
                    }
                    ConfigServerboundPackets::CustomClickAction(_packet) => Ok(Some(
                        ServerboundEventEnum::Config(ServerboundConfigEvent::DialogAction()),
                    )),
                    ConfigServerboundPackets::AcceptCodeOfConduct(_) => Ok(Some(
                        ServerboundEventEnum::Config(ServerboundConfigEvent::AcceptCodeOfConduct),
                    )),
                }
            }

            VersionPacket::Play(play) => match play {
                PlayServerboundPackets::AcceptTeleportation(_packet) => todo!(),
                PlayServerboundPackets::Attack(_packet) => todo!(),
                PlayServerboundPackets::BlockEntityTagQuery(_packet) => todo!(),
                PlayServerboundPackets::BundleItemSelected(_packet) => todo!(),
                PlayServerboundPackets::ChangeDifficulty(_packet) => todo!(),
                PlayServerboundPackets::ChangeGameMode(_packet) => todo!(),
                PlayServerboundPackets::ChatAck(_packet) => todo!(),
                PlayServerboundPackets::ChatCommand(_packet) => todo!(),
                PlayServerboundPackets::ChatCommandSigned(_packet) => todo!(),
                PlayServerboundPackets::Chat(_packet) => todo!(),
                PlayServerboundPackets::ChatSessionUpdate(_packet) => todo!(),
                PlayServerboundPackets::ChunkBatchReceived(_packet) => todo!(),
                PlayServerboundPackets::ClientCommand(_packet) => todo!(),
                PlayServerboundPackets::ClientTickEnd(_packet) => todo!(),
                PlayServerboundPackets::ClientInformation(_packet) => todo!(),
                PlayServerboundPackets::CommandSuggestion(_packet) => todo!(),
                PlayServerboundPackets::ConfigurationAcknowledged(_packet) => todo!(),
                PlayServerboundPackets::ContainerButtonClick(_packet) => todo!(),
                PlayServerboundPackets::ContainerClick(_packet) => todo!(),
                PlayServerboundPackets::ContainerClose(_packet) => todo!(),
                PlayServerboundPackets::ContainerSlotStateChanged(_packet) => todo!(),
                PlayServerboundPackets::CookieResponse(_packet) => todo!(),
                PlayServerboundPackets::CustomPayload(_packet) => todo!(),
                PlayServerboundPackets::DebugSubscriptionRequest(_packet) => todo!(),
                PlayServerboundPackets::EditBook(_packet) => todo!(),
                PlayServerboundPackets::EntityTagQuery(_packet) => todo!(),
                PlayServerboundPackets::Interact(_packet) => todo!(),
                PlayServerboundPackets::JigsawGenerate(_packet) => todo!(),
                PlayServerboundPackets::KeepAlive(packet) => {
                    Ok(Some(ServerboundEventEnum::Play(ServerboundPlayEvent::KeepAlive(packet.id))))
                }
                PlayServerboundPackets::LockDifficulty(_packet) => todo!(),
                PlayServerboundPackets::MovePlayerPos(_packet) => todo!(),
                PlayServerboundPackets::MovePlayerPosRot(_packet) => todo!(),
                PlayServerboundPackets::MovePlayerRot(_packet) => todo!(),
                PlayServerboundPackets::MovePlayerStatusOnly(_packet) => todo!(),
                PlayServerboundPackets::MoveVehicle(_packet) => todo!(),
                PlayServerboundPackets::PaddleBoat(_packet) => todo!(),
                PlayServerboundPackets::PickItemFromBlock(_packet) => todo!(),
                PlayServerboundPackets::PickItemFromEntity(_packet) => todo!(),
                PlayServerboundPackets::PingRequest(packet) => Ok(Some(
                    ServerboundEventEnum::Play(ServerboundPlayEvent::PingRequest(packet.id)),
                )),
                PlayServerboundPackets::PlaceRecipe(_packet) => todo!(),
                PlayServerboundPackets::PlayerAbilities(_packet) => todo!(),
                PlayServerboundPackets::PlayerAction(_packet) => todo!(),
                PlayServerboundPackets::PlayerCommand(_packet) => todo!(),
                PlayServerboundPackets::PlayerInput(_packet) => todo!(),
                PlayServerboundPackets::PlayerLoaded(_packet) => todo!(),
                PlayServerboundPackets::Pong(packet) => {
                    Ok(Some(ServerboundEventEnum::Play(ServerboundPlayEvent::Pong(packet.id))))
                }
                PlayServerboundPackets::RecipeBookChangeSettings(_packet) => todo!(),
                PlayServerboundPackets::RecipeBookSeenRecipe(_packet) => todo!(),
                PlayServerboundPackets::RenameItem(_packet) => todo!(),
                PlayServerboundPackets::ResourcePack(_packet) => todo!(),
                PlayServerboundPackets::SeenAdvancements(_packet) => todo!(),
                PlayServerboundPackets::SelectTrade(_packet) => todo!(),
                PlayServerboundPackets::SetBeacon(_packet) => todo!(),
                PlayServerboundPackets::SetCarriedItem(_packet) => todo!(),
                PlayServerboundPackets::SetCommandBlock(_packet) => todo!(),
                PlayServerboundPackets::SetCommandMinecart(_packet) => todo!(),
                PlayServerboundPackets::SetCreativeModeSlot(_packet) => todo!(),
                PlayServerboundPackets::SetGameRule(_packet) => todo!(),
                PlayServerboundPackets::SetJigsawBlock(_packet) => todo!(),
                PlayServerboundPackets::SetStructureBlock(_packet) => todo!(),
                PlayServerboundPackets::SetTestBlock(_packet) => todo!(),
                PlayServerboundPackets::SignUpdate(_packet) => todo!(),
                PlayServerboundPackets::SpectateEntity(_packet) => todo!(),
                PlayServerboundPackets::Swing(_packet) => todo!(),
                PlayServerboundPackets::TeleportToEntity(_packet) => todo!(),
                PlayServerboundPackets::TestInstanceBlockAction(_packet) => todo!(),
                PlayServerboundPackets::UseItemOn(_packet) => todo!(),
                PlayServerboundPackets::UseItem(_packet) => todo!(),
                PlayServerboundPackets::CustomClickAction(_packet) => todo!(),
            },
        }
    }
}
