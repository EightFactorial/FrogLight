//! TODO
#![expect(missing_docs)]

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{From, TryInto, TryUnwrap};

pub use crate::v1_21_4::play::{QueryPingC2SPacket, PingResultS2CPacket};

pub(super) mod c2s_0x00_status_request;
pub use c2s_0x00_status_request::QueryRequestC2SPacket;

pub(super) mod s2c_0x00_status_response;
pub use s2c_0x00_status_response::QueryResponseS2CPacket;


#[repr(u8)]
#[derive(Debug, Clone, PartialEq, From, TryInto, TryUnwrap)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogPackets))]
pub enum ClientboundStatusPackets {
    QueryRequest(QueryRequestC2SPacket) = 0x00,
    QueryPing(QueryPingC2SPacket) = 0x01,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, From, TryInto, TryUnwrap)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogPackets))]
pub enum ServerboundStatusPackets {
    QueryResponse(QueryResponseS2CPacket) = 0x00,
    PingResult(PingResultS2CPacket) = 0x01,
}
