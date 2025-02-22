use rkyv::{
    ArchiveUnsized, ArchivedMetadata, DeserializeUnsized, Portable, SerializeUnsized,
    primitive::ArchivedUsize,
    ptr_meta::Pointee,
    rancor::{Fallible, Source},
    ser::Writer,
    traits::ArchivePointee,
};

use super::compound::ArchivedNbtCompound;
use crate::{
    mutf8::Mutf8Str,
    nbt::{NamedNbt, NbtTag, UnnamedNbt},
};

/// An unsized representation of [`NamedNbt`].
#[repr(transparent)]
#[derive(Portable)]
pub struct ArchivedNamedNbt([u8]);

impl ArchivedNamedNbt {
    /// Get the [`Mutf8Str`] name of the [`ArchivedNamedNbt`].
    #[must_use]
    pub fn name(&self) -> Option<&Mutf8Str> {
        self.has_compound().then(|| {
            // Not `unsafe`, but could cause a nasty surprise later
            Mutf8Str::from_bytes(self.name_bytes())
        })
    }

    /// Get the [`ArchivedNbtCompound`] of the [`ArchivedNamedNbt`].
    ///
    /// TODO: Check if this is safe.
    #[must_use]
    pub fn compound(&self) -> Option<&ArchivedNbtCompound> {
        self.has_compound().then(|| {
            // SAFETY: Checked `ArchivedNamedNbt` contains an `ArchivedNbtCompound`.
            Some(unsafe { ArchivedNbtCompound::from_bytes(&self.compound_bytes()) })
        })
    }

    /// Get the base tag of the [`ArchivedNamedNbt`].
    ///
    /// If the base tag is [`NbtTag::COMPOUND`], then the [`ArchivedNamedNbt`]
    /// contains a [`ArchivedNbtCompound`].
    ///
    /// If the base tag is [`NbtTag::END`], then the [`ArchivedNamedNbt`] is
    /// empty.
    #[must_use]
    pub fn base_tag(&self) -> u8 { self.0.get(0).copied().unwrap_or_default() }
    /// Returns `true` if the [`ArchivedNamedNbt`] contains a
    /// [`ArchivedNbtCompound`].
    #[must_use]
    pub fn has_compound(&self) -> bool { self.base_tag() == NbtTag::COMPOUND }

    /// Get the bytes of the name of the [`ArchivedNamedNbt`].
    #[inline]
    #[must_use]
    pub fn name_bytes(&self) -> &[u8] {
        self.has_compound().then(|| &self.0[3..3 + self.name_len() as usize]).unwrap_or_default()
    }
    /// Get the number of bytes in the name of the [`ArchivedNamedNbt`].
    #[inline]
    #[must_use]
    pub fn name_len(&self) -> u16 {
        self.has_compound().then(|| u16::from_be_bytes([self.0[1], self.0[2]])).unwrap_or_default()
    }

    /// Get the bytes of the [`ArchivedNamedNbt`].
    #[inline]
    #[must_use]
    pub fn compound_bytes(&self) -> &[u8] {
        self.has_compound().then(|| &self.0[3 + self.name_len() as usize..]).unwrap_or_default()
    }
    /// Get the number of bytes in the [`ArchivedNamedNbt`].
    #[inline]
    #[must_use]
    pub fn compound_len(&self) -> usize { self.compound_bytes().len() }

    /// Get the bytes of the [`ArchivedNamedNbt`].
    #[inline]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] { &self.0 }
}

unsafe impl Pointee for ArchivedNamedNbt {
    type Metadata = <[u8] as Pointee>::Metadata;
}
impl ArchivePointee for ArchivedNamedNbt {
    type ArchivedMetadata = <[u8] as ArchivePointee>::ArchivedMetadata;
    #[inline]
    fn pointer_metadata(archived: &Self::ArchivedMetadata) -> <Self as Pointee>::Metadata {
        <[u8] as ArchivePointee>::pointer_metadata(archived)
    }
}

impl ArchiveUnsized for NamedNbt {
    type Archived = ArchivedNamedNbt;
    fn archived_metadata(&self) -> ArchivedMetadata<Self> {
        ArchivedUsize::from_native(froglight_io::standard::FrogWrite::frog_len(self) as _)
    }
}

impl<S: Fallible + Writer + ?Sized> SerializeUnsized<S> for NamedNbt {
    // TODO: Write another serializer instead of using `froglight_io`.
    fn serialize_unsized(&self, serializer: &mut S) -> Result<usize, S::Error> {
        let result = serializer.pos();
        let buffer: Vec<u8> = froglight_io::standard::FrogWrite::frog_to_buf(self).unwrap();
        serializer.write(&buffer).map(|()| result)
    }
}
impl<D: Fallible + ?Sized> DeserializeUnsized<NamedNbt, D> for ArchivedNamedNbt
where
    D::Error: Source,
{
    // TODO: Write another deserializer instead of using `froglight_io`.
    unsafe fn deserialize_unsized(&self, _: &mut D, out: *mut NamedNbt) -> Result<(), D::Error> {
        let mut cursor = std::io::Cursor::new(&self.0);
        match froglight_io::standard::FrogRead::frog_read(&mut cursor) {
            Ok(result) => unsafe { *out = result },
            Err(err) => rkyv::rancor::fail!(err),
        }
        Ok(())
    }

    fn deserialize_metadata(&self) -> <NamedNbt as Pointee>::Metadata {}
}

// -------------------------------------------------------------------------------------------------

/// An unsized representation of [`UnnamedNbt`].
#[repr(transparent)]
#[derive(Portable)]
pub struct ArchivedUnnamedNbt([u8]);

impl ArchivedUnnamedNbt {
    /// Get the [`ArchivedNbtCompound`] of the [`ArchivedUnnamedNbt`].
    ///
    /// TODO: Check if this is safe.
    #[must_use]
    pub fn compound(&self) -> Option<&ArchivedNbtCompound> {
        if self.has_compound() {
            // SAFETY: Checked `ArchivedUnnamedNbt` contains an `ArchivedNbtCompound`
            Some(unsafe { ArchivedNbtCompound::from_bytes(self.compound_bytes()) })
        } else {
            None
        }
    }

    /// Get the base tag of the [`ArchivedUnnamedNbt`].
    ///
    /// If the base tag is [`NbtTag::COMPOUND`], then the [`ArchivedUnnamedNbt`]
    /// contains a [`ArchivedNbtCompound`].
    ///
    /// If the base tag is [`NbtTag::END`], then the [`ArchivedUnnamedNbt`] is
    /// empty.
    #[must_use]
    pub fn base_tag(&self) -> u8 { self.0.get(0).copied().unwrap_or_default() }
    /// Returns `true` if the [`ArchivedUnnamedNbt`] contains a
    /// [`ArchivedNbtCompound`].
    #[must_use]
    pub fn has_compound(&self) -> bool { self.base_tag() == NbtTag::COMPOUND }

    /// Get the bytes of the [`ArchivedUnnamedNbt`].
    #[inline]
    #[must_use]
    pub fn compound_bytes(&self) -> &[u8] {
        (self.0.len() >= 2).then(|| &self.0[1..]).unwrap_or_default()
    }
    /// Get the number of bytes in the [`ArchivedUnnamedNbt`].
    #[inline]
    #[must_use]
    pub fn compound_len(&self) -> usize { self.0.len().saturating_sub(1) }

    /// Get the bytes of the [`ArchivedUnnamedNbt`].
    #[inline]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] { &self.0 }
}

unsafe impl Pointee for ArchivedUnnamedNbt {
    type Metadata = <[u8] as Pointee>::Metadata;
}
impl ArchivePointee for ArchivedUnnamedNbt {
    type ArchivedMetadata = <[u8] as ArchivePointee>::ArchivedMetadata;
    #[inline]
    fn pointer_metadata(archived: &Self::ArchivedMetadata) -> <Self as Pointee>::Metadata {
        <[u8] as ArchivePointee>::pointer_metadata(archived)
    }
}

impl ArchiveUnsized for UnnamedNbt {
    type Archived = ArchivedUnnamedNbt;
    fn archived_metadata(&self) -> ArchivedMetadata<Self> {
        ArchivedUsize::from_native(froglight_io::standard::FrogWrite::frog_len(self) as _)
    }
}
