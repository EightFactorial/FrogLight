use core::ops::{Deref, DerefMut};

use crate::common::login::LoginHelloContent;

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct LoginHelloC2SPacket(pub LoginHelloContent);

impl LoginHelloC2SPacket {
    /// Create a new [`LoginHelloC2SPacket`].
    #[inline]
    #[must_use]
    pub const fn new(content: LoginHelloContent) -> Self { Self(content) }
}

// -------------------------------------------------------------------------------------------------

impl AsRef<LoginHelloContent> for LoginHelloC2SPacket {
    #[inline]
    fn as_ref(&self) -> &LoginHelloContent { &self.0 }
}
impl AsMut<LoginHelloContent> for LoginHelloC2SPacket {
    #[inline]
    fn as_mut(&mut self) -> &mut LoginHelloContent { &mut self.0 }
}

impl Deref for LoginHelloC2SPacket {
    type Target = LoginHelloContent;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for LoginHelloC2SPacket {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl From<LoginHelloContent> for LoginHelloC2SPacket {
    fn from(value: LoginHelloContent) -> Self { Self(value) }
}
impl From<LoginHelloC2SPacket> for LoginHelloContent {
    fn from(value: LoginHelloC2SPacket) -> Self { value.0 }
}
