//! TODO
#![expect(missing_docs)]

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{From, TryInto, TryUnwrap};

pub use crate::v1_21_4::play::{TeleportConfirmC2SPacket, QueryBlockNbtC2SPacket, BundleItemSelectedC2SPacket, UpdateDifficultyC2SPacket, MessageAcknowledgmentC2SPacket, CommandExecutionC2SPacket, PlayerSessionC2SPacket, AcknowledgeChunksC2SPacket, ClientStatusC2SPacket, ClientTickEndC2SPacket, ClientOptionsC2SPacket, RequestCommandCompletionsC2SPacket, AcknowledgeReconfigurationC2SPacket, ButtonClickC2SPacket, CloseHandledScreenC2SPacket, SlotChangedStateC2SPacket, CookieResponseC2SPacket, CustomPayloadC2SPacket, DebugSampleSubscriptionC2SPacket, BookUpdateC2SPacket, QueryEntityNbtC2SPacket, PlayerInteractEntityC2SPacket, JigsawGeneratingC2SPacket, KeepAliveC2SPacket, UpdateDifficultyLockC2SPacket, PlayerPositionAndOnGroundC2SPacket, PlayerFullC2SPacket, PlayerLookAndOnGroundC2SPacket, PlayerOnGroundOnlyC2SPacket, VehicleMoveC2SPacket, BoatPaddleStateC2SPacket, PickItemFromBlockC2SPacket, PickItemFromEntityC2SPacket, QueryPingC2SPacket, CraftRequestC2SPacket, UpdatePlayerAbilitiesC2SPacket, PlayerActionC2SPacket, ClientCommandC2SPacket, PlayerInputC2SPacket, PlayerLoadedC2SPacket, CommonPongC2SPacket, RecipeCategoryOptionsC2SPacket, RecipeBookDataC2SPacket, RenameItemC2SPacket, ResourcePackStatusC2SPacket, AdvancementTabC2SPacket, SelectMerchantTradeC2SPacket, UpdateBeaconC2SPacket, UpdateSelectedSlotC2SPacket, UpdateCommandBlockC2SPacket, UpdateCommandBlockMinecartC2SPacket, CreativeInventoryActionC2SPacket, UpdateJigsawC2SPacket, UpdateSignC2SPacket, HandSwingC2SPacket, SpectatorTeleportC2SPacket, PlayerInteractBlockC2SPacket, PlayerInteractItemC2SPacket, BundleDelimiterS2CPacket, EntitySpawnS2CPacket, EntityAnimationS2CPacket, StatisticsS2CPacket, PlayerActionResponseS2CPacket, BlockBreakingProgressS2CPacket, BlockEntityUpdateS2CPacket, BlockEventS2CPacket, BlockUpdateS2CPacket, BossBarS2CPacket, DifficultyS2CPacket, ChunkSentS2CPacket, StartChunkSendS2CPacket, ChunkBiomeDataS2CPacket, ClearTitleS2CPacket, CommandSuggestionsS2CPacket, CommandTreeS2CPacket, CloseScreenS2CPacket, ScreenHandlerPropertyUpdateS2CPacket, ScreenHandlerSlotUpdateS2CPacket, CookieRequestS2CPacket, CooldownUpdateS2CPacket, ChatSuggestionsS2CPacket, CustomPayloadS2CPacket, EntityDamageS2CPacket, DebugSampleS2CPacket, RemoveMessageS2CPacket, DisconnectS2CPacket, ProfilelessChatMessageS2CPacket, EntityStatusS2CPacket, EntityPositionSyncS2CPacket, ExplosionS2CPacket, UnloadChunkS2CPacket, GameStateChangeS2CPacket, OpenHorseScreenS2CPacket, DamageTiltS2CPacket, WorldBorderInitializeS2CPacket, KeepAliveS2CPacket, ChunkDataS2CPacket, WorldEventS2CPacket, ParticleS2CPacket, LightUpdateS2CPacket, GameJoinS2CPacket, MapUpdateS2CPacket, SetTradeOffersS2CPacket, EntityMoveRelativeS2CPacket, EntityRotateAndMoveRelativeS2CPacket, MoveMinecartAlongTrackS2CPacket, EntityRotateS2CPacket, VehicleMoveS2CPacket, OpenWrittenBookS2CPacket, OpenScreenS2CPacket, SignEditorOpenS2CPacket, CommonPingS2CPacket, PingResultS2CPacket, CraftFailedResponseS2CPacket, PlayerAbilitiesS2CPacket, EndCombatS2CPacket, EnterCombatS2CPacket, DeathMessageS2CPacket, PlayerRemoveS2CPacket, PlayerListS2CPacket, LookAtS2CPacket, PlayerPositionLookS2CPacket, PlayerRotationS2CPacket, RecipeBookAddS2CPacket, RecipeBookRemoveS2CPacket, RecipeBookSettingsS2CPacket, EntitiesDestroyS2CPacket, RemoveEntityStatusEffectS2CPacket, ScoreboardScoreResetS2CPacket, ResourcePackRemoveS2CPacket, ResourcePackSendS2CPacket, PlayerRespawnS2CPacket, EntitySetHeadYawS2CPacket, ChunkDeltaUpdateS2CPacket, SelectAdvancementTabS2CPacket, ServerMetadataS2CPacket, OverlayMessageS2CPacket, WorldBorderCenterChangedS2CPacket, WorldBorderInterpolateSizeS2CPacket, WorldBorderSizeChangedS2CPacket, WorldBorderWarningTimeChangedS2CPacket, WorldBorderWarningBlocksChangedS2CPacket, SetCameraEntityS2CPacket, ChunkRenderDistanceCenterS2CPacket, ChunkLoadDistanceS2CPacket, SetCursorItemS2CPacket, PlayerSpawnPositionS2CPacket, ScoreboardDisplayS2CPacket, EntityTrackerUpdateS2CPacket, EntityAttachS2CPacket, EntityVelocityUpdateS2CPacket, EntityEquipmentUpdateS2CPacket, ExperienceBarUpdateS2CPacket, HealthUpdateS2CPacket, UpdateSelectedSlotS2CPacket, ScoreboardObjectiveUpdateS2CPacket, EntityPassengersSetS2CPacket, SetPlayerInventoryS2CPacket, TeamS2CPacket, ScoreboardScoreUpdateS2CPacket, SimulationDistanceS2CPacket, SubtitleS2CPacket, WorldTimeUpdateS2CPacket, TitleS2CPacket, TitleFadeS2CPacket, PlaySoundFromEntityS2CPacket, PlaySoundS2CPacket, EnterReconfigurationS2CPacket, StopSoundS2CPacket, StoreCookieS2CPacket, GameMessageS2CPacket, PlayerListHeaderS2CPacket, NbtQueryResponseS2CPacket, ItemPickupAnimationS2CPacket, EntityPositionS2CPacket, UpdateTickRateS2CPacket, TickStepS2CPacket, ServerTransferS2CPacket, EntityAttributesS2CPacket, EntityStatusEffectS2CPacket, SynchronizeRecipesS2CPacket, SynchronizeTagsS2CPacket, ProjectilePowerS2CPacket, CustomReportDetailsS2CPacket, ServerLinksS2CPacket};

pub(super) mod c2s_0x06_chat_command_signed;
pub use c2s_0x06_chat_command_signed::ChatCommandSignedC2SPacket;

pub(super) mod c2s_0x07_chat;
pub use c2s_0x07_chat::ChatMessageC2SPacket;

pub(super) mod c2s_0x10_container_click;
pub use c2s_0x10_container_click::ClickSlotC2SPacket;

pub(super) mod c2s_0x38_set_structure_block;
pub use c2s_0x38_set_structure_block::UpdateStructureBlockC2SPacket;

pub(super) mod c2s_0x39_set_test_block;
pub use c2s_0x39_set_test_block::SetTestBlockC2SPacket;

pub(super) mod c2s_0x3d_test_instance_block_action;
pub use c2s_0x3d_test_instance_block_action::TestInstanceBlockActionC2SPacket;

pub(super) mod s2c_0x12_container_set_content;
pub use s2c_0x12_container_set_content::InventoryS2CPacket;

pub(super) mod s2c_0x3a_player_chat;
pub use s2c_0x3a_player_chat::ChatMessageS2CPacket;

pub(super) mod s2c_0x77_test_instance_block_status;
pub use s2c_0x77_test_instance_block_status::TestInstanceBlockStatusS2CPacket;

pub(super) mod s2c_0x7b_update_advancements;
pub use s2c_0x7b_update_advancements::AdvancementUpdateS2CPacket;


#[repr(u8)]
#[derive(Debug, Clone, PartialEq, From, TryInto, TryUnwrap)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogPackets))]
pub enum ClientboundPlayPackets {
    TeleportConfirm(TeleportConfirmC2SPacket) = 0x00,
    QueryBlockNbt(QueryBlockNbtC2SPacket) = 0x01,
    BundleItemSelected(BundleItemSelectedC2SPacket) = 0x02,
    UpdateDifficulty(UpdateDifficultyC2SPacket) = 0x03,
    MessageAcknowledgment(MessageAcknowledgmentC2SPacket) = 0x04,
    CommandExecution(CommandExecutionC2SPacket) = 0x05,
    ChatCommandSigned(ChatCommandSignedC2SPacket) = 0x06,
    ChatMessage(ChatMessageC2SPacket) = 0x07,
    PlayerSession(PlayerSessionC2SPacket) = 0x08,
    AcknowledgeChunks(AcknowledgeChunksC2SPacket) = 0x09,
    ClientStatus(ClientStatusC2SPacket) = 0x0a,
    ClientTickEnd(ClientTickEndC2SPacket) = 0x0b,
    ClientOptions(ClientOptionsC2SPacket) = 0x0c,
    RequestCommandCompletions(RequestCommandCompletionsC2SPacket) = 0x0d,
    AcknowledgeReconfiguration(AcknowledgeReconfigurationC2SPacket) = 0x0e,
    ButtonClick(ButtonClickC2SPacket) = 0x0f,
    ClickSlot(ClickSlotC2SPacket) = 0x10,
    CloseHandledScreen(CloseHandledScreenC2SPacket) = 0x11,
    SlotChangedState(SlotChangedStateC2SPacket) = 0x12,
    CookieResponse(CookieResponseC2SPacket) = 0x13,
    CustomPayload(CustomPayloadC2SPacket) = 0x14,
    DebugSampleSubscription(DebugSampleSubscriptionC2SPacket) = 0x15,
    BookUpdate(BookUpdateC2SPacket) = 0x16,
    QueryEntityNbt(QueryEntityNbtC2SPacket) = 0x17,
    PlayerInteractEntity(PlayerInteractEntityC2SPacket) = 0x18,
    JigsawGenerating(JigsawGeneratingC2SPacket) = 0x19,
    KeepAlive(KeepAliveC2SPacket) = 0x1a,
    UpdateDifficultyLock(UpdateDifficultyLockC2SPacket) = 0x1b,
    PlayerPositionAndOnGround(PlayerPositionAndOnGroundC2SPacket) = 0x1c,
    PlayerFull(PlayerFullC2SPacket) = 0x1d,
    PlayerLookAndOnGround(PlayerLookAndOnGroundC2SPacket) = 0x1e,
    PlayerOnGroundOnly(PlayerOnGroundOnlyC2SPacket) = 0x1f,
    VehicleMove(VehicleMoveC2SPacket) = 0x20,
    BoatPaddleState(BoatPaddleStateC2SPacket) = 0x21,
    PickItemFromBlock(PickItemFromBlockC2SPacket) = 0x22,
    PickItemFromEntity(PickItemFromEntityC2SPacket) = 0x23,
    QueryPing(QueryPingC2SPacket) = 0x24,
    CraftRequest(CraftRequestC2SPacket) = 0x25,
    UpdatePlayerAbilities(UpdatePlayerAbilitiesC2SPacket) = 0x26,
    PlayerAction(PlayerActionC2SPacket) = 0x27,
    ClientCommand(ClientCommandC2SPacket) = 0x28,
    PlayerInput(PlayerInputC2SPacket) = 0x29,
    PlayerLoaded(PlayerLoadedC2SPacket) = 0x2a,
    CommonPong(CommonPongC2SPacket) = 0x2b,
    RecipeCategoryOptions(RecipeCategoryOptionsC2SPacket) = 0x2c,
    RecipeBookData(RecipeBookDataC2SPacket) = 0x2d,
    RenameItem(RenameItemC2SPacket) = 0x2e,
    ResourcePackStatus(ResourcePackStatusC2SPacket) = 0x2f,
    AdvancementTab(AdvancementTabC2SPacket) = 0x30,
    SelectMerchantTrade(SelectMerchantTradeC2SPacket) = 0x31,
    UpdateBeacon(UpdateBeaconC2SPacket) = 0x32,
    UpdateSelectedSlot(UpdateSelectedSlotC2SPacket) = 0x33,
    UpdateCommandBlock(UpdateCommandBlockC2SPacket) = 0x34,
    UpdateCommandBlockMinecart(UpdateCommandBlockMinecartC2SPacket) = 0x35,
    CreativeInventoryAction(CreativeInventoryActionC2SPacket) = 0x36,
    UpdateJigsaw(UpdateJigsawC2SPacket) = 0x37,
    UpdateStructureBlock(UpdateStructureBlockC2SPacket) = 0x38,
    SetTestBlock(SetTestBlockC2SPacket) = 0x39,
    UpdateSign(UpdateSignC2SPacket) = 0x3a,
    HandSwing(HandSwingC2SPacket) = 0x3b,
    SpectatorTeleport(SpectatorTeleportC2SPacket) = 0x3c,
    TestInstanceBlockAction(TestInstanceBlockActionC2SPacket) = 0x3d,
    PlayerInteractBlock(PlayerInteractBlockC2SPacket) = 0x3e,
    PlayerInteractItem(PlayerInteractItemC2SPacket) = 0x3f,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, From, TryInto, TryUnwrap)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogPackets))]
pub enum ServerboundPlayPackets {
    BundleDelimiter(BundleDelimiterS2CPacket) = 0x00,
    EntitySpawn(EntitySpawnS2CPacket) = 0x01,
    EntityAnimation(EntityAnimationS2CPacket) = 0x02,
    Statistics(StatisticsS2CPacket) = 0x03,
    PlayerActionResponse(PlayerActionResponseS2CPacket) = 0x04,
    BlockBreakingProgress(BlockBreakingProgressS2CPacket) = 0x05,
    BlockEntityUpdate(BlockEntityUpdateS2CPacket) = 0x06,
    BlockEvent(BlockEventS2CPacket) = 0x07,
    BlockUpdate(BlockUpdateS2CPacket) = 0x08,
    BossBar(BossBarS2CPacket) = 0x09,
    Difficulty(DifficultyS2CPacket) = 0x0a,
    ChunkSent(ChunkSentS2CPacket) = 0x0b,
    StartChunkSend(StartChunkSendS2CPacket) = 0x0c,
    ChunkBiomeData(ChunkBiomeDataS2CPacket) = 0x0d,
    ClearTitle(ClearTitleS2CPacket) = 0x0e,
    CommandSuggestions(CommandSuggestionsS2CPacket) = 0x0f,
    CommandTree(CommandTreeS2CPacket) = 0x10,
    CloseScreen(CloseScreenS2CPacket) = 0x11,
    Inventory(InventoryS2CPacket) = 0x12,
    ScreenHandlerPropertyUpdate(ScreenHandlerPropertyUpdateS2CPacket) = 0x13,
    ScreenHandlerSlotUpdate(ScreenHandlerSlotUpdateS2CPacket) = 0x14,
    CookieRequest(CookieRequestS2CPacket) = 0x15,
    CooldownUpdate(CooldownUpdateS2CPacket) = 0x16,
    ChatSuggestions(ChatSuggestionsS2CPacket) = 0x17,
    CustomPayload(CustomPayloadS2CPacket) = 0x18,
    EntityDamage(EntityDamageS2CPacket) = 0x19,
    DebugSample(DebugSampleS2CPacket) = 0x1a,
    RemoveMessage(RemoveMessageS2CPacket) = 0x1b,
    Disconnect(DisconnectS2CPacket) = 0x1c,
    ProfilelessChatMessage(ProfilelessChatMessageS2CPacket) = 0x1d,
    EntityStatus(EntityStatusS2CPacket) = 0x1e,
    EntityPositionSync(EntityPositionSyncS2CPacket) = 0x1f,
    Explosion(ExplosionS2CPacket) = 0x20,
    UnloadChunk(UnloadChunkS2CPacket) = 0x21,
    GameStateChange(GameStateChangeS2CPacket) = 0x22,
    OpenHorseScreen(OpenHorseScreenS2CPacket) = 0x23,
    DamageTilt(DamageTiltS2CPacket) = 0x24,
    WorldBorderInitialize(WorldBorderInitializeS2CPacket) = 0x25,
    KeepAlive(KeepAliveS2CPacket) = 0x26,
    ChunkData(ChunkDataS2CPacket) = 0x27,
    WorldEvent(WorldEventS2CPacket) = 0x28,
    Particle(ParticleS2CPacket) = 0x29,
    LightUpdate(LightUpdateS2CPacket) = 0x2a,
    GameJoin(GameJoinS2CPacket) = 0x2b,
    MapUpdate(MapUpdateS2CPacket) = 0x2c,
    SetTradeOffers(SetTradeOffersS2CPacket) = 0x2d,
    EntityMoveRelative(EntityMoveRelativeS2CPacket) = 0x2e,
    EntityRotateAndMoveRelative(EntityRotateAndMoveRelativeS2CPacket) = 0x2f,
    MoveMinecartAlongTrack(MoveMinecartAlongTrackS2CPacket) = 0x30,
    EntityRotate(EntityRotateS2CPacket) = 0x31,
    VehicleMove(VehicleMoveS2CPacket) = 0x32,
    OpenWrittenBook(OpenWrittenBookS2CPacket) = 0x33,
    OpenScreen(OpenScreenS2CPacket) = 0x34,
    SignEditorOpen(SignEditorOpenS2CPacket) = 0x35,
    CommonPing(CommonPingS2CPacket) = 0x36,
    PingResult(PingResultS2CPacket) = 0x37,
    CraftFailedResponse(CraftFailedResponseS2CPacket) = 0x38,
    PlayerAbilities(PlayerAbilitiesS2CPacket) = 0x39,
    ChatMessage(ChatMessageS2CPacket) = 0x3a,
    EndCombat(EndCombatS2CPacket) = 0x3b,
    EnterCombat(EnterCombatS2CPacket) = 0x3c,
    DeathMessage(DeathMessageS2CPacket) = 0x3d,
    PlayerRemove(PlayerRemoveS2CPacket) = 0x3e,
    PlayerList(PlayerListS2CPacket) = 0x3f,
    LookAt(LookAtS2CPacket) = 0x40,
    PlayerPositionLook(PlayerPositionLookS2CPacket) = 0x41,
    PlayerRotation(PlayerRotationS2CPacket) = 0x42,
    RecipeBookAdd(RecipeBookAddS2CPacket) = 0x43,
    RecipeBookRemove(RecipeBookRemoveS2CPacket) = 0x44,
    RecipeBookSettings(RecipeBookSettingsS2CPacket) = 0x45,
    EntitiesDestroy(EntitiesDestroyS2CPacket) = 0x46,
    RemoveEntityStatusEffect(RemoveEntityStatusEffectS2CPacket) = 0x47,
    ScoreboardScoreReset(ScoreboardScoreResetS2CPacket) = 0x48,
    ResourcePackRemove(ResourcePackRemoveS2CPacket) = 0x49,
    ResourcePackSend(ResourcePackSendS2CPacket) = 0x4a,
    PlayerRespawn(PlayerRespawnS2CPacket) = 0x4b,
    EntitySetHeadYaw(EntitySetHeadYawS2CPacket) = 0x4c,
    ChunkDeltaUpdate(ChunkDeltaUpdateS2CPacket) = 0x4d,
    SelectAdvancementTab(SelectAdvancementTabS2CPacket) = 0x4e,
    ServerMetadata(ServerMetadataS2CPacket) = 0x4f,
    OverlayMessage(OverlayMessageS2CPacket) = 0x50,
    WorldBorderCenterChanged(WorldBorderCenterChangedS2CPacket) = 0x51,
    WorldBorderInterpolateSize(WorldBorderInterpolateSizeS2CPacket) = 0x52,
    WorldBorderSizeChanged(WorldBorderSizeChangedS2CPacket) = 0x53,
    WorldBorderWarningTimeChanged(WorldBorderWarningTimeChangedS2CPacket) = 0x54,
    WorldBorderWarningBlocksChanged(WorldBorderWarningBlocksChangedS2CPacket) = 0x55,
    SetCameraEntity(SetCameraEntityS2CPacket) = 0x56,
    ChunkRenderDistanceCenter(ChunkRenderDistanceCenterS2CPacket) = 0x57,
    ChunkLoadDistance(ChunkLoadDistanceS2CPacket) = 0x58,
    SetCursorItem(SetCursorItemS2CPacket) = 0x59,
    PlayerSpawnPosition(PlayerSpawnPositionS2CPacket) = 0x5a,
    ScoreboardDisplay(ScoreboardDisplayS2CPacket) = 0x5b,
    EntityTrackerUpdate(EntityTrackerUpdateS2CPacket) = 0x5c,
    EntityAttach(EntityAttachS2CPacket) = 0x5d,
    EntityVelocityUpdate(EntityVelocityUpdateS2CPacket) = 0x5e,
    EntityEquipmentUpdate(EntityEquipmentUpdateS2CPacket) = 0x5f,
    ExperienceBarUpdate(ExperienceBarUpdateS2CPacket) = 0x60,
    HealthUpdate(HealthUpdateS2CPacket) = 0x61,
    UpdateSelectedSlot(UpdateSelectedSlotS2CPacket) = 0x62,
    ScoreboardObjectiveUpdate(ScoreboardObjectiveUpdateS2CPacket) = 0x63,
    EntityPassengersSet(EntityPassengersSetS2CPacket) = 0x64,
    SetPlayerInventory(SetPlayerInventoryS2CPacket) = 0x65,
    Team(TeamS2CPacket) = 0x66,
    ScoreboardScoreUpdate(ScoreboardScoreUpdateS2CPacket) = 0x67,
    SimulationDistance(SimulationDistanceS2CPacket) = 0x68,
    Subtitle(SubtitleS2CPacket) = 0x69,
    WorldTimeUpdate(WorldTimeUpdateS2CPacket) = 0x6a,
    Title(TitleS2CPacket) = 0x6b,
    TitleFade(TitleFadeS2CPacket) = 0x6c,
    PlaySoundFromEntity(PlaySoundFromEntityS2CPacket) = 0x6d,
    PlaySound(PlaySoundS2CPacket) = 0x6e,
    EnterReconfiguration(EnterReconfigurationS2CPacket) = 0x6f,
    StopSound(StopSoundS2CPacket) = 0x70,
    StoreCookie(StoreCookieS2CPacket) = 0x71,
    GameMessage(GameMessageS2CPacket) = 0x72,
    PlayerListHeader(PlayerListHeaderS2CPacket) = 0x73,
    NbtQueryResponse(NbtQueryResponseS2CPacket) = 0x74,
    ItemPickupAnimation(ItemPickupAnimationS2CPacket) = 0x75,
    EntityPosition(EntityPositionS2CPacket) = 0x76,
    TestInstanceBlockStatus(TestInstanceBlockStatusS2CPacket) = 0x77,
    UpdateTickRate(UpdateTickRateS2CPacket) = 0x78,
    TickStep(TickStepS2CPacket) = 0x79,
    ServerTransfer(ServerTransferS2CPacket) = 0x7a,
    AdvancementUpdate(AdvancementUpdateS2CPacket) = 0x7b,
    EntityAttributes(EntityAttributesS2CPacket) = 0x7c,
    EntityStatusEffect(EntityStatusEffectS2CPacket) = 0x7d,
    SynchronizeRecipes(SynchronizeRecipesS2CPacket) = 0x7e,
    SynchronizeTags(SynchronizeTagsS2CPacket) = 0x7f,
    ProjectilePower(ProjectilePowerS2CPacket) = 0x80,
    CustomReportDetails(CustomReportDetailsS2CPacket) = 0x81,
    ServerLinks(ServerLinksS2CPacket) = 0x82,
}
