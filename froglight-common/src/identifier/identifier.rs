//! TODO
#![allow(
    clippy::unsafe_derive_deserialize,
    reason = "Allowed, as while important, it does not cause undefined behavior"
)]

#[cfg(feature = "bevy")]
use alloc::boxed::Box;
use alloc::{borrow::Cow, string::String};
#[cfg(feature = "bevy")]
use core::any::Any;
use core::{borrow::Borrow, fmt, hash::Hash, ops::Deref, str::FromStr};

#[cfg(feature = "bevy")]
use bevy_reflect::{
    ApplyError, FromReflect, FromType, GetTypeRegistration, OpaqueInfo, PartialReflect, Reflect,
    ReflectCloneError, ReflectFromReflect, ReflectMut, ReflectOwned, ReflectRef, TypeInfo,
    TypePath, TypeRegistration, Typed,
    utility::{NonGenericTypeCell, NonGenericTypeInfoCell},
};
#[cfg(all(feature = "bevy", feature = "serde"))]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};
#[cfg(feature = "facet")]
use facet::{Def, Facet, Shape, Type, TypeOpsDirect, UserType, VTableDirect};
#[cfg(feature = "froglight-facet")]
use froglight_facet::facet::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::identifier::{Ident, IdentifierError};

/// A namespaced identifier [`String`].
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bevy", derive(TypePath))]
pub struct Identifier<'a>(Cow<'a, str>);

impl Identifier<'_> {
    /// The default namespace to use when one is not provided.
    pub const DEFAULT_NAMESPACE: &'static str = "minecraft";

    /// Create a new static [`Identifier`].
    ///
    /// # Panics
    ///
    /// This will panic if the string is not a valid identifier.
    #[inline]
    #[must_use]
    pub const fn new_static(str: &'static str) -> Identifier<'static> {
        Ident::new_static(str).as_identifier()
    }

    /// Try to create a new [`Identifier`] from a string slice.
    ///
    /// If the string does not contain a namespace,
    /// the [`DEFAULT_NAMESPACE`] will be prepended.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not a valid identifier.
    pub fn try_new<T: AsRef<str> + ?Sized>(str: &T) -> Result<Identifier<'_>, IdentifierError> {
        let val = str.as_ref();
        match Ident::try_new(val) {
            Ok(ident) => Ok(ident.as_identifier()),
            Err(IdentifierError::RequiresNamespace) => {
                let mut string =
                    String::with_capacity(Self::DEFAULT_NAMESPACE.len() + 1 + val.len());

                string.push_str(Self::DEFAULT_NAMESPACE);
                string.push(':');
                string.push_str(val);

                // SAFETY: We just checked that `string` is valid.
                Ok(unsafe { Self::new_owned_unchecked(string) })
            }
            Err(err) => Err(err),
        }
    }

    /// Try to create an owned [`Identifier`] from a string slice.
    ///
    /// If the string does not contain a namespace,
    /// the [`DEFAULT_NAMESPACE`] will be prepended.
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
    /// If the string does not contain a namespace,
    /// the [`DEFAULT_NAMESPACE`] will be prepended.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not a valid identifier.
    pub fn try_new_string(s: String) -> Result<Self, IdentifierError> {
        match Ident::try_new(s.as_str()) {
            Ok(..) => Ok(unsafe { Self::new_owned_unchecked(s) }),
            Err(IdentifierError::RequiresNamespace) => {
                let mut string = String::with_capacity(Self::DEFAULT_NAMESPACE.len() + 1 + s.len());

                string.push_str(Self::DEFAULT_NAMESPACE);
                string.push(':');
                string.push_str(&s);

                // SAFETY: We just checked that `string` is valid.
                Ok(unsafe { Self::new_owned_unchecked(string) })
            }
            Err(err) => Err(err),
        }
    }

    /// Convert this [`Identifier`] into an owned version.
    #[inline]
    #[must_use]
    pub fn into_owned(self) -> Identifier<'static> { Identifier(Cow::Owned(self.0.into_owned())) }

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
    #[inline]
    #[must_use]
    pub const fn reborrow(&self) -> Identifier<'_> {
        match &self.0 {
            Cow::Borrowed(s) => Identifier(Cow::Borrowed(s)),
            Cow::Owned(s) => Identifier(Cow::Borrowed(s.as_str())),
        }
    }

    /// Get the content of this [`Identifier`] as a string slice.
    #[inline]
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match &self.0 {
            Cow::Borrowed(s) => s,
            Cow::Owned(s) => s.as_str(),
        }
    }

    /// Get the content of this [`Identifier`] as an [`Ident`].
    #[inline]
    #[must_use]
    pub const fn as_ident(&self) -> &Ident {
        // SAFETY: `self` is guaranteed to be a valid identifier.
        unsafe { Ident::from_string_unchecked(self.as_str()) }
    }

    /// A `const` method for comparing two identifiers for equality.
    ///
    /// Likely much slower than the standard [`PartialEq`]/[`Eq`]
    /// implementations, but usable in `const` contexts.
    #[inline]
    #[must_use]
    pub const fn const_eq(&self, other: &Self) -> bool {
        self.as_ident().const_eq(other.as_ident())
    }

    /// Create a new [`Identifier`] without checking its validity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided string is a valid identifier.
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(s: &str) -> Identifier<'_> { Identifier(Cow::Borrowed(s)) }

    /// Create a new owned [`Identifier`] without checking its validity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided string is a valid identifier.
    #[inline]
    #[must_use]
    pub const unsafe fn new_owned_unchecked(s: String) -> Identifier<'static> {
        Identifier(Cow::Owned(s))
    }
}

impl FromStr for Identifier<'_> {
    type Err = IdentifierError;

    fn from_str(s: &str) -> Result<Self, Self::Err> { Self::try_new_owned(s) }
}

impl<'a> TryFrom<&'a str> for Identifier<'a> {
    type Error = IdentifierError;

    #[inline]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> { Identifier::try_new(value) }
}
impl<'a> TryFrom<&'a [u8]> for Identifier<'a> {
    type Error = IdentifierError;

    #[inline]
    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        str::from_utf8(value).map_or(Err(IdentifierError::Invalid), Identifier::try_new)
    }
}

// -------------------------------------------------------------------------------------------------

impl AsRef<Ident> for Identifier<'_> {
    #[inline]
    fn as_ref(&self) -> &Ident { self.as_ident() }
}
impl Borrow<Ident> for Identifier<'_> {
    #[inline]
    fn borrow(&self) -> &Ident { self.as_ident() }
}

impl AsRef<str> for Identifier<'_> {
    #[inline]
    fn as_ref(&self) -> &str { self.as_str() }
}
impl Borrow<str> for Identifier<'_> {
    #[inline]
    fn borrow(&self) -> &str { self.as_str() }
}

impl AsRef<[u8]> for Identifier<'_> {
    #[inline]
    fn as_ref(&self) -> &[u8] { self.as_str().as_bytes() }
}

impl Deref for Identifier<'_> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target { self.as_str() }
}

impl fmt::Display for Identifier<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(self.as_str(), f) }
}

impl fmt::Debug for Identifier<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Identifier").field(&self.as_str()).finish()
    }
}

impl PartialEq<Ident> for Identifier<'_> {
    #[inline]
    fn eq(&self, other: &Ident) -> bool { self.as_ident() == other }
}
impl PartialEq<Identifier<'_>> for Ident {
    #[inline]
    fn eq(&self, other: &Identifier<'_>) -> bool { self == other.as_ident() }
}

impl PartialEq<String> for Identifier<'_> {
    #[inline]
    fn eq(&self, other: &String) -> bool { self.as_str() == other.as_str() }
}
impl PartialEq<Identifier<'_>> for String {
    #[inline]
    fn eq(&self, other: &Identifier<'_>) -> bool { self.as_str() == other.as_str() }
}

impl PartialEq<str> for Identifier<'_> {
    #[inline]
    fn eq(&self, other: &str) -> bool { self.as_str() == other }
}
impl PartialEq<Identifier<'_>> for str {
    #[inline]
    fn eq(&self, other: &Identifier<'_>) -> bool { self == other.as_str() }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "bevy")]
impl Typed for Identifier<'static> {
    fn type_info() -> &'static TypeInfo {
        static CELL: NonGenericTypeInfoCell = NonGenericTypeCell::new();
        CELL.get_or_set(|| TypeInfo::Opaque(OpaqueInfo::new::<Identifier<'static>>()))
    }
}

#[cfg(feature = "bevy")]
impl GetTypeRegistration for Identifier<'static> {
    fn get_type_registration() -> TypeRegistration {
        let mut registration = TypeRegistration::of::<Self>();
        registration.insert::<ReflectFromReflect>(FromType::<Self>::from_type());

        #[cfg(feature = "serde")]
        {
            registration.insert::<ReflectSerialize>(FromType::<Self>::from_type());
            registration.insert::<ReflectDeserialize>(FromType::<Self>::from_type());
        }

        registration
    }
}

#[cfg(feature = "bevy")]
impl FromReflect for Identifier<'static> {
    fn from_reflect(reflect: &dyn PartialReflect) -> Option<Self> {
        reflect.try_downcast_ref::<Self>().cloned()
    }
}

#[cfg(feature = "bevy")]
impl PartialReflect for Identifier<'static> {
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

    fn reflect_clone(&self) -> Result<Box<dyn Reflect>, ReflectCloneError> {
        Ok(Box::new(self.clone()))
    }

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
impl Reflect for Identifier<'static> {
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
impl Identifier<'_> {
    const DROP_IN_PLACE: unsafe fn(*mut ()) = unsafe {
        core::mem::transmute::<unsafe fn(*mut Self), _>(core::ptr::drop_in_place::<Self>)
    };
}

#[cfg(feature = "facet")]
unsafe impl<'facet> Facet<'facet> for Identifier<'facet> {
    const SHAPE: &'static Shape = &const {
        const VTABLE: VTableDirect = facet::vtable_direct!(Identifier<'_> =>
            Display,
            Debug,
            PartialEq,
            PartialOrd,
            Ord,
            Hash,
        );

        const OPS: TypeOpsDirect = TypeOpsDirect {
            drop_in_place: Identifier::<'_>::DROP_IN_PLACE,
            default_in_place: None,
            clone_into: None,
            is_truthy: None,
        };

        Shape::builder_for_sized::<Identifier<'facet>>("Identifier")
            .doc(&[" A namespaced identifier [`String`]."])
            .type_name(|_shape, f, _opts| ::core::fmt::Write::write_str(f, "Identifier"))
            .ty(Type::User(UserType::Opaque))
            .def(Def::Scalar)
            .vtable_direct(&VTABLE)
            .type_ops_direct(&OPS)
            .eq()
            .send()
            .sync()
            .build()
    };
}

#[cfg(feature = "froglight-facet")]
#[allow(clippy::cast_possible_truncation, reason = "Ignored")]
impl FacetTemplate for Identifier<'_> {
    fn serialize(item: SerializeItem<'_, '_>, writer: &mut Writer<'_>) -> Result<(), WriterError> {
        let item = item.get::<Identifier<'_>>()?;
        encode_u32_into(item.as_str().len() as u32, writer)?;
        writer.write_bytes(item.as_str().as_bytes())
    }

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

#[cfg(feature = "froglight-facet")]
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

#[cfg(feature = "serde")]
impl Serialize for Identifier<'_> {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Identifier<'_> {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        use serde::de::{Error, Visitor};

        struct BorrowedVisitor<'out>(core::marker::PhantomData<&'out ()>);

        impl<'out> Visitor<'_> for BorrowedVisitor<'out> {
            type Value = Identifier<'out>;

            fn expecting(&self, formatter: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
                formatter.write_str("a valid identifier string")
            }

            fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
                Identifier::try_new_owned(v).map_err(E::custom)
            }

            fn visit_bytes<E: Error>(self, v: &[u8]) -> Result<Self::Value, E> {
                str::from_utf8(v)
                    .map_err(E::custom)
                    .and_then(|s| Identifier::try_new_owned(s).map_err(E::custom))
            }

            fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
                Identifier::try_new_string(v).map_err(E::custom)
            }

            fn visit_byte_buf<E: Error>(self, v: alloc::vec::Vec<u8>) -> Result<Self::Value, E> {
                String::from_utf8(v)
                    .map_err(E::custom)
                    .and_then(|s| Identifier::try_new_string(s).map_err(E::custom))
            }
        }

        de.deserialize_string(BorrowedVisitor(core::marker::PhantomData))
    }
}
