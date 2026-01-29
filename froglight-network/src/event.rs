#![expect(missing_docs, reason = "WIP")]

#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
use froglight_packet::common::handshake::HandshakeContent;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub enum ClientboundEventEnum {
    Play(ClientboundPlayEvent),
    Config(ClientboundConfigEvent),
    Login(ClientboundLoginEvent),
    Status(ClientboundStatusEvent),
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub enum ClientboundPlayEvent {
    Placeholder,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub enum ServerboundPlayEvent {
    Placeholder,
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub enum ClientboundConfigEvent {
    Placeholder,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub enum ServerboundConfigEvent {
    Placeholder,
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub enum ClientboundLoginEvent {
    Placeholder,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub enum ServerboundLoginEvent {
    Placeholder,
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub enum ClientboundStatusEvent {
    Placeholder,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub enum ServerboundStatusEvent {
    RequestStatus,
    RequestPing,
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub enum ServerboundHandshakeEvent {
    Handshake(HandshakeContent),
}
