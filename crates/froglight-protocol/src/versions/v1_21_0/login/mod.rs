//! [`Login`](crate::states::Login) state packets for
//! [`V1_21_0`](super::V1_21_0)
#![allow(missing_docs)]

pub use super::play::{
    CookieRequestPacket, CookieResponsePacket, CustomPayloadC2SPacket as LoginQueryResponsePacket,
    CustomPayloadS2CPacket as LoginQueryRequestPacket, DisconnectPacket as LoginDisconnectPacket,
};

mod login_compression;
pub use login_compression::*;

mod enter_configuration;
pub use enter_configuration::*;

mod login_success;
pub use login_success::*;

mod login_hello_c2s;
pub use login_hello_c2s::*;

mod login_hello_s2c;
pub use login_hello_s2c::*;

mod login_key;
pub use login_key::*;

froglight_macros::frog_state! {
    Login,
    V1_21_0,
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
