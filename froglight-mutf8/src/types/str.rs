//! TODO

#[cfg(feature = "alloc")]
use alloc::{borrow::ToOwned, boxed::Box};
use core::{borrow::Borrow, fmt, str::from_utf8 as from_utf8_core};

use simdutf8::basic::from_utf8 as from_utf8_simd;

#[cfg(feature = "alloc")]
use crate::types::MString;

/// MUTF-8 string slices.
///
/// Equivalent to [`str`],
/// but uses MUTF-8 instead of UTF-8.
#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MStr([u8]);

impl fmt::Debug for MStr {
    #[cfg(feature = "alloc")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.to_mstring().into_utf8(), f)
    }

    #[cfg(not(feature = "alloc"))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.as_bytes(), f)
    }
}
impl fmt::Display for MStr {
    #[cfg(feature = "alloc")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.to_mstring().into_utf8(), f)
    }

    #[cfg(not(feature = "alloc"))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.as_bytes(), f)
    }
}

impl MStr {
    /// Returns the length of `self`.
    ///
    /// This length is in bytes, not [`char`]s or graphemes.
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize { self.0.len() }

    /// Returns `true` if the string is empty.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Converts a string slice to a byte slice.
    #[inline]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] { &self.0 }

    /// Converts a string slice to a mutable byte slice.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the bytes are valid MUTF-8 after mutation.
    #[inline]
    #[must_use]
    pub const unsafe fn as_bytes_mut(&mut self) -> &mut [u8] { &mut self.0 }

    /// Creates a new [`MStr`] from a string slice.
    ///
    /// This is slightly faster than [`Self::from_mutf8`] because it can
    /// skip the UTF-8 validation step.
    ///
    /// # Errors
    ///
    /// Returns an error if the bytes are not valid MUTF-8.
    #[expect(clippy::result_unit_err, reason = "WIP")]
    pub fn from_utf8(str: &str) -> Result<&Self, ()> {
        if contains_null_or_4_byte_header(str.as_bytes()) {
            Err(())
        } else {
            // SAFETY: The bytes were just checked to be valid MUTF-8.
            Ok(unsafe { Self::from_mutf8_unchecked(str.as_bytes()) })
        }
    }

    /// Creates a new [`MStr`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the bytes are not valid MUTF-8.
    #[expect(clippy::result_unit_err, reason = "WIP")]
    pub fn from_mutf8(bytes: &[u8]) -> Result<&Self, ()> {
        from_utf8_simd(bytes).map_or_else(|_| Err(()), Self::from_utf8)
    }

    /// Creates a [`str`] from a MUTF-8 string slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the bytes are not valid UTF-8.
    #[expect(clippy::result_unit_err, reason = "WIP")]
    pub fn to_utf8(&self) -> Result<&str, ()> {
        if from_utf8_simd(self.as_bytes()).is_ok() {
            Ok(unsafe { str::from_utf8_unchecked(self.as_bytes()) })
        } else {
            Err(())
        }
    }

    /// Creates a new [`MStr`] from a string slice.
    ///
    /// This is a `const` version of [`Self::from_utf8`],
    /// and may be slower due to the lack of optimizations.
    ///
    /// # Errors
    ///
    /// Returns `None` if the bytes are not valid MUTF-8.
    #[must_use]
    pub const fn const_from_utf8(str: &str) -> Option<&Self> {
        if fallback::const_contains_null_or_4_byte_header(str.as_bytes()) {
            None
        } else {
            // SAFETY: The bytes were just checked to be valid MUTF-8.
            Some(unsafe { Self::from_mutf8_unchecked(str.as_bytes()) })
        }
    }

    /// Creates a new [`MStr`] from a byte slice.
    ///
    /// This is a `const` version of [`Self::from_mutf8`],
    /// and may be slower due to the lack of optimizations.
    ///
    /// # Errors
    ///
    /// Returns `None` if the bytes are not valid MUTF-8.
    #[must_use]
    pub const fn const_from_mutf8(bytes: &[u8]) -> Option<&Self> {
        if let Ok(str) = from_utf8_core(bytes) { Self::const_from_utf8(str) } else { None }
    }

    /// Creates a new [`MStr`] from a string slice.
    ///
    /// This is a `const` version of [`Self::to_utf8`],
    /// and may be slower due to the lack of optimizations.
    ///
    /// # Errors
    ///
    /// Returns `None` if the bytes are not valid UTF-8.
    #[must_use]
    pub const fn const_to_utf8(&self) -> Option<&str> {
        if fallback::const_contains_null_or_4_byte_header(self.as_bytes()) {
            None
        } else {
            match from_utf8_core(self.as_bytes()) {
                Ok(str) => Some(str),
                Err(_) => None,
            }
        }
    }

    /// Creates a new [`MStr`] from a slice without checking if the bytes are
    /// valid MUTF-8.
    ///
    /// # Safety
    ///
    /// The caller must ensure the slice is valid MUTF-8.
    #[inline]
    #[must_use]
    pub const unsafe fn from_mutf8_unchecked(bytes: &[u8]) -> &Self {
        unsafe { &*(core::ptr::from_ref::<[u8]>(bytes) as *const Self) }
    }

    /// Creates a new [`MStr`] from a slice without checking if the bytes are
    /// valid MUTF-8.
    ///
    /// # Safety
    ///
    /// The caller must ensure the slice is valid MUTF-8.
    #[inline]
    #[must_use]
    pub const unsafe fn from_mutf8_mut_unchecked(bytes: &mut [u8]) -> &mut Self {
        unsafe { &mut *(core::ptr::from_mut::<[u8]>(bytes) as *mut Self) }
    }
}

#[cfg(feature = "alloc")]
impl MStr {
    /// Converts the string slice to an owned [`MString`].
    #[inline]
    #[must_use]
    pub fn to_mstring(&self) -> MString {
        // SAFETY: The bytes are guaranteed to be valid MUTF-8.
        unsafe { MString::from_mutf8_unchecked(self.0.to_vec()) }
    }

    /// Converts the given boxed [`MStr`] slice to a [`MString`].
    /// It is notable that the [`MStr`] slice is owned.
    #[inline]
    #[must_use]
    pub fn into_mstring(self: Box<Self>) -> MString {
        // SAFETY: `MStr` is `repr(transparent)` over `[u8]`.
        let bytes: Box<[u8]> = unsafe { core::mem::transmute(self) };
        // SAFETY: The bytes are guaranteed to be valid MUTF-8.
        unsafe { MString::from_mutf8_unchecked(bytes.into_vec()) }
    }
}

#[cfg(feature = "alloc")]
impl ToOwned for MStr {
    type Owned = MString;

    #[inline]
    fn to_owned(&self) -> Self::Owned { self.to_mstring() }
}

// -------------------------------------------------------------------------------------------------

impl AsRef<[u8]> for MStr {
    #[inline]
    fn as_ref(&self) -> &[u8] { self.as_bytes() }
}
impl Borrow<[u8]> for MStr {
    #[inline]
    fn borrow(&self) -> &[u8] { self.as_bytes() }
}

impl PartialEq<str> for MStr {
    #[inline]
    fn eq(&self, other: &str) -> bool { self.as_bytes() == other.as_bytes() }
}
impl PartialEq<MStr> for str {
    #[inline]
    fn eq(&self, other: &MStr) -> bool { self.as_bytes() == other.as_bytes() }
}

impl PartialEq<[u8]> for MStr {
    #[inline]
    fn eq(&self, other: &[u8]) -> bool { self.as_bytes() == other }
}
impl PartialEq<MStr> for [u8] {
    #[inline]
    fn eq(&self, other: &MStr) -> bool { self == other.as_bytes() }
}

impl<'a> TryFrom<&'a str> for &'a MStr {
    type Error = ();

    #[inline]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> { MStr::from_utf8(value) }
}
impl<'a> TryFrom<&'a [u8]> for &'a MStr {
    type Error = ();

    #[inline]
    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> { MStr::from_mutf8(value) }
}

#[cfg(feature = "facet")]
unsafe impl facet::Facet<'_> for MStr {
    const SHAPE: &'static facet::Shape = &const {
        #[allow(clippy::wildcard_imports, reason = "For this span")]
        use facet::*;

        const VTABLE: VTableIndirect = vtable_indirect!(MStr =>
            Debug,
            Display,
            PartialEq,
            PartialOrd,
            Ord,
            Hash,
        );

        static MSTR_TYPE_OPS: TypeOpsIndirect = TypeOpsIndirect {
            clone_into: None,
            default_in_place: None,
            drop_in_place: mstr_drop,
            is_truthy: Some(mstr_truthy),
        };

        const unsafe fn mstr_drop(_: OxPtrMut) {}

        #[inline(always)]
        #[allow(clippy::inline_always, reason = "Inline")]
        unsafe fn mstr_truthy(value: PtrConst) -> bool {
            !unsafe { value.get::<MStr>() }.is_empty()
        }

        ShapeBuilder::for_unsized::<MStr>("MStr")
            .ty(Type::User(UserType::Opaque))
            .def(Def::Undefined)
            .vtable_indirect(&VTABLE)
            .type_ops_indirect(&MSTR_TYPE_OPS)
            .eq()
            .send()
            .sync()
            .build()
    };
}

// -------------------------------------------------------------------------------------------------

/// A `const` macro for creating [`MStr`] literals.
///
/// This should only be used for `const` and `static` items,
/// as the methods on [`MStr`] and [`MString`] are generally faster.
///
/// # Panics
///
/// Panics if the string literal is not valid MUTF-8.
#[macro_export]
macro_rules! mutf8 {
    ($str:literal) => {{
        match $crate::prelude::MStr::const_from_utf8($str) {
            Some(mstr) => mstr,
            None => panic!(concat!("Invalid MUTF-8 string literal: ", $str)),
        }
    }};
}

// -------------------------------------------------------------------------------------------------

cfg_select! {
    feature = "simd" => {
        pub use crate::simd::mutf8::{contains_null_or_4_byte_header, contains_4_byte_header};
    }
    _ => {
        pub use fallback::{contains_null_or_4_byte_header, contains_4_byte_header};
    }
}

#[doc(hidden)]
pub mod fallback {
    /// Returns `true` if the given slice contains any null bytes or 4-byte
    /// UTF-8 headers.
    #[must_use]
    #[inline(always)]
    #[expect(clippy::inline_always, reason = "Performance")]
    pub fn contains_null_or_4_byte_header(bytes: &[u8]) -> bool {
        bytes.iter().any(|b| *b == 0b0000_0000 || (*b & 0b1111_1000) == 0b1111_0000)
    }

    /// Returns `true` if the given slice contains any 4-byte UTF-8 headers.
    #[must_use]
    #[inline(always)]
    #[expect(clippy::inline_always, reason = "Performance")]
    pub fn contains_4_byte_header(bytes: &[u8]) -> bool {
        bytes.iter().any(|b| (*b & 0b1111_1000) == 0b1111_0000)
    }

    /// Returns `true` if the given slice contains any null bytes or 4-byte
    /// UTF-8 headers.
    #[must_use]
    #[inline(always)]
    #[expect(clippy::inline_always, reason = "Performance")]
    pub const fn const_contains_null_or_4_byte_header(bytes: &[u8]) -> bool {
        let mut i = 0;
        while i < bytes.len() {
            let b = bytes[i];
            if b == 0b0000_0000 || (b & 0b1111_1000) == 0b1111_0000 {
                return true;
            }
            i += 1;
        }
        false
    }

    /// Returns `true` if the given slice contains any 4-byte UTF-8 headers.
    #[must_use]
    #[inline(always)]
    #[expect(clippy::inline_always, reason = "Performance")]
    pub const fn const_contains_4_byte_header(bytes: &[u8]) -> bool {
        let mut i = 0;
        while i < bytes.len() {
            if (bytes[i] & 0b1111_1000) == 0b1111_0000 {
                return true;
            }
            i += 1;
        }
        false
    }
}
