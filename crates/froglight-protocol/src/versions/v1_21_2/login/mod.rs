//! [`Login`](crate::states::Login) state packets for
//! [`V1_21_2`](super::V1_21_2)
#![allow(missing_docs)]

pub use super::play::{
    CookieRequestPacket, CookieResponsePacket, DisconnectPacket as LoginDisconnectPacket,
};
pub use crate::versions::v1_21_0::login::{
    EnterConfigurationPacket, LoginCompressionPacket, LoginHelloC2SPacket, LoginHelloS2CPacket,
    LoginKeyPacket, LoginQueryRequestPacket, LoginQueryResponsePacket,
};

mod login_success;
pub use login_success::*;

froglight_macros::frog_state! {
    Login,
    V1_21_2,
    Clientbound {
        0u32 => LoginDisconnectPacket,
        1u32 => LoginHelloS2CPacket,
        2u32 => LoginSuccessPacket,
        3u32 => LoginCompressionPacket,
        4u32 => LoginQueryRequestPacket,
        5u32 => CookieRequestPacket,
    },
    Serverbound {
        0u32 => LoginHelloC2SPacket,
        1u32 => LoginKeyPacket,
        2u32 => LoginQueryResponsePacket,
        3u32 => EnterConfigurationPacket,
        4u32 => CookieResponsePacket,
    },
}
