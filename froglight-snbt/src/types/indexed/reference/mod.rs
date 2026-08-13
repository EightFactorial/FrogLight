//! TODO

use alloc::string::String;
use core::fmt;

use crate::types::indexed::index::{
    Index, Indexable,
    numeric::{Float, Integer, IntegerSignness},
};

mod entry;
pub use entry::EntryReference;

mod value;
pub use value::ValueReference;

/// A reference to an SNBT value.
pub struct IndexedReference<'index, T: Referenceable + ?Sized> {
    root: &'index str,
    index: Index<T::Indexable>,
}

/// A trait for types that can be referenced by an [`IndexedReference`].
pub trait Referenceable {
    /// The type of index used to reference this type.
    type Indexable: Indexable + ?Sized;
    /// The type of value that can be obtained from this reference.
    type Value<'a>: Sized
    where
        Self: 'a;

    /// Get the value of this reference.
    #[must_use]
    fn get_value(reference: IndexedReference<'_, Self>) -> Self::Value<'_>;
}

impl<'index, T: Referenceable + ?Sized> IndexedReference<'index, T> {
    /// Create a new [`IndexedReference`] with the given root and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure the index is valid for the given root string.
    #[inline]
    #[must_use]
    pub const unsafe fn new(root: &'index str, index: Index<T::Indexable>) -> Self {
        Self { root, index }
    }

    /// Upgrade the lifetime of the reference.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the same root string is used.
    #[inline]
    #[must_use]
    pub const unsafe fn upgrade(self, root: &str) -> IndexedReference<'_, T> {
        IndexedReference { root, index: self.index }
    }

    /// Get the value of this reference.
    #[inline]
    #[must_use]
    pub fn get(self) -> T::Value<'index> { T::get_value(self) }

    /// Get a description of this value.
    #[inline]
    #[must_use]
    pub const fn description(&self) -> <T::Indexable as Indexable>::Description {
        self.index.description()
    }
}

impl<'index, T: Referenceable<Indexable = Integer> + ?Sized + 'index> IndexedReference<'index, T> {
    /// Call the function based on the signness of the value.
    ///
    /// Try using with [`cast_signed`](u32::cast_signed) to convert the value to
    /// a signed integer.
    #[inline]
    pub fn or_signed<R>(
        self,
        unsigned: impl FnOnce(T::Value<'index>) -> R,
        signed: impl FnOnce(T::Value<'index>) -> R,
    ) -> R {
        match self.description().signness() {
            IntegerSignness::None | IntegerSignness::Unsigned => unsigned(self.get()),
            IntegerSignness::Signed => signed(self.get()),
        }
    }

    /// Call the function based on the signness of the value.
    ///
    /// Can be used where the `argument` cannot be moved into either closure.
    ///
    /// Try using with [`cast_signed`](u32::cast_signed) to convert the value to
    /// a signed integer.
    #[inline]
    pub fn or_signed_with<U, R>(
        self,
        argument: U,
        unsigned: impl FnOnce(T::Value<'index>, U) -> R,
        signed: impl FnOnce(T::Value<'index>, U) -> R,
    ) -> R {
        match self.description().signness() {
            IntegerSignness::None | IntegerSignness::Unsigned => unsigned(self.get(), argument),
            IntegerSignness::Signed => signed(self.get(), argument),
        }
    }
}

// -------------------------------------------------------------------------------------------------

macro_rules! impl_referenceable {
    ( $ty:ty $(,)? => { $($ref_tt:tt)* }, => { $($dbg_tt:tt)* }, => { $($dsp_tt:tt)* } ) => {
        impl Referenceable for $ty {
            $($ref_tt)*
        }

        impl core::fmt::Debug for IndexedReference<'_, $ty> {
            $($dbg_tt)*
        }
        impl core::fmt::Display for IndexedReference<'_, $ty> {
            $($dsp_tt)*
        }
    };
    ( $($ty:ty),+ $(,)? => $ref_block:tt, => $dbg_block:tt, => $dsp_block:tt ) => {
        $(
            impl_referenceable! { $ty => $ref_block, => $dbg_block, => $dsp_block }
        )+
    };
}

impl_referenceable! {
    bool,
    => {
        type Indexable = bool;
        type Value<'a> = Self;

        #[inline]
        fn get_value(reference: IndexedReference<'_, Self>) -> Self::Value<'_> {
            // SAFETY: `IndexedReference` guarantees that this is safe.
            unsafe { reference.index.read_value(reference.root).into() }
        }
    },
    => {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.get(), f)
        }
    },
    => {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(&self.get(), f)
        }
    }
}

impl_referenceable! {
    u8, u16, u32, u64,
    => {
        type Indexable = Integer;
        type Value<'a> = Self;

        #[inline]
        fn get_value(reference: IndexedReference<'_, Self>) -> Self::Value<'_> {
            // SAFETY: `IndexedReference` guarantees that this is safe.
            unsafe { reference.index.read_value(reference.root).into() }
        }
    },
    => {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.or_signed_with(f, |v, f| fmt::Debug::fmt(&v, f), |v, f| fmt::Debug::fmt(&v.cast_signed(), f))
        }
    },
    => {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.or_signed_with(f, |v, f| fmt::Display::fmt(&v, f), |v, f| fmt::Display::fmt(&v.cast_signed(), f))
        }
    }
}

impl_referenceable! {
    f32, f64,
    => {
        type Indexable = Float;
        type Value<'a> = Self;

        #[inline]
        fn get_value(reference: IndexedReference<'_, Self>) -> Self::Value<'_> {
            // SAFETY: `IndexedReference` guarantees that this is safe.
            unsafe { reference.index.read_value(reference.root).into() }
        }
    },
    => {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.get(), f)
        }
    },
    => {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(&self.get(), f)
        }
    }
}

impl_referenceable! {
    String,
    => {
        type Indexable = String;
        type Value<'a> = &'a str;

        #[inline]
        fn get_value(reference: IndexedReference<'_, Self>) -> Self::Value<'_> {
            // SAFETY: `IndexedReference` guarantees that this is safe.
            unsafe { reference.index.read_value(reference.root) }
        }
    },
    => {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.get(), f)
        }
    },
    => {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(&self.get(), f)
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<T: Referenceable + ?Sized> Clone for IndexedReference<'_, T> {
    #[inline]
    fn clone(&self) -> Self { *self }
}
impl<T: Referenceable + ?Sized> Copy for IndexedReference<'_, T> {}

impl<T: Referenceable + ?Sized> PartialEq for IndexedReference<'_, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool { self.index == other.index && self.root == other.root }
}
impl<T: Referenceable + ?Sized> Eq for IndexedReference<'_, T> {}

impl<T: Referenceable + ?Sized> PartialOrd for IndexedReference<'_, T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> { Some(self.cmp(other)) }
}
impl<T: Referenceable + ?Sized> Ord for IndexedReference<'_, T> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.index.cmp(&other.index).then_with(|| self.root.cmp(other.root))
    }
}
