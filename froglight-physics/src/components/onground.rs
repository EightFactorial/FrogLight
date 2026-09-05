use core::ops::{Deref, DerefMut};

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};
#[cfg(all(feature = "bevy", feature = "serde"))]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};
#[cfg(feature = "serde")]
use froglight_common::crates::serde::{Deserialize, Serialize};

/// A OnGround vector.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
#[cfg_attr(feature = "bevy", require(PrevOnGround))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "froglight_common::crates::serde"))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct OnGround(pub bool);

impl OnGround {
    /// A constant for [`OnGround::new(false)`](OnGround::new).
    pub const FALSE: Self = Self(false);
    /// A constant for [`OnGround::new(true)`](OnGround::new).
    pub const TRUE: Self = Self(true);

    /// Create a new [`OnGround`] from a [`Vec3A`].
    #[inline]
    #[must_use]
    pub const fn new(on_ground: bool) -> Self { Self(on_ground) }

    /// Get the inner [`bool`] value.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> bool { self.0 }
}

impl AsRef<bool> for OnGround {
    #[inline]
    fn as_ref(&self) -> &bool { &self.0 }
}
impl AsMut<bool> for OnGround {
    #[inline]
    fn as_mut(&mut self) -> &mut bool { &mut self.0 }
}

impl Deref for OnGround {
    type Target = bool;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for OnGround {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

// -------------------------------------------------------------------------------------------------

/// The previous tick's [`OnGround`].
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "froglight_common::crates::serde"))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct PrevOnGround(OnGround);

impl PrevOnGround {
    /// Create a new [`PrevOnGround`] from a [`Vec3A`].
    #[inline]
    #[must_use]
    pub const fn new(on_ground: bool) -> Self { Self(OnGround::new(on_ground)) }

    /// Convert this [`PrevOnGround`] into an [`OnGround`].
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> OnGround { self.0 }
}

impl From<OnGround> for PrevOnGround {
    #[inline]
    fn from(on_ground: OnGround) -> Self { Self(on_ground) }
}

impl AsRef<OnGround> for PrevOnGround {
    #[inline]
    fn as_ref(&self) -> &OnGround { &self.0 }
}
impl AsMut<OnGround> for PrevOnGround {
    #[inline]
    fn as_mut(&mut self) -> &mut OnGround { &mut self.0 }
}

impl AsRef<bool> for PrevOnGround {
    #[inline]
    fn as_ref(&self) -> &bool { self.0.as_ref() }
}
impl AsMut<bool> for PrevOnGround {
    #[inline]
    fn as_mut(&mut self) -> &mut bool { self.0.as_mut() }
}

impl Deref for PrevOnGround {
    type Target = OnGround;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for PrevOnGround {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
