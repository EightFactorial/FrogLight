//! [`Status`](crate::states::Status) state packets for
//! [`V1_21_0`](super::V1_21_0)
#![allow(missing_docs)]

pub use super::play::{PingResultPacket, QueryPingPacket};

mod query_response;
pub use query_response::*;

mod query_request;
pub use query_request::*;

froglight_macros::frog_state! {
    Status,
    V1_21_0,
    Clientbound {
        0u32 => QueryResponsePacket,
        1u32 => PingResultPacket,
    },
    Serverbound {
        0u32 => QueryRequestPacket,
        1u32 => QueryPingPacket,
    },
}
