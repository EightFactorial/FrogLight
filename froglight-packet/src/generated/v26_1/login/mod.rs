//! @generated [`Login`](crate::version::Login) packets for v26.1

mod s2c_0x00_login_disconnect;
pub use s2c_0x00_login_disconnect::LoginDisconnectS2CPacket;

mod s2c_0x01_hello;
pub use s2c_0x01_hello::HelloS2CPacket;

mod s2c_0x02_login_finished;
pub use s2c_0x02_login_finished::LoginFinishedS2CPacket;

mod s2c_0x03_login_compression;
pub use s2c_0x03_login_compression::LoginCompressionS2CPacket;

mod s2c_0x04_custom_query;
pub use s2c_0x04_custom_query::CustomQueryS2CPacket;

mod s2c_0x05_cookie_request;
pub use s2c_0x05_cookie_request::CookieRequestS2CPacket;

mod c2s_0x00_hello;
pub use c2s_0x00_hello::HelloC2SPacket;

mod c2s_0x01_key;
pub use c2s_0x01_key::KeyC2SPacket;

mod c2s_0x02_custom_query_answer;
pub use c2s_0x02_custom_query_answer::CustomQueryAnswerC2SPacket;

mod c2s_0x03_login_acknowledged;
pub use c2s_0x03_login_acknowledged::LoginAcknowledgedC2SPacket;

mod c2s_0x04_cookie_response;
pub use c2s_0x04_cookie_response::CookieResponseC2SPacket;

#[repr(u8)]
#[cfg(feature = "v26_1")]
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
#[cfg(feature = "v26_1")]
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
