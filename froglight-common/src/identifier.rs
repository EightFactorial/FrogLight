//! TODO
#![allow(
    clippy::unsafe_derive_deserialize,
    reason = "Allowed, as while important, it does not cause undefined behavior"
)]

use alloc::{borrow::Cow, string::String};
use core::{
    borrow::Borrow,
    cmp::Ordering,
    error::Error,
    fmt::{self},
    hash::{Hash, Hasher},
    ops::Deref,
};

#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
#[cfg(feature = "facet")]
use facet::Facet;
#[cfg(feature = "facet")]
use froglight_facet::{self as mc, facet::prelude::*};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An identifier.
#[repr(transparent)]
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(Facet))]
#[cfg_attr(feature = "facet", facet(opaque, mc::with = Identifier::WITH_BORROW))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct Identifier<'a> {
    inner: Cow<'a, str>,
}

impl Identifier<'_> {
    /// The default namespace to use when one is not provided.
    pub const DEFAULT_NAMESPACE: &'static str = "minecraft";

    /// Create a new static [`Identifier`].
    ///
    /// # Panics
    ///
    /// This will panic if the string is not a valid identifier.
    #[must_use]
    pub const fn new_static(s: &'static str) -> Identifier<'static> {
        Identifier { inner: Cow::Borrowed(s) }
    }

    /// Try to create a new [`Identifier`] from a string slice.
    ///
    /// # Note
    ///
    /// If the `alloc` feature is enabled and a string without a namespace is
    /// provided, one will be automatically added.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not a valid identifier.
    pub fn try_new<T: AsRef<str> + ?Sized>(s: &T) -> Result<Identifier<'_>, IdentifierError> {
        match Self::validate_string(s.as_ref()) {
            Ok(()) => {
                // SAFETY: We just checked that `s` is valid.
                Ok(unsafe { Self::new_unchecked(s.as_ref()) })
            }
            Err(IdentifierError::RequiresNamespace) => {
                // SAFETY: We know that `s` is valid besides the missing namespace.
                let appended = alloc::format!("{}:{}", Self::DEFAULT_NAMESPACE, s.as_ref());
                Ok(unsafe { Self::new_owned_unchecked(appended) })
            }
            Err(err) => Err(err),
        }
    }

    /// Try to create an owned [`Identifier`] from a string slice.
    ///
    /// # Note
    ///
    /// If the `alloc` feature is enabled and a string without a namespace is
    /// provided, one will be automatically added.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not a valid identifier.
    pub fn try_new_owned<T: AsRef<str> + ?Sized>(
        s: &T,
    ) -> Result<Identifier<'static>, IdentifierError> {
        let val = Self::try_new::<T>(s)?;
        Ok(val.into_owned())
    }

    /// Try to create a new owned [`Identifier`] from a string.
    ///
    /// # Note
    ///
    /// If a string without a namespace is provided, one will be automatically
    /// added.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not a valid identifier.
    pub fn try_new_string(s: String) -> Result<Self, IdentifierError> {
        match Self::validate_string(s.as_str()) {
            Ok(()) => {
                // SAFETY: We just checked that `s` is valid.
                Ok(unsafe { Self::new_owned_unchecked(s) })
            }
            Err(IdentifierError::RequiresNamespace) => {
                // SAFETY: We know that `s` is not empty, so we can prepend a namespace.
                Ok(unsafe {
                    Self::new_owned_unchecked(alloc::format!("{}:{s}", Self::DEFAULT_NAMESPACE))
                })
            }
            Err(err) => Err(err),
        }
    }

    /// Convert this [`Identifier`] into an owned version.
    #[must_use]
    pub fn into_owned(self) -> Identifier<'static> {
        Identifier { inner: Cow::Owned(self.inner.into_owned()) }
    }

    /// Returns `Ok(())` if the given string is a valid identifier.
    ///
    /// # Errors
    ///
    /// Returns an error if the string:
    /// - Is empty.
    /// - Starts with a namespace separator (`:`).
    /// - Ends with a namespace separator (`:`).
    /// - Contains more than one namespace separator (`:`).
    pub fn validate_string(str: &str) -> Result<(), IdentifierError> {
        if str.is_empty() {
            return Err(IdentifierError::Empty);
        }

        // Two parts, namespace and path
        let Some((namespace, path)) = str.split_once(':') else {
            return Err(IdentifierError::RequiresNamespace);
        };

        // Neither part must be empty, and there must be no more separators
        if namespace.is_empty() || path.is_empty() || path.contains(':') {
            return Err(IdentifierError::Invalid);
        }

        Ok(())
    }

    /// Get the namespace of this [`Identifier`].
    #[must_use]
    pub fn namespace(&self) -> &str { self.namespace_and_path().0 }

    /// Get the path of this [`Identifier`].
    #[must_use]
    pub fn path(&self) -> &str { self.namespace_and_path().1 }

    /// Get the namespace and path of this [`Identifier`] as a tuple.
    #[must_use]
    #[expect(
        clippy::missing_panics_doc,
        reason = "Should never panic, as it is unsafe to create an invalid identifier"
    )]
    pub fn namespace_and_path(&self) -> (&str, &str) {
        self.as_str().split_once(':').expect("Invalid identifier: missing namespace separator?!")
    }

    /// Reborrow this [`Identifier`] with a shorter lifetime.
    ///
    /// Useful for converting a reference into an owned identifier without
    /// cloning.
    #[must_use]
    pub const fn reborrow(&self) -> Identifier<'_> {
        match &self.inner {
            Cow::Borrowed(s) => Identifier { inner: Cow::Borrowed(s) },
            Cow::Owned(s) => Identifier { inner: Cow::Borrowed(s.as_str()) },
        }
    }

    /// Get the content of this [`Identifier`] as a string slice.
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match &self.inner {
            Cow::Borrowed(s) => s,
            Cow::Owned(s) => s.as_str(),
        }
    }

    /// A `const` method for comparing two identifiers for equality.
    ///
    /// Likely much slower than the standard [`PartialEq`]/[`Eq`]
    /// implementations, but usable in `const` contexts.
    #[must_use]
    pub const fn const_eq(&self, other: &Identifier<'_>) -> bool {
        let s1 = self.as_str().as_bytes();
        let s2 = other.as_str().as_bytes();
        // Short-circuit if lengths differ
        if s1.len() != s2.len() {
            return false;
        }
        // Compare byte by byte
        let mut i = 0;
        while i < s1.len() {
            if s1[i] != s2[i] {
                return false;
            }
            i += 1;
        }
        true
    }

    /// Create a new [`Identifier`] without checking its validity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided string is a valid identifier.
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(s: &str) -> Identifier<'_> {
        Identifier { inner: Cow::Borrowed(s) }
    }

    /// Create a new owned [`Identifier`] without checking its validity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided string is a valid identifier.
    #[inline]
    #[must_use]
    pub const unsafe fn new_owned_unchecked(s: String) -> Identifier<'static> {
        Identifier { inner: Cow::Owned(s) }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "facet")]
impl FacetTemplate for Identifier<'_> {
    #[allow(clippy::cast_possible_truncation, reason = "Ignored")]
    fn serialize(item: SerializeItem<'_, '_>, writer: &mut Writer<'_>) -> Result<(), WriterError> {
        let item = item.get::<Identifier<'_>>()?;
        encode_u32_into(item.as_str().len() as u32, writer)?;
        writer.write_bytes(item.as_str().as_bytes())
    }

    #[allow(clippy::cast_possible_truncation, reason = "Ignored")]
    fn deserialize<'facet, const BORROW: bool>(
        item: DeserializeItem<'facet, BORROW>,
        reader: &mut Reader<'_>,
    ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError> {
        let len = decode_u32_from(reader)?;

        let content = reader.read(len as usize)?;
        let content = str::from_utf8(content).map_err(ReaderError::other)?;
        let content = Identifier::try_new_owned(content).map_err(ReaderError::other)?;

        item.set(content)
    }
}

#[cfg(feature = "facet")]
impl FacetBorrowedTemplate for Identifier<'_> {
    #[allow(clippy::cast_possible_truncation, reason = "Ignored")]
    fn deserialize_borrowed<'facet>(
        item: DeserializeItem<'facet, true>,
        reader: &mut Reader<'facet>,
    ) -> Result<DeserializeItem<'facet, true>, ReaderError> {
        let len = decode_u32_from(reader)?;

        let content = reader.read(len as usize)?;
        let content = str::from_utf8(content).map_err(ReaderError::other)?;
        let content = Identifier::try_new(content).map_err(ReaderError::other)?;

        item.set(content)
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

impl fmt::Display for Identifier<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.as_str()) }
}

impl fmt::Debug for Identifier<'_> {
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
pub enum IdentifierError {
    /// The string is empty.
    Empty,
    /// The string has no namespace separator (`:`).
    RequiresNamespace,
    /// The string either starts with, ends with,
    /// or contains more than one namespace separator (`:`).
    Invalid,
}

impl fmt::Display for IdentifierError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result { todo!() }
}

impl fmt::Debug for IdentifierError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result { todo!() }
}

impl Error for IdentifierError {}
