use core::ops::{Deref, DerefMut};

use bevy_reflect::Reflect;
use uuid::Uuid;

/// An [`ArgumentParser`] that parses a [`u128`] as a [`Uuid`].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect)]
#[reflect(Debug, Clone, PartialEq, Hash)]
pub struct UuidInteger(pub Uuid);

impl From<UuidInteger> for Uuid {
    #[inline]
    fn from(value: UuidInteger) -> Self { value.0 }
}
impl From<Uuid> for UuidInteger {
    #[inline]
    fn from(value: Uuid) -> Self { Self(value) }
}

impl Deref for UuidInteger {
    type Target = Uuid;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for UuidInteger {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

// -------------------------------------------------------------------------------------------------

/// An [`ArgumentParser`] that parses a [`Simple`](uuid::fmt::Simple) [`Uuid`].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect)]
#[reflect(Debug, Clone, PartialEq, Hash)]
pub struct UuidSimple(pub Uuid);

impl From<UuidSimple> for Uuid {
    #[inline]
    fn from(value: UuidSimple) -> Self { value.0 }
}
impl From<Uuid> for UuidSimple {
    #[inline]
    fn from(value: Uuid) -> Self { Self(value) }
}

impl Deref for UuidSimple {
    type Target = Uuid;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for UuidSimple {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

// -------------------------------------------------------------------------------------------------

/// An [`ArgumentParser`] that parses a [`Braced`](uuid::fmt::Braced) [`Uuid`].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect)]
#[reflect(Debug, Clone, PartialEq, Hash)]
pub struct UuidBraced(pub Uuid);

impl From<UuidBraced> for Uuid {
    #[inline]
    fn from(value: UuidBraced) -> Self { value.0 }
}
impl From<Uuid> for UuidBraced {
    #[inline]
    fn from(value: Uuid) -> Self { Self(value) }
}

impl Deref for UuidBraced {
    type Target = Uuid;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for UuidBraced {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
