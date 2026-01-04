//! TODO
#![allow(
    clippy::unsafe_derive_deserialize,
    reason = "Allowed, as while important, it does not cause undefined behavior"
)]

#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
use core::{
    borrow::Borrow,
    cmp::Ordering,
    error::Error,
    fmt::{self, Debug, Display},
    hash::{Hash, Hasher},
    ops::Deref,
};

#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
#[cfg(feature = "facet")]
use facet::Facet;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An identifier.
#[repr(transparent)]
#[derive(Clone)]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(Facet))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct Identifier<'a> {
    inner: Cow<'a, str>,
}

/// An identifier.
#[repr(transparent)]
#[derive(Clone)]
#[cfg(not(feature = "alloc"))]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(Facet))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct Identifier<'a> {
    inner: &'a str,
}

impl Identifier<'_> {
    /// Create a new static [`Identifier`].
    ///
    /// # Panics
    ///
    /// This will panic if the string is not a valid identifier.
    #[must_use]
    pub const fn new_static(s: &'static str) -> Identifier<'static> {
        #[cfg(feature = "alloc")]
        {
            Identifier { inner: Cow::Borrowed(s) }
        }
        #[cfg(not(feature = "alloc"))]
        {
            Identifier { inner: s }
        }
    }

    /// Try to create a new [`Identifier`] from a string slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not a valid identifier.
    pub fn try_new<T: AsRef<str> + ?Sized>(_s: &T) -> Result<Self, IdentifierError> { todo!() }

    /// Try to create an owned [`Identifier`] from a string slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not a valid identifier.
    #[cfg(feature = "alloc")]
    pub fn try_new_owned<T: AsRef<str> + ?Sized>(s: &T) -> Result<Self, IdentifierError> {
        Self::try_new::<T>(s).map(Self::into_owned)
    }

    /// Convert this [`Identifier`] into an owned version.
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn into_owned(self) -> Identifier<'static> {
        Identifier { inner: Cow::Owned(self.inner.into_owned()) }
    }

    /// Reborrow this [`Identifier`] with a shorter lifetime.
    ///
    /// Useful for converting a reference into an owned identifier without
    /// cloning.
    #[must_use]
    pub const fn reborrow(&self) -> Identifier<'_> {
        Identifier { inner: Cow::Borrowed(self.as_str()) }
    }

    /// Get the content of this [`Identifier`] as a string slice.
    #[must_use]
    pub const fn as_str(&self) -> &str {
        #[cfg(feature = "alloc")]
        {
            match &self.inner {
                Cow::Borrowed(s) => s,
                Cow::Owned(s) => s.as_str(),
            }
        }
        #[cfg(not(feature = "alloc"))]
        {
            self.inner
        }
    }

    /// Create a new [`Identifier`] without checking its validity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided string is a valid identifier.
    #[inline]
    #[must_use]
    pub const fn new_unchecked(s: &str) -> Identifier<'_> {
        #[cfg(feature = "alloc")]
        {
            Identifier { inner: Cow::Borrowed(s) }
        }
        #[cfg(not(feature = "alloc"))]
        {
            Identifier { inner: s }
        }
    }

    /// Create a new owned [`Identifier`] without checking its validity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided string is a valid identifier.
    #[inline]
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn new_owned_unchecked(s: alloc::string::String) -> Identifier<'static> {
        Identifier { inner: Cow::Owned(s) }
    }
}

// -------------------------------------------------------------------------------------------------

impl AsRef<str> for Identifier<'_> {
    fn as_ref(&self) -> &str { self.as_str() }
}
impl Borrow<str> for Identifier<'_> {
    fn borrow(&self) -> &str { self.as_str() }
}
impl Deref for Identifier<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target { self.as_str() }
}

impl Display for Identifier<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.as_str()) }
}

impl Debug for Identifier<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Identifier").field(&self.as_str()).finish()
    }
}

impl Eq for Identifier<'_> {}
impl PartialEq for Identifier<'_> {
    #[inline]
    fn eq(&self, other: &Self) -> bool { self.as_str() == other.as_str() }
}
impl PartialEq<str> for Identifier<'_> {
    #[inline]
    fn eq(&self, other: &str) -> bool { self.as_str() == other }
}
impl PartialEq<Identifier<'_>> for str {
    #[inline]
    fn eq(&self, other: &Identifier<'_>) -> bool { self == other.as_str() }
}

impl Ord for Identifier<'_> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering { self.as_str().cmp(other.as_str()) }
}
impl PartialOrd for Identifier<'_> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Hash for Identifier<'_> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) { self.as_str().hash(state); }
}

// -------------------------------------------------------------------------------------------------

/// An error that occurs when creating an [`Identifier`].
#[derive(Clone, Copy)]
pub enum IdentifierError {}

impl Display for IdentifierError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result { todo!() }
}

impl Debug for IdentifierError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result { todo!() }
}

impl Error for IdentifierError {}
