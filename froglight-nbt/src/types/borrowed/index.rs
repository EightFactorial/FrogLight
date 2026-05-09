//! TODO

use core::{any::TypeId, fmt, hash, marker::PhantomData};

use froglight_mutf8::prelude::MStr;

/// A typed index for an NBT entry.
#[repr(transparent)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct Index<T: ?Sized>(usize, PhantomData<T>);

impl<T: ?Sized> Index<T> {
    /// Create a new typed [`Index`].
    #[inline]
    #[must_use]
    pub const fn new(index: usize) -> Self { Self(index, PhantomData) }

    /// Cast this [`Index`] to a different type.
    ///
    /// # Safety
    ///
    /// The caller must ensure that index value points to a valid entry of type
    /// `U`.
    #[inline]
    #[must_use]
    pub const unsafe fn cast<U: ?Sized>(self) -> Index<U> { Index(self.0, PhantomData) }

    /// Get the raw index value.
    #[inline]
    #[must_use]
    pub const fn value(self) -> usize { self.0 }
}

impl<T: ?Sized> fmt::Debug for Index<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Index").field(&self.0).finish()
    }
}
impl<T: ?Sized> fmt::Display for Index<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(&self.0, f) }
}

impl<T: ?Sized> Clone for Index<T> {
    #[inline]
    fn clone(&self) -> Self { *self }
}
impl<T: ?Sized> Copy for Index<T> {}

impl<T: ?Sized> PartialEq for Index<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}
impl<T: ?Sized> Eq for Index<T> {}

impl<T: ?Sized> PartialOrd for Index<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> { Some(self.cmp(other)) }
}
impl<T: ?Sized> Ord for Index<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering { self.0.cmp(&other.0) }
}

impl<T: ?Sized + 'static> hash::Hash for Index<T> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        TypeId::of::<T>().hash(state);
        self.0.hash(state);
    }
}

impl<T: ?Sized> From<usize> for Index<T> {
    #[inline]
    fn from(value: usize) -> Self { Self::new(value) }
}
impl<T: ?Sized> From<Index<T>> for usize {
    #[inline]
    fn from(index: Index<T>) -> Self { index.0 }
}

// -------------------------------------------------------------------------------------------------

/// The [`Index`] of an NBT value.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum ValueIndex {
    /// A [`u8`] value.
    Byte(Index<u8>),
    /// A [`u16`] value.
    Short(Index<u16>),
    /// A [`u32`] value.
    Int(Index<u32>),
    /// A [`u64`] value.
    Long(Index<u64>),
    /// A [`f32`] value.
    Float(Index<f32>),
    /// A [`f64`] value.
    Double(Index<f64>),
    /// A [`u8`] array.
    ByteArray(Index<[u8]>),
    /// An [`MStr`] string.
    String(Index<MStr>),
    /// A list of values.
    List(Index<IndexedListType>),
    /// A compound of named entries.
    Compound(Index<IndexedMapType>),
    /// A [`u32`] array.
    IntArray(Index<[u32]>),
    /// A [`u64`] array.
    LongArray(Index<[u64]>),
}

/// A marker type for a list of values.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct IndexedListType;

/// A marker type for a map of named values.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct IndexedMapType;

impl ValueIndex {
    /// Get the raw index value.
    #[inline]
    #[must_use]
    pub const fn index(&self) -> usize {
        match self {
            Self::Byte(index) => index.value(),
            Self::Short(index) => index.value(),
            Self::Int(index) => index.value(),
            Self::Long(index) => index.value(),
            Self::Float(index) => index.value(),
            Self::Double(index) => index.value(),
            Self::ByteArray(index) => index.value(),
            Self::String(index) => index.value(),
            Self::List(index) => index.value(),
            Self::Compound(index) => index.value(),
            Self::IntArray(index) => index.value(),
            Self::LongArray(index) => index.value(),
        }
    }
}

impl PartialOrd for ValueIndex {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for ValueIndex {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering { self.index().cmp(&other.index()) }
}

// -------------------------------------------------------------------------------------------------

/// A pair of name and value [`Index`]es.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct EntryIndex {
    name: Index<MStr>,
    value: ValueIndex,
}

impl EntryIndex {
    /// Create a new [`EntryIndex`] with the given name and value [`Index`]es.
    #[inline]
    #[must_use]
    pub const fn new(name: Index<MStr>, value: ValueIndex) -> Self { Self { name, value } }

    /// Get the [`Index`] of the name of this entry.
    #[inline]
    #[must_use]
    pub const fn name(&self) -> Index<MStr> { self.name }

    /// Get the [`Index`] of the value of this entry.
    #[inline]
    #[must_use]
    pub const fn value(&self) -> ValueIndex { self.value }
}
