use rkyv::{
    ArchiveUnsized, ArchivedMetadata, Portable, primitive::ArchivedUsize, ptr_meta::Pointee,
    traits::ArchivePointee,
};

use crate::nbt::NbtCompound;

/// An unsized representation of an [`NbtCompound`].
#[repr(transparent)]
#[derive(Portable)]
pub struct ArchivedNbtCompound([u8]);

impl ArchivedNbtCompound {
    /// Create a new [`ArchivedNbtCompound`] from a byte slice.
    #[must_use]
    pub(super) const unsafe fn from_bytes(bytes: &[u8]) -> &Self {
        // SAFETY: `ArchivedNbtCompound` is a newtype around `[u8]`, so this is safe.
        unsafe { &*(std::ptr::from_ref::<[u8]>(bytes) as *const ArchivedNbtCompound) }
    }

    /// Get the bytes of the [`ArchivedNbtCompound`].
    #[inline]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] { &self.0 }
}

unsafe impl Pointee for ArchivedNbtCompound {
    type Metadata = <[u8] as Pointee>::Metadata;
}
impl ArchivePointee for ArchivedNbtCompound {
    type ArchivedMetadata = <[u8] as ArchivePointee>::ArchivedMetadata;
    #[inline]
    fn pointer_metadata(archived: &Self::ArchivedMetadata) -> <Self as Pointee>::Metadata {
        <[u8] as ArchivePointee>::pointer_metadata(archived)
    }
}

impl ArchiveUnsized for NbtCompound {
    type Archived = ArchivedNbtCompound;
    fn archived_metadata(&self) -> ArchivedMetadata<Self> {
        ArchivedUsize::from_native(froglight_io::standard::FrogWrite::frog_len(self) as _)
    }
}

// -------------------------------------------------------------------------------------------------
