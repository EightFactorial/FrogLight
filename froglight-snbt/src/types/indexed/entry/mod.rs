//! TODO

use alloc::string::String;
use core::fmt;

mod index;
pub use index::{EntryIndex, ValueDescription, ValueIndex};

use crate::types::indexed::{
    core::IndexCore,
    reference::{IndexedReference, ValueReference},
};

/// An SNBT entry that is indexed by an [`IndexCore`].
pub struct IndexedEntry<'index, C: IndexCore> {
    core: &'index C,
    index: EntryIndex,
}

impl<'index, C: IndexCore> IndexedEntry<'index, C> {
    /// Create a new [`IndexedEntry`] from the given core and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is valid for the given core.
    #[inline]
    #[must_use]
    pub const unsafe fn new(core: &'index C, index: EntryIndex) -> Self { Self { core, index } }

    /// Get the name of this entry.
    #[inline]
    #[must_use]
    pub fn name(self) -> IndexedReference<'index, String> {
        // SAFETY: `IndexedEntry` guarantees that this is safe.
        unsafe { IndexedReference::new(self.core.root(), self.index.name()) }
    }

    /// Get the value of this entry.
    #[inline]
    #[must_use]
    pub fn value(self) -> ValueReference<'index, C> {
        // SAFETY: `IndexedEntry` guarantees that this is safe.
        unsafe { ValueReference::new(self.core, self.index.value()) }
    }

    /// Get the name and value pair of this entry.
    #[inline]
    #[must_use]
    pub fn pair(self) -> (IndexedReference<'index, String>, ValueReference<'index, C>) {
        (self.name(), self.value())
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
pub struct IndexedValue<'index, C: IndexCore> {
    core: &'index C,
    index: ValueIndex,
}

impl<'index, C: IndexCore> IndexedValue<'index, C> {
    /// Create a new [`IndexedValue`] from the given core and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is valid for the given core.
    #[inline]
    #[must_use]
    pub const unsafe fn new(core: &'index C, index: ValueIndex) -> Self { Self { core, index } }

    /// Get the value of this entry.
    #[must_use]
    pub fn get(&self) -> ValueReference<'index, C> {
        // SAFETY: `IndexedValue` guarantees that this is safe.
        unsafe { ValueReference::new(self.core, self.index) }
    }
}

macro_rules! create_fns {
    ($($ident:ident: $ty:ty => $variant:ident),*) => {
        impl<'index, C: IndexCore + 'index> IndexedValue<'index, C> {
            $(
                #[must_use]
                #[doc = concat!("Return a reference to the stored value if it is of type [`", stringify!($ty), "`], else `None`.")]
                pub fn $ident(&self) -> Option<IndexedReference<'index, $ty>> {
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
