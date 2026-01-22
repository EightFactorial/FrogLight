#![expect(missing_docs, reason = "WIP")]

#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub enum ClientboundEvent {
    Play(ClientboundPlayEvent),
    Config(ClientboundConfigEvent),
    Login(ClientboundLoginEvent),
    Status(ClientboundStatusEvent),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub enum ServerboundEvent {
    Play(ServerboundPlayEvent),
    Config(ServerboundConfigEvent),
    Login(ServerboundLoginEvent),
    Status(ServerboundStatusEvent),
}

impl From<ClientboundPlayEvent> for ClientboundEvent {
    fn from(value: ClientboundPlayEvent) -> Self { ClientboundEvent::Play(value) }
}
impl From<ClientboundConfigEvent> for ClientboundEvent {
    fn from(value: ClientboundConfigEvent) -> Self { ClientboundEvent::Config(value) }
}
impl From<ClientboundLoginEvent> for ClientboundEvent {
    fn from(value: ClientboundLoginEvent) -> Self { ClientboundEvent::Login(value) }
}
impl From<ClientboundStatusEvent> for ClientboundEvent {
    fn from(value: ClientboundStatusEvent) -> Self { ClientboundEvent::Status(value) }
}

impl From<ServerboundPlayEvent> for ServerboundEvent {
    fn from(value: ServerboundPlayEvent) -> Self { ServerboundEvent::Play(value) }
}
impl From<ServerboundConfigEvent> for ServerboundEvent {
    fn from(value: ServerboundConfigEvent) -> Self { ServerboundEvent::Config(value) }
}
impl From<ServerboundLoginEvent> for ServerboundEvent {
    fn from(value: ServerboundLoginEvent) -> Self { ServerboundEvent::Login(value) }
}
impl From<ServerboundStatusEvent> for ServerboundEvent {
    fn from(value: ServerboundStatusEvent) -> Self { ServerboundEvent::Status(value) }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub enum ClientboundPlayEvent {
    Placeholder,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub enum ServerboundPlayEvent {
    Placeholder,
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub enum ClientboundConfigEvent {
    Placeholder,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub enum ServerboundConfigEvent {
    Placeholder,
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub enum ClientboundLoginEvent {
    Placeholder,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub enum ServerboundLoginEvent {
    Placeholder,
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub enum ClientboundStatusEvent {
    Placeholder,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub enum ServerboundStatusEvent {
    RequestStatus,
    RequestPing,
}
