use core::net::SocketAddr;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_common::version::Version;
use smol_str::{SmolStr, ToSmolStr};

use crate::common::ConnectionIntent;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub struct HandshakePacket {
    #[cfg_attr(feature = "io", frog(var))]
    pub protocol: i32,
    pub address: SmolStr,
    pub port: u16,
    pub intent: ConnectionIntent,
}

impl HandshakePacket {
    /// Create a new [`HandshakePacket`] with the given
    /// address, port, and [ConnectionIntent].
    #[inline]
    #[must_use]
    #[expect(clippy::cast_possible_wrap)]
    pub fn new<V: Version>(address: impl ToSmolStr, port: u16, intent: ConnectionIntent) -> Self {
        Self { protocol: V::PROTOCOL_ID as i32, address: address.to_smolstr(), port, intent }
    }

    /// Create a new [`HandshakePacket`] with the given
    /// [`SocketAddr`] and [ConnectionIntent].
    #[inline]
    #[must_use]
    pub fn new_raw<V: Version>(socket: SocketAddr, intent: ConnectionIntent) -> Self {
        Self::new::<V>(socket.ip(), socket.port(), intent)
    }
}
