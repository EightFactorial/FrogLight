//! TODO

#[cfg(feature = "bevy")]
use alloc::boxed::Box;
#[cfg(feature = "bevy")]
use core::any::Any;
use core::{borrow::Borrow, fmt, ops::Deref};

#[cfg(all(feature = "bevy", feature = "serde"))]
use bevy_reflect::ReflectSerialize;
#[cfg(feature = "bevy")]
use bevy_reflect::{
    ApplyError, FromReflect, FromType, GetTypeRegistration, OpaqueInfo, PartialReflect, Reflect,
    ReflectCloneError, ReflectFromReflect, ReflectMut, ReflectOwned, ReflectRef, TypeInfo,
    TypePath, TypeRegistration, Typed,
    utility::{NonGenericTypeCell, NonGenericTypeInfoCell},
};
#[cfg(feature = "facet")]
use facet::{
    Def, Facet, OxPtrMut, PtrConst, Shape, ShapeBuilder, Type, TypeOpsIndirect, UserType,
    VTableIndirect,
};

use crate::identifier::IdentifierError;

/// A namespaced identifier [`str`].
///
/// Typically created at compile-time or during zero-copy operations.
#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bevy", derive(TypePath))]
pub struct Ident(str);

impl Ident {
    /// Try to create an [`Ident`] from a string slice.
    ///
    /// # Errors
    ///
    /// Returns an [`IdentifierError`] if the string is not a valid identifier.
    pub const fn try_new(str: &str) -> Result<&Ident, IdentifierError> {
        match Self::validate(str) {
            Ok(()) => Ok(unsafe { Self::from_string_unchecked(str) }),
            Err(err) => Err(err),
        }
    }

    /// Create an [`Ident`] from a string slice,
    /// panicking if the string is not a valid identifier.
    ///
    /// This is intended for use in `const` contexts
    /// where the string is known to be valid at compile-time.
    ///
    /// # Panics
    ///
    /// Panics if the string is not a valid identifier.
    #[inline]
    #[must_use]
    pub const fn new_static(str: &'static str) -> &'static Ident {
        match Self::try_new(str) {
            Ok(ident) => ident,
            Err(err) => panic!("{}", err.describe()),
        }
    }

    /// Returns `Ok(())` if the string is a valid identifier,
    /// or an [`IdentifierError`] if not.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not a valid identifier.
    pub const fn validate(str: &str) -> Result<(), IdentifierError> {
        static VALID_NAMESPACE: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz_-.";
        static VALID_PATH: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz_-./";

        /// Returns `true` if the slice contains the given byte.
        const fn slice_contains(slice: &[u8], byte: u8) -> bool {
            let mut i = 0;
            while i < slice.len() {
                if slice[i] == byte {
                    return true;
                }
                i += 1;
            }
            false
        }

        let mut namespace = true;
        let mut current = 0usize;
        let bytes = str.as_bytes();

        while current < bytes.len() {
            // Get the current character
            let char = bytes[current];
            current += 1;

            if char == b':' {
                // Check for the namespace separator
                if namespace {
                    // Switch to path validation
                    namespace = false;
                } else {
                    // Duplicate separators, invalid identifier
                    return Err(IdentifierError::Invalid);
                }
            } else if namespace {
                // Validate namespace characters
                if !slice_contains(VALID_NAMESPACE, char) {
                    return Err(IdentifierError::Invalid);
                }
            } else {
                // Validate path characters
                if !slice_contains(VALID_PATH, char) {
                    return Err(IdentifierError::Invalid);
                }
            }
        }

        if namespace {
            // If we never switched to the path, there is no namespace separator
            Err(IdentifierError::RequiresNamespace)
        } else {
            // Otherwise, the identifier is valid
            Ok(())
        }
    }

    /// Returns the length of `self`.
    ///
    /// This length is in bytes, not [`char`]s or graphemes.
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize { self.0.len() }

    /// Returns `true` if the string is empty.
    ///
    /// # Note
    ///
    /// This will always return false, as an empty string is not a valid
    /// identifier.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool { false }

    /// Get this string as an [`Identifier`].
    #[inline]
    #[must_use]
    #[cfg(feature = "alloc")]
    pub const fn as_identifier(&self) -> crate::identifier::Identifier<'_> {
        // SAFETY: `self` is guaranteed to be a valid identifier.
        unsafe { crate::identifier::Identifier::new_unchecked(&self.0) }
    }

    /// Get the inner string of the [`Ident`].
    #[inline]
    #[must_use]
    pub const fn as_str(&self) -> &str { &self.0 }

    /// Get the inner string as a slice of bytes.
    #[inline]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] { self.0.as_bytes() }

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

    /// A `const` method for comparing two identifiers for equality.
    ///
    /// Likely much slower than the standard [`PartialEq`]/[`Eq`]
    /// implementations, but usable in `const` contexts.
    #[inline]
    #[must_use]
    pub const fn const_eq(&self, other: &Self) -> bool { self.const_eq_str(other.as_str()) }

    /// A `const` method for comparing two identifiers for equality.
    ///
    /// Likely much slower than the standard [`PartialEq`]/[`Eq`]
    /// implementations, but usable in `const` contexts.
    #[must_use]
    pub const fn const_eq_str(&self, other: &str) -> bool {
        let s1 = self.as_bytes();
        let s2 = other.as_bytes();
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

    /// Create an [`Ident`] from a string without checking if the string is
    /// valid.
    ///
    /// # Safety
    ///
    /// The caller must ensure the string is a valid identifier.
    #[inline]
    #[must_use]
    pub const unsafe fn from_string_unchecked(str: &str) -> &Ident {
        unsafe { &*(core::ptr::from_ref::<str>(str) as *const Self) }
    }

    /// Create an [`Ident`] from UTF-8 bytes without checking if the slice is
    /// valid.
    ///
    /// # Safety
    ///
    /// The caller must ensure the bytes are valid UTF-8,
    /// and are a valid identifier.
    #[inline]
    #[must_use]
    pub const unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Ident {
        unsafe { &*(core::ptr::from_ref::<[u8]>(bytes) as *const Self) }
    }
}

impl<'a> TryFrom<&'a str> for &'a Ident {
    type Error = IdentifierError;

    #[inline]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> { Ident::try_new(value) }
}
impl<'a> TryFrom<&'a [u8]> for &'a Ident {
    type Error = IdentifierError;

    #[inline]
    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        str::from_utf8(value).map_or(Err(IdentifierError::Invalid), Ident::try_new)
    }
}

// -------------------------------------------------------------------------------------------------

impl AsRef<str> for Ident {
    #[inline]
    fn as_ref(&self) -> &str { self.as_str() }
}
impl Borrow<str> for Ident {
    #[inline]
    fn borrow(&self) -> &str { self.as_str() }
}

impl AsRef<[u8]> for Ident {
    #[inline]
    fn as_ref(&self) -> &[u8] { self.as_bytes() }
}

impl Deref for Ident {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target { self.as_str() }
}

impl PartialEq<str> for Ident {
    #[inline]
    fn eq(&self, other: &str) -> bool { self.as_bytes() == other.as_bytes() }
}
impl PartialEq<Ident> for str {
    #[inline]
    fn eq(&self, other: &Ident) -> bool { self.as_bytes() == other.as_bytes() }
}

impl PartialEq<[u8]> for Ident {
    #[inline]
    fn eq(&self, other: &[u8]) -> bool { self.as_bytes() == other }
}
impl PartialEq<Ident> for [u8] {
    #[inline]
    fn eq(&self, other: &Ident) -> bool { self == other.as_bytes() }
}

impl fmt::Display for Ident {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(self.as_str(), f) }
}
impl fmt::Debug for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Ident").field(&self.as_str()).finish()
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "bevy")]
impl Typed for &'static Ident {
    fn type_info() -> &'static TypeInfo {
        static CELL: NonGenericTypeInfoCell = NonGenericTypeCell::new();
        CELL.get_or_set(|| TypeInfo::Opaque(OpaqueInfo::new::<&'static Ident>()))
    }
}

#[cfg(feature = "bevy")]
impl GetTypeRegistration for &'static Ident {
    fn get_type_registration() -> TypeRegistration {
        let mut registration = TypeRegistration::of::<Self>();
        registration.insert::<ReflectFromReflect>(FromType::<Self>::from_type());
        #[cfg(feature = "serde")]
        registration.insert::<ReflectSerialize>(FromType::<Self>::from_type());
        registration
    }
}

#[cfg(feature = "bevy")]
impl FromReflect for &'static Ident {
    fn from_reflect(reflect: &dyn PartialReflect) -> Option<Self> {
        reflect.try_downcast_ref::<Self>().copied()
    }
}

#[cfg(feature = "bevy")]
impl PartialReflect for &'static Ident {
    fn get_represented_type_info(&self) -> Option<&'static TypeInfo> {
        Some(<Self as Typed>::type_info())
    }

    #[inline]
    fn into_partial_reflect(self: Box<Self>) -> Box<dyn PartialReflect> { self }

    fn as_partial_reflect(&self) -> &dyn PartialReflect { self }

    fn as_partial_reflect_mut(&mut self) -> &mut dyn PartialReflect { self }

    fn try_into_reflect(self: Box<Self>) -> Result<Box<dyn Reflect>, Box<dyn PartialReflect>> {
        Ok(self)
    }

    fn try_as_reflect(&self) -> Option<&dyn Reflect> { Some(self) }

    fn try_as_reflect_mut(&mut self) -> Option<&mut dyn Reflect> { Some(self) }

    fn reflect_ref(&self) -> ReflectRef<'_> { ReflectRef::Opaque(self) }

    fn reflect_mut(&mut self) -> ReflectMut<'_> { ReflectMut::Opaque(self) }

    fn reflect_owned(self: Box<Self>) -> ReflectOwned { ReflectOwned::Opaque(self) }

    fn reflect_clone(&self) -> Result<Box<dyn Reflect>, ReflectCloneError> { Ok(Box::new(*self)) }

    fn reflect_hash(&self) -> Option<u64> {
        let mut hasher = bevy_reflect::utility::reflect_hasher();
        core::hash::Hash::hash(&Any::type_id(self), &mut hasher);
        core::hash::Hash::hash(self, &mut hasher);
        Some(core::hash::Hasher::finish(&hasher))
    }

    fn reflect_partial_eq(&self, value: &dyn PartialReflect) -> Option<bool> {
        if let Some(value) = value.try_downcast_ref::<Self>() {
            Some(PartialEq::eq(self, value))
        } else {
            Some(false)
        }
    }

    fn reflect_partial_cmp(&self, value: &dyn PartialReflect) -> Option<core::cmp::Ordering> {
        if let Some(value) = value.try_downcast_ref::<Self>() {
            PartialOrd::partial_cmp(self, value)
        } else {
            None
        }
    }

    fn debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(&self, f) }

    fn try_apply(&mut self, value: &dyn PartialReflect) -> Result<(), ApplyError> {
        if let Some(value) = value.try_downcast_ref::<Self>() {
            self.clone_from(value);
        } else {
            return Err(ApplyError::MismatchedTypes {
                from_type: value.reflect_type_path().into(),
                to_type: Self::type_path().into(),
            });
        }
        Ok(())
    }
}

#[cfg(feature = "bevy")]
impl Reflect for &'static Ident {
    fn into_any(self: Box<Self>) -> Box<dyn Any> { self }

    fn as_any(&self) -> &dyn Any { self }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }

    fn into_reflect(self: Box<Self>) -> Box<dyn Reflect> { self }

    fn as_reflect(&self) -> &dyn Reflect { self }

    fn as_reflect_mut(&mut self) -> &mut dyn Reflect { self }

    fn set(&mut self, value: Box<dyn Reflect>) -> Result<(), Box<dyn Reflect>> {
        *self = value.take()?;
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "facet")]
#[expect(clippy::inline_always, reason = "Ignored")]
unsafe impl Facet<'_> for Ident {
    const SHAPE: &'static Shape = &const {
        // Note: Copied from the facet `str` impl.
        const unsafe fn ident_drop(_: OxPtrMut) {}

        // Note: Copied from the facet `str` impl.
        #[inline(always)]
        unsafe fn ident_truthy(value: PtrConst) -> bool {
            !unsafe { value.get::<Ident>() }.is_empty()
        }

        const VTABLE: VTableIndirect = facet::vtable_indirect!(Ident =>
            Display,
            Debug,
            Hash,
            PartialEq,
            PartialOrd,
            Ord,
        );

        const OPS: TypeOpsIndirect = TypeOpsIndirect {
            drop_in_place: ident_drop,
            default_in_place: None,
            clone_into: None,
            is_truthy: Some(ident_truthy),
        };

        ShapeBuilder::for_unsized::<Ident>("Ident")
            .ty(Type::User(UserType::Opaque))
            .def(Def::Scalar)
            .vtable_indirect(&VTABLE)
            .type_ops_indirect(&OPS)
            .eq()
            .send()
            .sync()
            .build()
    };
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "serde")]
impl serde::Serialize for Ident {
    #[inline]
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for &'de Ident {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        use serde::de::{Error, Visitor};

        struct BorrowedVisitor;

        impl<'de> Visitor<'de> for BorrowedVisitor {
            type Value = &'de Ident;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid, borrowed identifier string")
            }

            fn visit_borrowed_str<E: Error>(self, v: &'de str) -> Result<Self::Value, E> {
                Ident::try_new(v).map_err(E::custom)
            }
        }

        de.deserialize_str(BorrowedVisitor)
    }
}
