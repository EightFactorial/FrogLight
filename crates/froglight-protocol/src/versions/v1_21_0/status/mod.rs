//! [`Status`](crate::states::Status) state packets for
//! [`V1_21_0`](super::V1_21_0)
#![allow(missing_docs)]

mod ping_result;
pub use ping_result::*;

mod query_response;
pub use query_response::*;

mod query_ping;
pub use query_ping::*;

mod query_request;
pub use query_request::*;

froglight_macros::frog_state! {
    Status,
    V1_21_0,
    Clientbound {
        0u32 => PingResultPacket,
        1u32 => QueryResponsePacket,
    },
    Serverbound {
        0u32 => QueryPingPacket,
        1u32 => QueryRequestPacket,
    },
}
