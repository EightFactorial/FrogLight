//! TODO

use alloc::string::String;
use core::fmt;

mod index;
pub use index::{EntryIndex, ValueIndex};

use crate::types::indexed::{
    core::IndexCore,
    reference::{IndexedReference, ValueReference},
};

/// An SNBT entry that is indexed by an [`IndexCore`].
pub struct IndexedEntry<'data, C: IndexCore> {
    core: &'data C,
    index: EntryIndex,
}

impl<'data, C: IndexCore> IndexedEntry<'data, C> {
    /// Create a new [`IndexedEntry`] from the given core and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is valid for the given core.
    #[inline]
    #[must_use]
    pub const unsafe fn new(core: &'data C, index: EntryIndex) -> Self { Self { core, index } }

    /// Get the name of this entry.
    #[must_use]
    pub fn name(&self) -> IndexedReference<'data, String> {
        // SAFETY: `IndexedEntry` guarantees that this is safe.
        unsafe { IndexedReference::new(self.core.root(), self.index.name()) }
    }

    /// Get the value of this entry.
    #[must_use]
    pub fn value(&self) -> ValueReference<'data, C> {
        // SAFETY: `IndexedEntry` guarantees that this is safe.
        unsafe { ValueReference::new(self.core, self.index.value()) }
    }
}

impl<C: IndexCore> fmt::Debug for IndexedEntry<'_, C> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IndexedEntry")
            .field("name", &self.name())
            .field("value", &self.value())
            .finish()
    }
}

impl<C: IndexCore> Clone for IndexedEntry<'_, C> {
    #[inline]
    fn clone(&self) -> Self { *self }
}
impl<C: IndexCore> Copy for IndexedEntry<'_, C> {}

impl<C: IndexCore> PartialEq for IndexedEntry<'_, C> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name() && self.value() == other.value()
    }
}
impl<C: IndexCore> Eq for IndexedEntry<'_, C> {}

// -------------------------------------------------------------------------------------------------

/// An SNBT value that is indexed by an [`IndexCore`].
pub struct IndexedValue<'data, C: IndexCore> {
    core: &'data C,
    index: ValueIndex,
}

impl<'data, C: IndexCore> IndexedValue<'data, C> {
    /// Create a new [`IndexedValue`] from the given core and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is valid for the given core.
    #[inline]
    #[must_use]
    pub const unsafe fn new(core: &'data C, index: ValueIndex) -> Self { Self { core, index } }

    /// Get the value of this entry.
    #[must_use]
    pub fn get(&self) -> ValueReference<'data, C> {
        // SAFETY: `IndexedValue` guarantees that this is safe.
        unsafe { ValueReference::new(self.core, self.index) }
    }
}

macro_rules! create_fns {
    ($($ident:ident: $ty:ty => $variant:ident),*) => {
        impl<'data, C: IndexCore + 'data> IndexedValue<'data, C> {
            $(
                #[must_use]
                #[doc = concat!("Return a reference to the stored value if it is of type [`", stringify!($ty), "`], else `None`.")]
                pub fn $ident(&self) -> Option<IndexedReference<'data, $ty>> {
                    if let ValueIndex::$variant(value) = self.index {
                        Some(unsafe { IndexedReference::<$ty>::new(self.core.root(), value) })
                    } else {
                        None
                    }
                }
            )*
        }
    };
}

create_fns! {
    as_byte: u8 => Byte,
    as_short: u16 => Short,
    as_int: u32 => Int,
    as_long: u64 => Long,
    as_float: f32 => Float,
    as_double: f64 => Double,
    as_string: String => String
}
