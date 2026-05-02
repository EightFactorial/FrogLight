//! TODO
use alloc::borrow::Cow;

mod entry;
pub use entry::{IndexedCompound, IndexedEntry, IndexedTag, IndexedTagItem};

mod mutability;
#[cfg(feature = "froglight-facet")]
use froglight_facet::{
    self as mc,
    facet::{FacetBorrowedTemplate, FacetTemplate},
    format::{Reader, Writer, WriterError, serialize::SerializeItem},
};
pub use mutability::{Mut, NbtMut, Ref};

mod reference;
pub use reference::{NbtIndex, NbtItem};

/// TODO
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", facet(opaque))]
#[cfg_attr(feature = "froglight-facet", facet(mc::with = IndexedNbt::<Mut>::WITH_BORROW))]
pub struct IndexedNbt<'a, Mut: NbtMut> {
    root: Cow<'a, [u8]>,
    _phantom: core::marker::PhantomData<Mut>,
}

impl<'a> IndexedNbt<'a, Ref> {
    /// Get the raw byte data of this NBT structure.
    #[must_use]
    pub const fn raw_data(&self) -> &[u8] {
        match self.root {
            Cow::Borrowed(data) => data,
            Cow::Owned(ref data) => data.as_slice(),
        }
    }

    /// Create a new [`IndexedNbt`] from a byte slice.
    #[must_use]
    pub fn new_unchecked(root: &'a [u8]) -> Self {
        Self { root: Cow::Borrowed(root), _phantom: core::marker::PhantomData }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "froglight-facet")]
impl<Mut: NbtMut> FacetTemplate for IndexedNbt<'static, Mut> {
    fn serialize(
        _item: SerializeItem<'_, '_>,
        _writer: &mut Writer<'_>,
    ) -> Result<(), WriterError> {
        todo!()
    }

    fn deserialize(
        _: facet::Partial<'static, false>,
        _: &mut Reader<'_>,
    ) -> Result<facet::Partial<'static, false>, facet::ReflectError> {
        todo!()
    }
}

#[cfg(feature = "froglight-facet")]
impl<Mut: NbtMut> FacetBorrowedTemplate for IndexedNbt<'static, Mut> {
    fn deserialize_borrowed<'facet>(
        _: facet::Partial<'facet, true>,
        _: &mut Reader<'facet>,
    ) -> Result<facet::Partial<'facet, true>, facet::ReflectError> {
        todo!()
    }
}
