//! @generated [`Status`](crate::version::Status) packets for v26.1

mod s2c_0x00_status_response;
pub use s2c_0x00_status_response::StatusResponseS2CPacket;

mod s2c_0x01_pong_response;
pub use s2c_0x01_pong_response::PongResponseS2CPacket;

mod c2s_0x00_status_request;
pub use c2s_0x00_status_request::StatusRequestC2SPacket;

mod c2s_0x01_ping_request;
pub use c2s_0x01_ping_request::PingRequestC2SPacket;

#[repr(u8)]
#[cfg(feature = "v26_1")]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum ClientboundPackets {
    StatusResponse(StatusResponseS2CPacket) = 0x00,
    PongResponse(PongResponseS2CPacket) = 0x01,
}

#[repr(u8)]
#[cfg(feature = "v26_1")]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum ServerboundPackets {
    StatusRequest(StatusRequestC2SPacket) = 0x00,
    PingRequest(PingRequestC2SPacket) = 0x01,
}
