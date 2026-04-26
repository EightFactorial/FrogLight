//! TODO

use alloc::vec::Vec;
use core::{
    borrow::{Borrow, BorrowMut},
    fmt,
    ops::{Deref, DerefMut},
};

use crate::prelude::MStr;

/// A MUTF-8–encoded, growable string.
///
/// Equivalent to [`String`],
/// but uses MUTF-8 instead of UTF-8.
#[repr(transparent)]
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet), facet(opaque))]
pub struct MString(Vec<u8>);

impl fmt::Debug for MString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <MStr as fmt::Debug>::fmt(self.as_mstr(), f)
    }
}
impl fmt::Display for MString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <MStr as fmt::Display>::fmt(self.as_mstr(), f)
    }
}

impl MString {
    /// Creates a new, empty [`MString`].
    #[inline]
    #[must_use]
    pub const fn new() -> Self { Self(Vec::new()) }

    /// Extract a [`MStr`] slice containing the entire string.
    #[inline]
    #[must_use]
    pub const fn as_mstr(&self) -> &MStr {
        // SAFETY: The bytes are guaranteed to be valid MUTF-8.
        unsafe { MStr::from_bytes_unchecked(self.0.as_slice()) }
    }

    /// Converts a [`MString`] into a mutable string slice.
    #[inline]
    #[must_use]
    pub const fn as_mstr_mut(&mut self) -> &mut MStr {
        // SAFETY: The bytes are guaranteed to be valid MUTF-8.
        unsafe { MStr::from_bytes_mut_unchecked(self.0.as_mut_slice()) }
    }

    /// Converts a vector of bytes to a [`MString`] without checking that the
    /// bytes contain valid MUTF-8.
    ///
    /// # Safety
    ///
    /// The caller must ensure the bytes are valid MUTF-8.
    #[inline]
    #[must_use]
    pub const unsafe fn from_mutf8_unchecked(bytes: Vec<u8>) -> Self { Self(bytes) }
}

// -------------------------------------------------------------------------------------------------

impl AsRef<MStr> for MString {
    #[inline]
    fn as_ref(&self) -> &MStr { self.as_mstr() }
}
impl AsMut<MStr> for MString {
    #[inline]
    fn as_mut(&mut self) -> &mut MStr { self.as_mstr_mut() }
}
impl AsRef<[u8]> for MString {
    #[inline]
    fn as_ref(&self) -> &[u8] { self.0.as_slice() }
}

impl Borrow<MStr> for MString {
    #[inline]
    fn borrow(&self) -> &MStr { self.as_mstr() }
}
impl BorrowMut<MStr> for MString {
    #[inline]
    fn borrow_mut(&mut self) -> &mut MStr { self.as_mstr_mut() }
}
impl Borrow<[u8]> for MString {
    #[inline]
    fn borrow(&self) -> &[u8] { self.0.as_slice() }
}

impl Deref for MString {
    type Target = MStr;

    #[inline]
    fn deref(&self) -> &MStr { self.as_mstr() }
}
impl DerefMut for MString {
    #[inline]
    fn deref_mut(&mut self) -> &mut MStr { self.as_mstr_mut() }
}

impl PartialEq<MStr> for MString {
    #[inline]
    fn eq(&self, other: &MStr) -> bool { self.as_mstr() == other }
}
impl PartialEq<MString> for MStr {
    #[inline]
    fn eq(&self, other: &MString) -> bool { self == other.as_mstr() }
}

impl From<&MStr> for MString {
    #[inline]
    fn from(value: &MStr) -> Self { value.to_mstring() }
}
