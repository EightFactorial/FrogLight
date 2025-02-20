use derive_more::Into;

use crate::{mutf8::Mutf8String, nbt::NbtCompound};

/// A named set of NBT tags.
#[derive(Debug, PartialEq, Into)]
pub struct NamedNbt(Mutf8String, NbtCompound);

impl NamedNbt {
    /// Create a new [`NamedNbt`] from a name and [`NbtCompound`].
    #[inline]
    #[must_use]
    pub const fn new(name: Mutf8String, compound: NbtCompound) -> Self { Self(name, compound) }

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
    pub fn compound(&self) -> &NbtCompound { &self.1 }

    /// Get the [`NbtCompound`] of the [`NamedNbt`] mutably.
    #[inline]
    #[must_use]
    pub fn compound_mut(&mut self) -> &mut NbtCompound { &mut self.1 }

    /// Get an [`UnnamedNbt`] from a [`NamedNbt`].
    ///
    /// # Safety
    /// This function is *probably* safe, but will be
    /// marked as unsafe until it is proven to be safe.
    #[must_use]
    pub unsafe fn as_unnamed(&self) -> &UnnamedNbt {
        // SAFETY: `UnnamedNbt` is a newtype over a `NbtCompound`
        unsafe { &*std::ptr::from_ref::<NbtCompound>(&self.1).cast::<UnnamedNbt>() }
    }

    /// Create an [`UnnamedNbt`] from this [`NamedNbt`].
    #[inline]
    #[must_use]
    pub fn into_unnamed(self) -> UnnamedNbt { UnnamedNbt::new(self.1) }
}

impl AsRef<NbtCompound> for NamedNbt {
    fn as_ref(&self) -> &NbtCompound { &self.1 }
}
impl AsMut<NbtCompound> for NamedNbt {
    fn as_mut(&mut self) -> &mut NbtCompound { &mut self.1 }
}

impl std::ops::Deref for NamedNbt {
    type Target = NbtCompound;
    fn deref(&self) -> &Self::Target { &self.1 }
}
impl std::ops::DerefMut for NamedNbt {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.1 }
}

// -------------------------------------------------------------------------------------------------

/// An unnamed set of NBT tags.
#[derive(Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
pub struct UnnamedNbt(NbtCompound);

impl UnnamedNbt {
    /// Create a new [`UnnamedNbt`] from a [`NbtCompound`].
    #[inline]
    #[must_use]
    pub fn new(compound: NbtCompound) -> Self { Self(compound) }

    /// Get the [`NbtCompound`] of the [`UnnamedNbt`].
    #[inline]
    #[must_use]
    pub fn compound(&self) -> &NbtCompound { &self.0 }

    /// Get the [`NbtCompound`] of the [`UnnamedNbt`] mutably.
    #[inline]
    #[must_use]
    pub fn compound_mut(&mut self) -> &mut NbtCompound { &mut self.0 }

    /// Get the inner [`NbtCompound`] of the [`UnnamedNbt`].
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> NbtCompound { self.0 }

    /// Create a new [`NamedNbt`] from an [`UnnamedNbt`].
    #[inline]
    #[must_use]
    pub fn into_named(self, name: impl Into<Mutf8String>) -> NamedNbt {
        NamedNbt::new(name.into(), self.0)
    }
}

impl AsRef<NbtCompound> for UnnamedNbt {
    fn as_ref(&self) -> &NbtCompound { &self.0 }
}
impl AsMut<NbtCompound> for UnnamedNbt {
    fn as_mut(&mut self) -> &mut NbtCompound { &mut self.0 }
}

impl std::ops::Deref for UnnamedNbt {
    type Target = NbtCompound;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl std::ops::DerefMut for UnnamedNbt {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl From<NamedNbt> for UnnamedNbt {
    fn from(named: NamedNbt) -> Self { named.into_unnamed() }
}
impl<T: Into<NbtCompound>> From<T> for UnnamedNbt {
    fn from(compound: T) -> Self { Self(compound.into()) }
}
