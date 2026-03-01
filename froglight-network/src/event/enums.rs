#![expect(missing_docs, reason = "WIP")]

#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
use facet::Facet;
use froglight_common::{entity::EntityId, prelude::Identifier};
use froglight_packet::common::{
    chunk_data::ChunkData,
    client_information::ClientInformation,
    entity_data::EntityData,
    handshake::HandshakeContent,
    known_packs::KnownResourcePack,
    login::{LoginHelloContent, PlayLoginContent},
    unsized_buffer::UnsizedBuffer,
};
use froglight_player::prelude::PlayerProfile;
use froglight_world::prelude::ChunkPos;

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Facet)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub enum ClientboundEventEnum {
    Play(ClientboundPlayEvent),
    Config(ClientboundConfigEvent),
    Login(ClientboundLoginEvent),
    Status(ClientboundStatusEvent),
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Facet)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub enum ServerboundEventEnum {
    Play(ServerboundPlayEvent),
    Config(ServerboundConfigEvent),
    Login(ServerboundLoginEvent),
    Status(ServerboundStatusEvent),
    Handshake(ServerboundHandshakeEvent),
}

impl From<ClientboundPlayEvent> for ClientboundEventEnum {
    fn from(value: ClientboundPlayEvent) -> Self { ClientboundEventEnum::Play(value) }
}
impl From<ClientboundConfigEvent> for ClientboundEventEnum {
    fn from(value: ClientboundConfigEvent) -> Self { ClientboundEventEnum::Config(value) }
}
impl From<ClientboundLoginEvent> for ClientboundEventEnum {
    fn from(value: ClientboundLoginEvent) -> Self { ClientboundEventEnum::Login(value) }
}
impl From<ClientboundStatusEvent> for ClientboundEventEnum {
    fn from(value: ClientboundStatusEvent) -> Self { ClientboundEventEnum::Status(value) }
}

impl From<ServerboundPlayEvent> for ServerboundEventEnum {
    fn from(value: ServerboundPlayEvent) -> Self { ServerboundEventEnum::Play(value) }
}
impl From<ServerboundConfigEvent> for ServerboundEventEnum {
    fn from(value: ServerboundConfigEvent) -> Self { ServerboundEventEnum::Config(value) }
}
impl From<ServerboundLoginEvent> for ServerboundEventEnum {
    fn from(value: ServerboundLoginEvent) -> Self { ServerboundEventEnum::Login(value) }
}
impl From<ServerboundStatusEvent> for ServerboundEventEnum {
    fn from(value: ServerboundStatusEvent) -> Self { ServerboundEventEnum::Status(value) }
}
impl From<ServerboundHandshakeEvent> for ServerboundEventEnum {
    fn from(value: ServerboundHandshakeEvent) -> Self { ServerboundEventEnum::Handshake(value) }
}

// -------------------------------------------------------------------------------------------------

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Facet)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub enum ClientboundPlayEvent {
    ActionBarText(),
    AddEntity(EntityData),
    Animate(),
    AwardStats(),
    BlockChangedAck(),
    BlockDestruction(),
    BlockEntityData(),
    BlockEvent(),
    BlockUpdate(),
    BossEvent(),
    BundleDelimiter,
    ChangeDifficulty(),
    ChatSuggestions(),
    ChunkBatchFinished(u32),
    ChunkBatchStart,
    ChunkBiomes(),
    ChunkCacheCenter(),
    ChunkCacheRadius(),
    ChunkSectionUpdate(),
    ChunkWithLight(ChunkPos, ChunkData, UnsizedBuffer<'static>),
    ClearDialog,
    ClearTitles(),
    CommandSuggestions(),
    Commands(),
    ContainerClose(),
    ContainerContent(),
    ContainerData(),
    ContainerSlot(),
    CookieRequest(),
    Cooldown(),
    CustomPayload(Identifier<'static>, UnsizedBuffer<'static>),
    CustomReportDetails(),
    DamageEvent(),
    DebugBlock(),
    DebugChunk(),
    DebugEntity(),
    DebugEvent(),
    DebugSample(),
    DeleteChat(),
    Disconnect(UnsizedBuffer<'static>),
    DisguisedChat(),
    DiskSpaceWarning(),
    EntityEvent(),
    EntityPosition(),
    Explode(),
    ForgetChunk(),
    GameEvent(),
    GameRule(),
    GameTestHighlight(),
    GhostRecipe(),
    HurtAnimation(),
    InitializeBorder(),
    KeepAlive(u64),
    LevelEvent(),
    LevelParticles(),
    LightUpdate(),
    Login(PlayLoginContent),
    MapItemData(),
    MerchantOffers(),
    MountScreen(),
    MoveEntityPos(),
    MoveEntityPosRot(),
    MoveEntityRot(),
    MoveMinecartTrack(),
    MoveVehicle(),
    OpenBook(),
    OpenScreen(),
    OpenSignEditor(),
    Ping(u32),
    PlayerAbilities(),
    PlayerChat(),
    PlayerCombatEnd(),
    PlayerCombatEnter(),
    PlayerCombatKill(),
    PlayerInfoRemove(),
    PlayerInfoUpdate(),
    PlayerLookAt(),
    PlayerPosition(),
    PlayerRotation(),
    Pong(u32),
    ProjectilePower(),
    RecipeBookAdd(),
    RecipeBookRemove(),
    RecipeBookSettings(),
    RemoveEntities(Vec<EntityId>),
    RemoveMobEffect(),
    ResetScore(),
    ResourcePackPop(),
    ResourcePackPush(),
    Respawn(),
    RotateHead(),
    SelectAdvancementTab(),
    ServerData(),
    ServerLinks(),
    SetBorderCenter(),
    SetBorderLerpSize(),
    SetBorderSize(),
    SetBorderWarningDelay(),
    SetBorderWarningDistance(),
    SetCamera(),
    SetCursorItem(),
    SetDefaultSpawn(),
    SetDisplayObjective(),
    SetEntityData(),
    SetEntityLink(),
    SetEntityMotion(),
    SetEquipment(),
    SetExperience(),
    SetHealth(),
    SetHeldSlot(),
    SetObjective(),
    SetPassengers(),
    SetPlayerInventory(),
    SetPlayerTeam(),
    SetScore(),
    SetSimulationDistance(),
    SetSubtitleText(),
    SetTime(),
    SetTitleAnimation(),
    SetTitleText(),
    ShowDialog(),
    Sound(),
    SoundEntity(),
    StartConfiguration,
    StopSound(),
    StoreCookie(),
    SystemChat(),
    TabList(),
    TagQuery(),
    TakeItemEntity(),
    TeleportEntity(),
    TestBlockStatus(),
    TickingState(),
    TickingStep(),
    Transfer(),
    UpdateAdvancements(),
    UpdateAttributes(),
    UpdateMobEffect(),
    UpdateRecipes(),
    UpdateTags(),
    Waypoint(),
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Facet)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub enum ServerboundPlayEvent {
    ChunkBatchReceived(f32),
    KeepAlive(u64),
    PingRequest(u32),
    Pong(u32),
}

// -------------------------------------------------------------------------------------------------

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Facet)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub enum ClientboundConfigEvent {
    ClearDialog,
    CodeOfConduct(),
    CookieRequest(Identifier<'static>),
    CustomPayload(Identifier<'static>, UnsizedBuffer<'static>),
    CustomReportDetails(),
    Disconnect(UnsizedBuffer<'static>),
    EnabledFeatures(),
    FinishConfig,
    KeepAlive(u64),
    KnownResourcePacks(Vec<KnownResourcePack>),
    Ping(u32),
    RegistryData(),
    ResetChat,
    ResourcePackPop(),
    ResourcePackPush(),
    ServerLinks(),
    ShowDialog(),
    StoreCookie(Identifier<'static>, Vec<u8>),
    Transfer(),
    UpdateTags(),
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Facet)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub enum ServerboundConfigEvent {
    AcceptCodeOfConduct,
    AcknowledgeConfig,
    ClientInformation(ClientInformation),
    CookieResponse(Identifier<'static>, Option<Vec<u8>>),
    CustomPayload(Identifier<'static>, UnsizedBuffer<'static>),
    DialogAction(),
    KeepAlive(u64),
    Pong(u32),
    ResourcePackResponse(Vec<KnownResourcePack>),
    ResourcePackUpdate(),
}

// -------------------------------------------------------------------------------------------------

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Facet)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub enum ClientboundLoginEvent {
    CompressionThreshold(u32),
    CookieRequest(Identifier<'static>),
    CustomPayload(u32, Identifier<'static>, UnsizedBuffer<'static>),
    Disconnect(String),
    EncryptionRequest(),
    LoginFinished(PlayerProfile),
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Facet)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub enum ServerboundLoginEvent {
    AcknowledgeLogin,
    CookieResponse(Identifier<'static>, Option<Vec<u8>>),
    CustomPayload(u32, Option<UnsizedBuffer<'static>>),
    EncryptionResponse(),
    Hello(LoginHelloContent),
}

// -------------------------------------------------------------------------------------------------

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Facet)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub enum ClientboundStatusEvent {
    Placeholder,
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Facet)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub enum ServerboundStatusEvent {
    RequestStatus,
    RequestPing,
}

// -------------------------------------------------------------------------------------------------

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Facet)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub enum ServerboundHandshakeEvent {
    Handshake(HandshakeContent),
}
