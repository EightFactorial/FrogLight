//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:intention"

use core::ops::{Deref, DerefMut};

use crate::common::handshake::HandshakeContent;

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct IntentionC2SPacket(pub HandshakeContent);

impl IntentionC2SPacket {
    /// Create a new [`IntentionC2SPacket`].
    #[inline]
    #[must_use]
    pub const fn new(content: HandshakeContent) -> Self { Self(content) }
}

// -------------------------------------------------------------------------------------------------

impl AsRef<HandshakeContent> for IntentionC2SPacket {
    #[inline]
    fn as_ref(&self) -> &HandshakeContent { &self.0 }
}
impl AsMut<HandshakeContent> for IntentionC2SPacket {
    #[inline]
    fn as_mut(&mut self) -> &mut HandshakeContent { &mut self.0 }
}

impl Deref for IntentionC2SPacket {
    type Target = HandshakeContent;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for IntentionC2SPacket {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl From<HandshakeContent> for IntentionC2SPacket {
    fn from(value: HandshakeContent) -> Self { Self(value) }
}
impl From<IntentionC2SPacket> for HandshakeContent {
    fn from(value: IntentionC2SPacket) -> Self { value.0 }
}
