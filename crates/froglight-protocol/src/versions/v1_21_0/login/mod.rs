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
        0u32 => CookieRequestPacket,
        1u32 => LoginQueryRequestPacket,
        2u32 => LoginSuccessPacket,
        3u32 => LoginHelloS2CPacket,
        4u32 => LoginCompressionPacket,
        5u32 => LoginDisconnectPacket,
    },
    Serverbound {
        0u32 => CookieResponsePacket,
        1u32 => LoginQueryResponsePacket,
        2u32 => LoginHelloC2SPacket,
        3u32 => LoginKeyPacket,
        4u32 => EnterConfigurationPacket,
    },
}
