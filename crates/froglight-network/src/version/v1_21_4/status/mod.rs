//! TODO
#![expect(missing_docs)]

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::From;
use froglight_macros::FrogPackets;

pub use super::play::{PingResultPacket, QueryPingPacket};

mod query_request;
pub use query_request::QueryRequestPacket;

mod query_response;
pub use query_response::QueryResponsePacket;

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, FrogPackets, From)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub enum ClientboundStatusPackets {
    QueryResponse(QueryResponsePacket) = 0x0,
    PingResult(PingResultPacket) = 0x1,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, FrogPackets, From)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub enum ServerboundStatusPackets {
    QueryRequest(QueryRequestPacket) = 0x0,
    QueryPing(QueryPingPacket) = 0x1,
}
