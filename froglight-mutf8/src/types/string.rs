//! TODO

use alloc::{borrow::Cow, boxed::Box, string::String, vec::Vec};
use core::{
    borrow::{Borrow, BorrowMut},
    fmt,
    ops::{Add, AddAssign, Deref, DerefMut},
    str::FromStr,
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
    /// Creates a new empty [`MString`].
    #[inline]
    #[must_use]
    pub const fn new() -> Self { Self(Vec::new()) }

    /// Creates a new empty [`MString`] with at least the specified capacity.
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self { Self(Vec::with_capacity(capacity)) }

    /// Returns this [`MString`]'s capacity, in bytes.
    #[inline]
    #[must_use]
    pub const fn capacity(&self) -> usize { self.0.capacity() }

    /// Returns the length of this [`MString`], in bytes, not chars or
    /// graphemes.
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize { self.0.len() }

    /// Returns true if this [`MString`] has a length of zero, and false
    /// otherwise.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Converts a vector of bytes to a [`MString`].
    ///
    /// This method will take care to not copy the vector, for efficiency's
    /// sake.
    ///
    /// If you need a [`&MStr`](MStr) instead of a [`MString`], consider
    /// [`MStr::from_mutf8`].
    ///
    /// The inverse of this method is [`into_bytes`].
    ///
    /// # Errors
    ///
    /// TODO
    #[expect(clippy::result_unit_err, reason = "WIP")]
    pub fn from_mutf8(vec: Vec<u8>) -> Result<Self, ()> {
        match MStr::from_mutf8(&vec) {
            Ok(..) => Ok(Self(vec)),
            Err(err) => Err(err),
        }
    }

    /// Converts a slice of bytes to a string, including invalid characters.
    ///
    /// During this conversion, `from_mutf8_lossy()` will replace any invalid
    /// UTF-8 sequences with [`U+FFFD REPLACEMENT CHARACTER`][U+FFFD], which
    /// looks like this: �
    #[must_use]
    pub fn from_mutf8_lossy(v: &[u8]) -> Cow<'_, MStr> {
        match simdutf8::compat::from_utf8(v) {
            Ok(s) => Self::from_utf8(s),
            Err(err) => {
                // SAFETY: The index returned is within bounds.
                let (valid, invalid) = unsafe { v.split_at_unchecked(err.valid_up_to()) };
                let mut string = String::from_utf8_lossy(invalid).into_owned();

                // SAFETY: `valid` is guaranteed to be valid UTF-8.
                string.push_str(unsafe { str::from_utf8_unchecked(valid) });
                // SAFETY: `string` is rotated along char byte boundaries.
                unsafe { string.as_bytes_mut().rotate_right(invalid.len()) };

                Cow::Owned(Self::from_utf8_owned(string))
            }
        }
    }

    /// Converts a [`Vec<u8>`] to a [`MString`], substituting invalid MUTF-8
    /// sequences with replacement characters.
    ///
    /// See [`from_mutf8_lossy`](Self::from_mutf8_lossy) for more details.
    ///
    /// Note that this function does not guarantee reuse of the original [`Vec`]
    /// allocation.
    #[must_use]
    pub fn from_mutf8_lossy_owned(v: Vec<u8>) -> Self {
        match Self::from_mutf8_lossy(&v) {
            // SAFETY: `Borrowed` means the input was valid MUTF-8.
            Cow::Borrowed(_) => unsafe { Self::from_mutf8_unchecked(v) },
            Cow::Owned(mstr) => mstr,
        }
    }

    /// Converts a [`str`] to a [`MStr`].
    #[must_use]
    pub fn from_utf8(str: &str) -> Cow<'_, MStr> {
        match MStr::from_utf8(str) {
            // SAFETY: `Ok` means the input was valid MUTF-8.
            Ok(..) => Cow::Borrowed(unsafe { MStr::from_mutf8_unchecked(str.as_bytes()) }),
            // SAFETY: `utf8_to_mutf8` produces valid MUTF-8.
            Err(..) => Cow::Owned(utf8_to_mutf8(str)),
        }
    }

    /// Converts a [`String`] to a [`MString`].
    ///
    /// If you need a [`&MStr`](MStr) instead of a [`MString`], consider
    /// [`MStr::from_utf8`].
    ///
    /// See [`from_utf8`](Self::from_utf8) for more details.
    ///
    /// Note that this function does not guarantee reuse of the original
    /// [`String`] allocation.
    #[must_use]
    pub fn from_utf8_owned(s: String) -> Self {
        match Self::from_utf8(&s) {
            // SAFETY: `Borrowed` means the input was valid MUTF-8.
            Cow::Borrowed(_) => unsafe { Self::from_mutf8_unchecked(s.into_bytes()) },
            Cow::Owned(mstr) => mstr,
        }
    }

    /// Truncates this [`MString`], removing all contents.
    ///
    /// While this means the [`MString`] will have a length of zero, it does not
    /// touch its capacity.
    #[inline]
    pub fn clear(&mut self) { self.0.clear(); }

    /// Reserves capacity for at least `additional` bytes more than the
    /// current length. The allocator may reserve more space to speculatively
    /// avoid frequent allocations. After calling `reserve`,
    /// capacity will be greater than or equal to `self.len() + additional`.
    /// Does nothing if capacity is already sufficient.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` _bytes_.
    #[inline]
    pub fn reserve(&mut self, additional: usize) { self.0.reserve(additional); }

    /// Reserves the minimum capacity for at least `additional` bytes more than
    /// the current length. Unlike [`reserve`](Self::reserve), this will not
    /// deliberately over-allocate to speculatively avoid frequent allocations.
    /// After calling `reserve_exact`, capacity will be greater than or equal to
    /// `self.len() + additional`. Does nothing if the capacity is already
    /// sufficient.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` _bytes_.
    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) { self.0.reserve_exact(additional); }

    /// Appends a given string slice onto the end of this [`MString`].
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` _bytes_.
    #[inline]
    pub fn push_mstr(&mut self, string: &MStr) { self.0.extend_from_slice(string.as_bytes()); }

    /// Extract a [`MStr`] slice containing the entire string.
    #[inline]
    #[must_use]
    pub const fn as_mstr(&self) -> &MStr {
        // SAFETY: The bytes are guaranteed to be valid MUTF-8.
        unsafe { MStr::from_mutf8_unchecked(self.0.as_slice()) }
    }

    /// Converts a [`MString`] into a mutable string slice.
    #[inline]
    #[must_use]
    pub const fn as_mstr_mut(&mut self) -> &mut MStr {
        // SAFETY: The bytes are guaranteed to be valid MUTF-8.
        unsafe { MStr::from_mutf8_mut_unchecked(self.0.as_mut_slice()) }
    }

    /// Extract a byte slice containing the entire string.
    #[inline]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] { self.0.as_slice() }

    /// Converts a [`MString`] into a byte vector.
    ///
    /// This consumes the [`MString`], so we do not need to copy its contents.
    #[inline]
    #[must_use]
    pub const fn into_bytes(self) -> Vec<u8> {
        // SAFETY: `MString` is `repr(transparent)` over `Vec<u8>`.
        unsafe { core::mem::transmute(self) }
    }

    /// Converts a [`MString`] into a [`Box<MStr>`].
    ///
    /// Before doing the conversion, this method discards excess capacity.
    #[inline]
    #[must_use]
    pub fn into_boxed_mstr(self) -> Box<MStr> {
        let bytes = self.0.into_boxed_slice();
        // SAFETY: `MStr` is `repr(transparent)` over `[u8]`.
        unsafe { core::mem::transmute(bytes) }
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

// -------------------------------------------------------------------------------------------------

impl Add<&MStr> for MString {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: &MStr) -> Self::Output {
        self.push_mstr(rhs);
        self
    }
}
impl AddAssign<&MStr> for MString {
    #[inline]
    fn add_assign(&mut self, rhs: &MStr) { self.push_mstr(rhs); }
}

impl<'a> Extend<&'a MStr> for MString {
    #[inline]
    fn extend<T: IntoIterator<Item = &'a MStr>>(&mut self, iter: T) {
        iter.into_iter().for_each(move |mstr| self.push_mstr(mstr));
    }
}
impl<'a> Extend<&'a mut MStr> for MString {
    #[inline]
    fn extend<T: IntoIterator<Item = &'a mut MStr>>(&mut self, iter: T) {
        iter.into_iter().for_each(move |mstr| self.push_mstr(mstr));
    }
}
impl Extend<Box<MStr>> for MString {
    #[inline]
    fn extend<T: IntoIterator<Item = Box<MStr>>>(&mut self, iter: T) {
        iter.into_iter().for_each(move |mstr| self.push_mstr(&mstr));
    }
}
impl<'a> Extend<Cow<'a, MStr>> for MString {
    #[inline]
    fn extend<T: IntoIterator<Item = Cow<'a, MStr>>>(&mut self, iter: T) {
        iter.into_iter().for_each(move |mstr| self.push_mstr(&mstr));
    }
}

impl From<&MStr> for MString {
    #[inline]
    fn from(value: &MStr) -> Self { value.to_mstring() }
}
impl From<&mut MStr> for MString {
    #[inline]
    fn from(value: &mut MStr) -> Self { value.to_mstring() }
}

impl From<&MString> for MString {
    #[inline]
    fn from(value: &MString) -> Self { value.clone() }
}
impl From<&mut MString> for MString {
    #[inline]
    fn from(value: &mut MString) -> Self { value.clone() }
}

impl From<Box<MStr>> for MString {
    #[inline]
    fn from(value: Box<MStr>) -> Self { value.into_mstring() }
}
impl<'a> From<Cow<'a, MStr>> for MString {
    #[inline]
    fn from(value: Cow<'a, MStr>) -> Self {
        match value {
            Cow::Borrowed(mstr) => mstr.to_mstring(),
            Cow::Owned(mstr) => mstr,
        }
    }
}

impl FromStr for MString {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match MStr::from_utf8(s) {
            // SAFETY: `Ok` means the input was valid MUTF-8.
            Ok(..) => Ok(unsafe { Self::from_mutf8_unchecked(s.as_bytes().into()) }),
            Err(err) => Err(err),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Convert a UTF-8 string to MUTF-8.
#[must_use]
#[doc(hidden)]
pub fn utf8_to_mutf8(str: &str) -> MString {
    macro_rules! debug_panic {
        () => {{
            #[cfg(debug_assertions)]
            panic!("Invalid UTF-8, was expecting another byte!");

            // SAFETY: This should never be reachable for a valid UTF-8 string
            #[cfg(not(debug_assertions))]
            unsafe {
                core::hint::unreachable_unchecked()
            }
        }};
    }

    let cap = str.len().saturating_mul(2).min(isize::MAX as usize);
    let mut output = Vec::<u8>::with_capacity(cap);

    let mut iter = str.as_bytes().iter();
    while let Some(a) = iter.next() {
        match a {
            // U+0000 is encoded as [0xC0, 0x80] in MUTF-8.
            0x00 => {
                output.push(0xC0);
                output.push(0x80);
            }
            // U+0001 to U+007F are 1-byte UTF-8 sequences.
            ..=0x7F => {
                output.push(*a);
            }
            // U+0080 to U+07FF are 2-byte UTF-8 sequences.
            ..=0xDF => {
                let Some(b) = iter.next() else { debug_panic!() };
                output.push(*a);
                output.push(*b);
            }
            // U+0800 to U+FFFF are 3-byte UTF-8 sequences.
            ..=0xEF => {
                let Some(b) = iter.next() else { debug_panic!() };
                let Some(c) = iter.next() else { debug_panic!() };
                output.push(*a);
                output.push(*b);
                output.push(*c);
            }
            // U+10000 to U+10FFFF are 4-byte UTF-8 sequences. (UTF-8 max is U+10FFFF)
            _ => {
                let Some(b) = iter.next() else { debug_panic!() };
                let Some(c) = iter.next() else { debug_panic!() };
                let Some(d) = iter.next() else { debug_panic!() };
                output.extend_from_slice(&encode_4_byte_utf8([*a, *b, *c, *d]));
            }
        }
    }

    // SAFETY: The output is valid MUTF-8
    unsafe { MString::from_mutf8_unchecked(output) }
}

#[inline(always)]
#[allow(clippy::inline_always, reason = "Performance")]
fn encode_4_byte_utf8([a, b, c, d]: [u8; 4]) -> [u8; 6] {
    let codepoint = (u32::from(a & 0x07) << 18)
        | (u32::from(b & 0x3F) << 12)
        | (u32::from(c & 0x3F) << 6)
        | u32::from(d & 0x3F);

    let codepoint = codepoint - 0x0001_0000;
    let high = (codepoint >> 10) | 0xD800;
    let low = (codepoint & 0x03FF) | 0xDC00;

    [
        0xE0 | ((high & 0xF000) >> 12) as u8,
        0x80 | ((high & 0x0FC0) >> 6) as u8,
        0x80 | ((high & 0x003F) as u8),
        0xE0 | ((low & 0xF000) >> 12) as u8,
        0x80 | ((low & 0x0FC0) >> 6) as u8,
        0x80 | ((low & 0x003F) as u8),
    ]
}
