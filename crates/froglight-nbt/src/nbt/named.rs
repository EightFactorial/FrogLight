use derive_more::{From, Into};

use crate::{mutf8::Mutf8String, nbt::NbtCompound};

/// A named set of NBT tags.
#[derive(Debug, PartialEq, From, Into)]
pub struct NamedNbt(Mutf8String, UnnamedNbt);

impl NamedNbt {
    /// Create a new [`NamedNbt`] from a name and [`NbtCompound`].
    #[inline]
    #[must_use]
    pub const fn new(name: Mutf8String, compound: NbtCompound) -> Self {
        Self(name, UnnamedNbt::new(compound))
    }

    /// Create a new [`NamedNbt`] from a name and optional [`NbtCompound`].
    #[inline]
    #[must_use]
    pub const fn new_from(name: Mutf8String, compound: Option<NbtCompound>) -> Self {
        Self(name, UnnamedNbt::new_from(compound))
    }

    /// Get the name of the [`NamedNbt`].
    #[inline]
    #[must_use]
    pub fn name(&self) -> &Mutf8String { &self.0 }

    /// Get the name of the [`NamedNbt`] mutably.
    #[inline]
    #[must_use]
    pub fn name_mut(&mut self) -> &mut Mutf8String { &mut self.0 }

    /// Get the [`NbtCompound`] of the [`NamedNbt`].
    #[inline]
    #[must_use]
    pub fn compound(&self) -> Option<&NbtCompound> { self.1.as_ref().as_ref() }

    /// Get the [`NbtCompound`] of the [`NamedNbt`] mutably.
    #[inline]
    #[must_use]
    pub fn compound_mut(&mut self) -> Option<&mut NbtCompound> { self.1.as_mut().as_mut() }

    /// Get an [`UnnamedNbt`] from a [`NamedNbt`].
    #[must_use]
    pub fn as_unnamed(&self) -> &UnnamedNbt { &self.1 }

    /// Create an [`UnnamedNbt`] from this [`NamedNbt`].
    #[inline]
    #[must_use]
    pub fn into_unnamed(self) -> UnnamedNbt { self.1 }
}

impl AsRef<Option<NbtCompound>> for NamedNbt {
    fn as_ref(&self) -> &Option<NbtCompound> { &self.1 }
}
impl AsMut<Option<NbtCompound>> for NamedNbt {
    fn as_mut(&mut self) -> &mut Option<NbtCompound> { &mut self.1 }
}

impl std::ops::Deref for NamedNbt {
    type Target = UnnamedNbt;
    fn deref(&self) -> &Self::Target { &self.1 }
}
impl std::ops::DerefMut for NamedNbt {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.1 }
}

// -------------------------------------------------------------------------------------------------

/// An unnamed set of NBT tags.
#[repr(transparent)]
#[derive(Debug, Default, PartialEq, From, Into)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
pub struct UnnamedNbt(Option<NbtCompound>);

impl UnnamedNbt {
    /// Create a new [`UnnamedNbt`] from a [`NbtCompound`].
    #[inline]
    #[must_use]
    pub const fn new(compound: NbtCompound) -> Self { Self::new_from(Some(compound)) }

    /// Create a new [`UnnamedNbt`] from an optional [`NbtCompound`].
    #[inline]
    #[must_use]
    pub const fn new_from(compound: Option<NbtCompound>) -> Self { Self(compound) }

    /// Get the [`NbtCompound`] of the [`UnnamedNbt`].
    #[inline]
    #[must_use]
    pub fn compound(&self) -> Option<&NbtCompound> { self.0.as_ref() }

    /// Get the [`NbtCompound`] of the [`UnnamedNbt`] mutably.
    #[inline]
    #[must_use]
    pub fn compound_mut(&mut self) -> Option<&mut NbtCompound> { self.0.as_mut() }

    /// Get the inner [`NbtCompound`] of the [`UnnamedNbt`].
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> Option<NbtCompound> { self.0 }

    /// Create a new [`NamedNbt`] from an [`UnnamedNbt`].
    #[inline]
    #[must_use]
    pub fn into_named(self, name: impl Into<Mutf8String>) -> NamedNbt {
        NamedNbt::new_from(name.into(), self.0)
    }
}

impl AsRef<Option<NbtCompound>> for UnnamedNbt {
    fn as_ref(&self) -> &Option<NbtCompound> { &self.0 }
}
impl AsMut<Option<NbtCompound>> for UnnamedNbt {
    fn as_mut(&mut self) -> &mut Option<NbtCompound> { &mut self.0 }
}

impl std::ops::Deref for UnnamedNbt {
    type Target = Option<NbtCompound>;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl std::ops::DerefMut for UnnamedNbt {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl From<NamedNbt> for UnnamedNbt {
    fn from(named: NamedNbt) -> Self { named.into_unnamed() }
}
impl<T: Into<NbtCompound>> From<T> for UnnamedNbt {
    fn from(compound: T) -> Self { Self::new(compound.into()) }
}

impl<T> FromIterator<T> for UnnamedNbt
where
    NbtCompound: FromIterator<T>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}
