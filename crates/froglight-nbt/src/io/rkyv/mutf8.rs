//! MUTF-8 string serialization for rkyv.
//!
//! TODO: Check if these implementations are safe/correct.

use rkyv::{
    ArchiveUnsized, ArchivedMetadata, DeserializeUnsized, SerializeUnsized, ptr_meta::Pointee,
    rancor::Fallible, traits::ArchivePointee,
};

use crate::mutf8::{Mutf8Str, Mutf8String};

impl ArchiveUnsized for Mutf8String {
    type Archived = Mutf8Str;
    #[inline]
    fn archived_metadata(&self) -> ArchivedMetadata<Self> {
        <Mutf8Str as ArchiveUnsized>::archived_metadata(self.as_mutf8_str())
    }
}

impl<S: Fallible + ?Sized> SerializeUnsized<S> for Mutf8String
where
    Mutf8Str: SerializeUnsized<S>,
{
    #[inline]
    fn serialize_unsized(&self, serializer: &mut S) -> Result<usize, S::Error> {
        <Mutf8Str as SerializeUnsized<S>>::serialize_unsized(self.as_mutf8_str(), serializer)
    }
}
impl<D: Fallible + ?Sized> DeserializeUnsized<Mutf8Str, D> for Mutf8String {
    #[inline]
    unsafe fn deserialize_unsized(
        &self,
        deserializer: &mut D,
        out: *mut Mutf8Str,
    ) -> Result<(), <D as Fallible>::Error> {
        // SAFETY: The caller has guaranteed that `out` is non-null, properly
        // aligned, valid for writes, and points to memory allocated according
        // to the layout for the metadata returned from `deserialize_metadata`.
        // Therefore, `out` points to at least `self.len()` bytes.
        // `self.as_mutf8_str()` is valid for reads and points to the bytes of `self`
        // which are also at least `self.len()` bytes.
        unsafe {
            <Mutf8Str as DeserializeUnsized<Mutf8Str, D>>::deserialize_unsized(
                self.as_mutf8_str(),
                deserializer,
                out,
            )
        }
    }

    #[inline]
    fn deserialize_metadata(&self) -> <Mutf8Str as Pointee>::Metadata {
        <Mutf8Str as DeserializeUnsized<Mutf8Str, D>>::deserialize_metadata(self.as_mutf8_str())
    }
}

// -------------------------------------------------------------------------------------------------

impl ArchiveUnsized for Mutf8Str {
    type Archived = Mutf8Str;
    #[inline]
    fn archived_metadata(&self) -> ArchivedMetadata<Self> {
        <[u8] as ArchiveUnsized>::archived_metadata(self.as_bytes())
    }
}

impl<S: Fallible + ?Sized> SerializeUnsized<S> for Mutf8Str
where
    [u8]: SerializeUnsized<S>,
{
    #[inline]
    fn serialize_unsized(&self, serializer: &mut S) -> Result<usize, S::Error> {
        <[u8] as SerializeUnsized<S>>::serialize_unsized(self.as_bytes(), serializer)
    }
}
impl<D: Fallible + ?Sized> DeserializeUnsized<Mutf8Str, D> for Mutf8Str
where
    [u8]: DeserializeUnsized<[u8], D>,
{
    #[inline]
    unsafe fn deserialize_unsized(
        &self,
        deserializer: &mut D,
        out: *mut Mutf8Str,
    ) -> Result<(), <D as Fallible>::Error> {
        // SAFETY: The caller has guaranteed that `out` is non-null, properly
        // aligned, valid for writes, and points to memory allocated according
        // to the layout for the metadata returned from `deserialize_metadata`.
        // Therefore, `out` points to at least `self.len()` bytes.
        // `self.as_bytes()` is valid for reads and points to the bytes of `self`
        // which are also at least `self.len()` bytes.
        unsafe {
            <[u8] as DeserializeUnsized<[u8], D>>::deserialize_unsized(
                self.as_bytes(),
                deserializer,
                out as *mut [u8],
            )
        }
    }

    #[inline]
    fn deserialize_metadata(&self) -> <Mutf8Str as Pointee>::Metadata {
        <[u8] as DeserializeUnsized<[u8], D>>::deserialize_metadata(self.as_bytes())
    }
}

unsafe impl Pointee for Mutf8Str {
    type Metadata = <[u8] as Pointee>::Metadata;
}
impl ArchivePointee for Mutf8Str {
    type ArchivedMetadata = <[u8] as ArchivePointee>::ArchivedMetadata;
    #[inline]
    fn pointer_metadata(archived: &Self::ArchivedMetadata) -> <Self as Pointee>::Metadata {
        <[u8] as ArchivePointee>::pointer_metadata(archived)
    }
}
