//! TODO
#![expect(missing_docs)]

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{From, TryInto, TryUnwrap};

pub use crate::v1_21_4::login::{LoginHelloC2SPacket, LoginKeyC2SPacket, LoginQueryResponseC2SPacket, EnterConfigurationC2SPacket, CookieResponseC2SPacket, LoginDisconnectS2CPacket, LoginHelloS2CPacket, LoginSuccessS2CPacket, LoginCompressionS2CPacket, LoginQueryRequestS2CPacket, CookieRequestS2CPacket};


#[repr(u8)]
#[derive(Debug, Clone, PartialEq, From, TryInto, TryUnwrap)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogPackets))]
pub enum ClientboundLoginPackets {
    LoginHello(LoginHelloC2SPacket) = 0x00,
    LoginKey(LoginKeyC2SPacket) = 0x01,
    LoginQueryResponse(LoginQueryResponseC2SPacket) = 0x02,
    EnterConfiguration(EnterConfigurationC2SPacket) = 0x03,
    CookieResponse(CookieResponseC2SPacket) = 0x04,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, From, TryInto, TryUnwrap)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogPackets))]
pub enum ServerboundLoginPackets {
    LoginDisconnect(LoginDisconnectS2CPacket) = 0x00,
    LoginHello(LoginHelloS2CPacket) = 0x01,
    LoginSuccess(LoginSuccessS2CPacket) = 0x02,
    LoginCompression(LoginCompressionS2CPacket) = 0x03,
    LoginQueryRequest(LoginQueryRequestS2CPacket) = 0x04,
    CookieRequest(CookieRequestS2CPacket) = 0x05,
}
