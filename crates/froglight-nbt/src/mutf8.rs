//! TODO

#[cfg(not(feature = "std"))]
use alloc::{
    borrow::{Cow, ToOwned},
    string::String,
    vec::Vec,
};
use core::borrow::Borrow;
#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use froglight_common::prelude::Identifier;
use indexmap::Equivalent;

/// A MUTF-8 string.
///
/// Equivalent to a [`String`], but with a different encoding.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
pub struct Mutf8String(Vec<u8>);

impl Mutf8String {
    /// Create a new [`Mutf8String`] from a byte slice.
    ///
    /// # Warning
    /// This function will always succeed, as it does not check if the
    /// input is valid MUTF-8.
    ///
    /// The contents will only be checked when converting
    /// into a [`String`] or [`str`].
    #[inline]
    #[must_use]
    pub const fn from_bytes(bytes: Vec<u8>) -> Self { Self(bytes) }

    /// Create a new [`Mutf8String`] from a [`String`].
    ///
    /// See [`simd_cesu8::encode`] for more information.
    #[inline]
    #[must_use]
    pub fn from_string(string: &str) -> Self {
        Self::from_bytes(simd_cesu8::encode(string).to_vec())
    }

    /// Get the inner bytes of a [`Mutf8String`].
    #[inline]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] { &self.0 }

    /// Returns the number of bytes in the [`Mutf8String`].
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize { self.0.len() }

    /// Returns `true` if the [`Mutf8String`] is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Returns `true` if the [`Mutf8String`] is valid.
    #[inline]
    #[must_use]
    pub fn is_valid(&self) -> bool { self.as_mutf8_str().is_valid() }

    /// Convert a [`Mutf8String`] to a [`Mutf8Str`].
    #[inline]
    #[must_use]
    pub fn as_mutf8_str(&self) -> &Mutf8Str { Mutf8Str::from_bytes(self.as_bytes()) }

    /// Convert a [`Mutf8String`] to a [`Cow<str>`], including invalid
    /// characters.
    ///
    /// See [`simd_cesu8::decode_lossy`] for more information.
    #[inline]
    #[must_use]
    pub fn to_str_lossy(&self) -> Cow<'_, str> { simd_cesu8::decode_lossy(self.as_bytes()) }

    /// Convert a [`Mutf8String`] to a [`String`], including invalid
    /// characters.
    ///
    /// See [`simd_cesu8::decode_lossy`] for more information.
    #[inline]
    #[must_use]
    pub fn to_string_lossy(&self) -> String { self.to_str_lossy().into_owned() }

    /// Convert a [`Mutf8String`] to a [`Cow<str>`].
    ///
    /// # Errors
    /// Returns an error if the MUTF-8 string is invalid.
    ///
    /// See [`simd_cesu8::decode`] for more information.
    #[inline]
    pub fn try_as_str(&self) -> Result<Cow<'_, str>, simd_cesu8::DecodingError> {
        simd_cesu8::decode(self.as_bytes())
    }

    /// Convert a [`Mutf8String`] to a [`String`].
    ///
    /// # Errors
    /// Returns an error if the MUTF-8 string is invalid.
    ///
    /// See [`simd_cesu8::decode`] for more information.
    #[inline]
    pub fn try_as_string(&self) -> Result<String, simd_cesu8::DecodingError> {
        self.try_as_str().map(Cow::into_owned)
    }
}

impl AsRef<[u8]> for Mutf8String {
    fn as_ref(&self) -> &[u8] { &self.0 }
}
impl Borrow<[u8]> for Mutf8String {
    fn borrow(&self) -> &[u8] { &self.0 }
}

impl AsRef<Mutf8Str> for Mutf8String {
    fn as_ref(&self) -> &Mutf8Str { self.as_mutf8_str() }
}
impl Borrow<Mutf8Str> for Mutf8String {
    fn borrow(&self) -> &Mutf8Str { self.as_mutf8_str() }
}

impl From<String> for Mutf8String {
    fn from(value: String) -> Self { Self::from_string(&value) }
}
impl<'a> From<&'a str> for Mutf8String {
    fn from(value: &'a str) -> Self { Self::from_string(value) }
}

impl TryFrom<Mutf8String> for String {
    type Error = simd_cesu8::DecodingError;

    fn try_from(value: Mutf8String) -> Result<Self, Self::Error> { value.try_as_string() }
}
impl TryFrom<Mutf8String> for Cow<'static, str> {
    type Error = simd_cesu8::DecodingError;

    fn try_from(value: Mutf8String) -> Result<Self, Self::Error> {
        Ok(Cow::Owned(value.try_as_string()?))
    }
}
impl TryFrom<Mutf8String> for Identifier {
    type Error = simd_cesu8::DecodingError;

    fn try_from(value: Mutf8String) -> Result<Self, Self::Error> {
        Ok(Identifier::new(value.try_as_str()?))
    }
}

impl From<Vec<u8>> for Mutf8String {
    fn from(value: Vec<u8>) -> Self { Self::from_bytes(value) }
}

impl PartialEq<Mutf8Str> for Mutf8String {
    fn eq(&self, other: &Mutf8Str) -> bool { self.as_bytes() == other.as_bytes() }
}
impl PartialEq<Mutf8Str> for &Mutf8String {
    fn eq(&self, other: &Mutf8Str) -> bool { self.as_bytes() == other.as_bytes() }
}

impl Equivalent<Mutf8Str> for Mutf8String {
    fn equivalent(&self, key: &Mutf8Str) -> bool { self.as_bytes() == key.as_bytes() }
}
impl Equivalent<str> for Mutf8String {
    fn equivalent(&self, key: &str) -> bool { self.as_bytes() == key.as_bytes() }
}
impl Equivalent<[u8]> for Mutf8String {
    fn equivalent(&self, key: &[u8]) -> bool { self.as_bytes() == key }
}

impl core::fmt::Debug for Mutf8String {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Mutf8String").field(&self.to_str_lossy()).finish()
    }
}

// -------------------------------------------------------------------------------------------------

/// A MUTF-8 string slice.
///
/// Equivalent to a [`str`], but with a different encoding.
#[repr(transparent)]
#[derive(PartialEq, Eq, Hash)]
pub struct Mutf8Str([u8]);

impl Mutf8Str {
    /// Create a new [`Mutf8Str`] from a byte slice.
    ///
    /// # Warning
    /// This function will always succeed, as it does not check if the
    /// input is valid MUTF-8.
    ///
    /// The contents will only be checked when converting
    /// into a [`String`] or [`str`].
    #[must_use]
    pub const fn from_bytes(bytes: &[u8]) -> &Self {
        // SAFETY: `Mutf8Str` is a newtype around `[u8]`, so this is safe.
        unsafe { &*(core::ptr::from_ref::<[u8]>(bytes) as *const Mutf8Str) }
    }

    /// Get the inner bytes of a [`Mutf8Str`].
    #[inline]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] { &self.0 }

    /// Returns the number of bytes in the [`Mutf8Str`].
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize { self.as_bytes().len() }

    /// Returns `true` if the [`Mutf8Str`] is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.as_bytes().is_empty() }

    /// Returns `true` if the [`Mutf8Str`] is valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !simd_cesu8::implementation::active::contains_utf8_4_byte_char_header(self.as_bytes())
    }

    /// Convert a [`Mutf8Str`] to a [`Mutf8String`].
    #[inline]
    #[must_use]
    pub fn to_mutf8_string(&self) -> Mutf8String { Mutf8String(self.0.to_vec()) }

    /// Convert a [`Mutf8Str`] to a [`Cow<str>`], including invalid characters.
    ///
    /// See [`simd_cesu8::decode_lossy`] for more information.
    #[inline]
    #[must_use]
    pub fn to_str_lossy(&self) -> Cow<'_, str> { simd_cesu8::decode_lossy(self.as_bytes()) }

    /// Convert a [`Mutf8Str`] to a [`String`], including invalid characters.
    ///
    /// See [`simd_cesu8::decode_lossy`] for more information.
    #[inline]
    #[must_use]
    pub fn to_string_lossy(&self) -> String { self.to_str_lossy().into_owned() }

    /// Convert a [`Mutf8Str`] to a [`Cow<str>`].
    ///
    /// # Errors
    /// Returns an error if the MUTF-8 string is invalid.
    #[inline]
    pub fn try_as_str(&self) -> Result<Cow<'_, str>, simd_cesu8::DecodingError> {
        simd_cesu8::decode(self.as_bytes())
    }

    /// Convert a [`Mutf8Str`] to a [`String`].
    ///
    /// # Errors
    /// Returns an error if the MUTF-8 string is invalid.
    #[inline]
    pub fn try_as_string(&self) -> Result<String, simd_cesu8::DecodingError> {
        self.try_as_str().map(Cow::into_owned)
    }
}

impl AsRef<[u8]> for Mutf8Str {
    fn as_ref(&self) -> &[u8] { &self.0 }
}
impl Borrow<[u8]> for Mutf8Str {
    fn borrow(&self) -> &[u8] { &self.0 }
}

impl ToOwned for Mutf8Str {
    type Owned = Mutf8String;

    fn to_owned(&self) -> Self::Owned { self.to_mutf8_string() }
}

impl PartialEq<Mutf8String> for Mutf8Str {
    fn eq(&self, other: &Mutf8String) -> bool { self.as_bytes() == other.as_bytes() }
}
impl PartialEq<Mutf8String> for &Mutf8Str {
    fn eq(&self, other: &Mutf8String) -> bool { self.as_bytes() == other.as_bytes() }
}

impl Equivalent<str> for Mutf8Str {
    fn equivalent(&self, key: &str) -> bool { self.as_bytes() == key.as_bytes() }
}
impl Equivalent<[u8]> for Mutf8Str {
    fn equivalent(&self, key: &[u8]) -> bool { self.as_bytes() == key }
}

impl<'a> From<&'a Mutf8String> for &'a Mutf8Str {
    fn from(value: &'a Mutf8String) -> Self { value.as_mutf8_str() }
}
impl<'a> From<&'a str> for &'a Mutf8Str {
    fn from(value: &'a str) -> Self { Mutf8Str::from_bytes(value.as_bytes()) }
}
impl<'a> From<&'a [u8]> for &'a Mutf8Str {
    fn from(value: &'a [u8]) -> Self { Mutf8Str::from_bytes(value) }
}

impl<'a> TryFrom<&'a Mutf8Str> for String {
    type Error = simd_cesu8::DecodingError;

    fn try_from(value: &'a Mutf8Str) -> Result<Self, Self::Error> { value.try_as_string() }
}
impl<'a> TryFrom<&'a Mutf8Str> for Cow<'a, str> {
    type Error = simd_cesu8::DecodingError;

    fn try_from(value: &'a Mutf8Str) -> Result<Self, Self::Error> { value.try_as_str() }
}

impl core::fmt::Debug for Mutf8Str {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Mutf8Str").field(&self.to_str_lossy()).finish()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Mutf8String {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        self.as_mutf8_str().serialize(serializer)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Mutf8String {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        String::deserialize(deserializer).map(Mutf8String::from)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Mutf8Str {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        self.try_as_str().map_err(serde::ser::Error::custom)?.serialize(serializer)
    }
}
