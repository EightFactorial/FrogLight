//! [`Status`](crate::states::Status) state packets for
//! [`V1_21_2`](super::V1_21_2)
#![allow(missing_docs)]

pub use super::play::{PingResultPacket, QueryPingPacket};
pub use crate::versions::v1_21_0::status::{QueryRequestPacket, QueryResponsePacket};

froglight_macros::frog_state! {
    Status,
    V1_21_2,
    Clientbound {
        0u32 => QueryResponsePacket,
        1u32 => PingResultPacket,
    },
    Serverbound {
        0u32 => QueryRequestPacket,
        1u32 => QueryPingPacket,
    },
}
