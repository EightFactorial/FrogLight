//! @generated [`Login`](crate::version::Login) packets for v26.2

pub use crate::generated::v26_1::login::LoginDisconnectS2CPacket;

pub use crate::generated::v26_1::login::HelloS2CPacket;

pub use crate::generated::v26_1::login::LoginFinishedS2CPacket;

pub use crate::generated::v26_1::login::LoginCompressionS2CPacket;

pub use crate::generated::v26_1::login::CustomQueryS2CPacket;

pub use crate::generated::v26_1::login::CookieRequestS2CPacket;

pub use crate::generated::v26_1::login::HelloC2SPacket;

pub use crate::generated::v26_1::login::KeyC2SPacket;

pub use crate::generated::v26_1::login::CustomQueryAnswerC2SPacket;

pub use crate::generated::v26_1::login::LoginAcknowledgedC2SPacket;

pub use crate::generated::v26_1::login::CookieResponseC2SPacket;

#[repr(u8)]
#[cfg(feature = "v26_2")]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum ClientboundPackets {
    LoginDisconnect(LoginDisconnectS2CPacket) = 0x00,
    Hello(HelloS2CPacket) = 0x01,
    LoginFinished(LoginFinishedS2CPacket) = 0x02,
    LoginCompression(LoginCompressionS2CPacket) = 0x03,
    CustomQuery(CustomQueryS2CPacket) = 0x04,
    CookieRequest(CookieRequestS2CPacket) = 0x05,
}

#[repr(u8)]
#[cfg(feature = "v26_2")]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum ServerboundPackets {
    Hello(HelloC2SPacket) = 0x00,
    Key(KeyC2SPacket) = 0x01,
    CustomQueryAnswer(CustomQueryAnswerC2SPacket) = 0x02,
    LoginAcknowledged(LoginAcknowledgedC2SPacket) = 0x03,
    CookieResponse(CookieResponseC2SPacket) = 0x04,
}
