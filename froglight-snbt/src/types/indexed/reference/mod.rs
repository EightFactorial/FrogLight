//! TODO

use alloc::{borrow::Cow, string::String};
use core::fmt;

// use froglight_nbt::types::indexed::types::{IndexedListType, IndexedMapType};
use crate::types::indexed::index::{
    Index, Indexable,
    numeric::{Float, Integer},
};

mod entry;
pub use entry::EntryReference;

mod value;
pub use value::ValueReference;

/// A reference to an SNBT value.
pub struct IndexedReference<'data, T: Referenceable + ?Sized> {
    root: &'data str,
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

impl<'data, T: Referenceable + ?Sized> IndexedReference<'data, T> {
    /// Create a new [`IndexedReference`] with the given root and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure the index is valid for the given root string.
    #[inline]
    #[must_use]
    pub const unsafe fn new(root: &'data str, index: Index<T::Indexable>) -> Self {
        Self { root, index }
    }

    /// Get the value of this reference.
    #[inline]
    #[must_use]
    pub fn get(self) -> T::Value<'data> { T::get_value(self) }
}

// -------------------------------------------------------------------------------------------------

macro_rules! impl_referenceable {
    ( $ty:ty $(,)? => { $($tt:tt)* } ) => {
        impl Referenceable for $ty {
            $($tt)*
        }
    };
    ( $($ty:ty),+ $(,)? => $block:tt ) => {
        $(
            impl_referenceable! { $ty => $block }
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
    }
}

impl_referenceable! {
    String,
    => {
        type Indexable = String;
        type Value<'a> = Cow<'a, str>;

        #[inline]
        fn get_value(reference: IndexedReference<'_, Self>) -> Self::Value<'_> {
            // SAFETY: `IndexedReference` guarantees that this is safe.
            unsafe { reference.index.read_value(reference.root) }
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<T: Referenceable + ?Sized> fmt::Debug for IndexedReference<'_, T>
where
    for<'a> T::Value<'a>: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(&self.get(), f) }
}

impl<T: Referenceable + ?Sized> fmt::Display for IndexedReference<'_, T>
where
    for<'a> T::Value<'a>: fmt::Display,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(&self.get(), f) }
}

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
