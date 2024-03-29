//! [`Status`](crate::states::Status) state packets for
//! [`V1_20_2`](super::V1_20_2)
//!
//! @generated by `froglight-generator #a28591a`
#![allow(missing_docs)]

use froglight_macros::frog_state;

pub use crate::versions::v1_20_0::status::{
    QueryPingC2SPacket, QueryPongS2CPacket as PingResultS2CPacket, QueryRequestC2SPacket,
    QueryResponseS2CPacket,
};

frog_state! {
    Status,
    V1_20_2,
    Clientbound {
        0u32 => QueryResponseS2CPacket,
        1u32 => PingResultS2CPacket,
    },
    Serverbound {
        0u32 => QueryRequestC2SPacket,
        1u32 => QueryPingC2SPacket,
    },
}
