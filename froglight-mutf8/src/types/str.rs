//! TODO

use core::{borrow::Borrow, fmt, str::from_utf8 as from_utf8_core};

use simdutf8::basic::from_utf8 as from_utf8_simd;

/// MUTF-8 string slices.
///
/// Equivalent to [`str`],
/// but uses MUTF-8 instead of UTF-8.
#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MStr([u8]);

impl fmt::Debug for MStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("TODO") }
}
impl fmt::Display for MStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("TODO") }
}

impl MStr {
    /// Returns `true` if the string is empty.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Converts a string slice to a byte slice.
    #[inline]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] { &self.0 }

    /// Creates a new [`MStr`] from a slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the bytes are not valid MUTF-8.
    #[expect(clippy::result_unit_err, reason = "WIP")]
    pub fn try_from_bytes(bytes: &[u8]) -> Result<&Self, ()> {
        if contains_null_or_4_byte_header(bytes) && from_utf8_simd(bytes).is_ok() {
            // SAFETY: The bytes were just checked to be valid MUTF-8.
            Ok(unsafe { Self::from_bytes_unchecked(bytes) })
        } else {
            Err(())
        }
    }

    /// Creates a new [`MStr`] from a slice.
    ///
    /// This is a `const` version of [`Self::try_from_bytes`],
    /// and may be slower due to the lack of optimizations.
    ///
    /// # Errors
    ///
    /// Returns `None` if the bytes are not valid MUTF-8.
    #[must_use]
    pub const fn const_try_from_bytes(bytes: &[u8]) -> Option<&Self> {
        if fallback::const_contains_null_or_4_byte_header(bytes) && from_utf8_core(bytes).is_ok() {
            // SAFETY: The bytes were just checked to be valid MUTF-8.
            Some(unsafe { Self::from_bytes_unchecked(bytes) })
        } else {
            None
        }
    }

    /// Converts the string slice to an owned [`MString`].
    #[inline]
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn to_mstring(&self) -> super::MString {
        // SAFETY: The bytes are guaranteed to be valid MUTF-8.
        unsafe { super::MString::from_mutf8_unchecked(self.0.to_vec()) }
    }

    /// Creates a new [`MStr`] from a slice without checking if the bytes are
    /// valid MUTF-8.
    ///
    /// # Safety
    ///
    /// The caller must ensure the slice is valid MUTF-8.
    #[inline]
    #[must_use]
    pub const unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
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
    pub const unsafe fn from_bytes_mut_unchecked(bytes: &mut [u8]) -> &mut Self {
        unsafe { &mut *(core::ptr::from_mut::<[u8]>(bytes) as *mut Self) }
    }
}

impl AsRef<[u8]> for MStr {
    #[inline]
    fn as_ref(&self) -> &[u8] { self.as_bytes() }
}
impl Borrow<[u8]> for MStr {
    #[inline]
    fn borrow(&self) -> &[u8] { self.as_bytes() }
}

impl<'a> TryFrom<&'a [u8]> for &'a MStr {
    type Error = ();

    #[inline]
    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> { MStr::try_from_bytes(value) }
}
impl<'a> TryFrom<&'a str> for &'a MStr {
    type Error = ();

    #[inline]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        MStr::try_from_bytes(value.as_bytes())
    }
}

#[cfg(feature = "alloc")]
impl alloc::borrow::ToOwned for MStr {
    type Owned = super::MString;

    #[inline]
    fn to_owned(&self) -> Self::Owned { self.to_mstring() }
}

// -------------------------------------------------------------------------------------------------

cfg_select! {
    feature = "simd" => {
        pub use crate::simd::string::{contains_null_or_4_byte_header, contains_4_byte_header};
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
    pub fn contains_null_or_4_byte_header(bytes: &[u8]) -> bool {
        bytes.iter().copied().any(|b| b == 0b0000_0000 || (b & 0b1111_1000) == 0b1111_0000)
    }

    /// Returns `true` if the given slice contains any 4-byte UTF-8 headers.
    #[must_use]
    pub fn contains_4_byte_header(bytes: &[u8]) -> bool {
        bytes.iter().copied().any(|b| (b & 0b1111_1000) == 0b1111_0000)
    }

    /// Returns `true` if the given slice contains any null bytes or 4-byte
    /// UTF-8 headers.
    #[must_use]
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

// -------------------------------------------------------------------------------------------------

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
