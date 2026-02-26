//! TODO

use alloc::{borrow::Cow, vec::Vec};
use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

#[cfg(feature = "bevy")]
use bevy_reflect::std_traits::ReflectDefault;
#[cfg(feature = "facet")]
use facet::{Partial, Peek};
#[cfg(feature = "facet")]
use facet_minecraft::{
    self as mc, DeserializeFn, SerializeFn,
    deserialize::{InputCursor, error::DeserializeValueError},
    replace_with::replace_with_or_abort,
    serialize::{buffer::SerializeWriter, error::SerializeIterError},
};

/// A buffer of bytes that has no length prefix.
///
/// If used in a packet,
/// this must be the last field or it will be impossible to deserialize.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", facet(mc::serialize = UnsizedBuffer::SERIALIZE))]
#[cfg_attr(feature = "facet", facet(mc::deserialize = UnsizedBuffer::DESERIALIZE))]
pub struct UnsizedBuffer<'a>(pub Cow<'a, [u8]>);

impl<'a> UnsizedBuffer<'a> {
    /// Create a new empty [`UnsizedBuffer`].
    #[inline]
    #[must_use]
    pub const fn new() -> Self { Self::from_vec(Vec::new()) }

    /// Create a new [`UnsizedBuffer`] from the given [`Vec`].
    #[inline]
    #[must_use]
    pub const fn from_vec(vec: Vec<u8>) -> Self { Self(Cow::Owned(vec)) }

    /// Create a new [`UnsizedBuffer`] from the given `&[u8]` slice.
    #[inline]
    #[must_use]
    pub const fn from_slice(slice: &'a [u8]) -> Self { Self(Cow::Borrowed(slice)) }

    /// Get the contents as a slice.
    #[inline]
    #[must_use]
    pub const fn as_slice(&self) -> &[u8] {
        match self {
            Self(Cow::Borrowed(slice)) => slice,
            Self(Cow::Owned(vec)) => vec.as_slice(),
        }
    }

    /// Get the contents as mutable [`Vec<u8>`].
    ///
    /// Clones the contents if it is currently borrowed.
    #[must_use]
    pub fn as_vec(&mut self) -> &mut Vec<u8> {
        match self {
            Self(Cow::Borrowed(slice)) => {
                let vec = slice.to_vec();
                *self = Self(Cow::Owned(vec));
                match self {
                    Self(Cow::Owned(vec)) => vec,
                    _ => unreachable!(),
                }
            }
            Self(Cow::Owned(vec)) => vec,
        }
    }

    /// Convert the [`UnsizedBuffer`] into a [`Vec`].
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> Cow<'a, [u8]> { self.0 }
}

#[cfg(feature = "facet")]
impl UnsizedBuffer<'_> {
    const DESERIALIZE: DeserializeFn =
        DeserializeFn::new(Self::facet_deserialize_borrowed, Self::facet_deserialize);
    const SERIALIZE: SerializeFn = SerializeFn::new(Self::facet_serialize);

    fn facet_deserialize_borrowed<'facet>(
        partial: &mut Partial<'facet, true>,
        cursor: &mut InputCursor<'facet, 'facet>,
    ) -> Result<(), DeserializeValueError> {
        let slice = cursor.take(cursor.as_slice().len())?;
        replace_with_or_abort(partial, |partial| {
            partial.set(UnsizedBuffer::from_slice(slice)).unwrap()
        });
        Ok(())
    }

    fn facet_deserialize<'facet>(
        partial: &mut Partial<'facet, false>,
        cursor: &mut InputCursor<'_, 'facet>,
    ) -> Result<(), DeserializeValueError> {
        let vec = cursor.take(cursor.as_slice().len())?.to_vec();
        replace_with_or_abort(partial, |partial| {
            partial.set(UnsizedBuffer::from_vec(vec)).unwrap()
        });
        Ok(())
    }

    fn facet_serialize<'mem, 'facet>(
        peek: Peek<'mem, 'facet>,
        writer: &mut dyn SerializeWriter,
    ) -> Result<(), SerializeIterError<'mem, 'facet>> {
        let value = peek.get::<UnsizedBuffer<'facet>>()?;
        if writer.write_data(value.as_slice()) { Ok(()) } else { Err(SerializeIterError::new()) }
    }
}

impl Deref for UnsizedBuffer<'_> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target { self.as_slice() }
}
impl DerefMut for UnsizedBuffer<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { self.as_vec() }
}

impl AsRef<[u8]> for UnsizedBuffer<'_> {
    #[inline]
    fn as_ref(&self) -> &[u8] { self.as_slice() }
}
impl AsMut<[u8]> for UnsizedBuffer<'_> {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] { self.as_vec() }
}
impl AsMut<Vec<u8>> for UnsizedBuffer<'_> {
    #[inline]
    fn as_mut(&mut self) -> &mut Vec<u8> { self.as_vec() }
}

impl Borrow<[u8]> for UnsizedBuffer<'_> {
    #[inline]
    fn borrow(&self) -> &[u8] { self.as_slice() }
}
impl BorrowMut<[u8]> for UnsizedBuffer<'_> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [u8] { self.as_vec() }
}
