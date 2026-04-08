//! @generated [`Status`](crate::version::Status) packets for v26.2

pub use crate::generated::v26_1::status::StatusResponseS2CPacket;

pub use crate::generated::v26_1::status::PongResponseS2CPacket;

pub use crate::generated::v26_1::status::StatusRequestC2SPacket;

pub use crate::generated::v26_1::status::PingRequestC2SPacket;

#[repr(u8)]
#[cfg(feature = "v26_2")]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum ClientboundPackets {
    StatusResponse(StatusResponseS2CPacket) = 0x00,
    PongResponse(PongResponseS2CPacket) = 0x01,
}

#[repr(u8)]
#[cfg(feature = "v26_2")]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum ServerboundPackets {
    StatusRequest(StatusRequestC2SPacket) = 0x00,
    PingRequest(PingRequestC2SPacket) = 0x01,
}
