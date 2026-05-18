//! TODO

use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};

use froglight_mutf8::prelude::{MStr, MString};

mod deserialize;
mod serialize;

/// An NBT structure.
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct Nbt {
    name: Option<MString>,
    root: NbtCompound,
}

impl Nbt {
    /// Create a new [`Nbt`] structure.
    #[inline]
    #[must_use]
    pub const fn new(name: Option<MString>, root: NbtCompound) -> Self { Self { name, root } }

    /// Get the name of this structure, if it has one.
    #[inline]
    #[must_use]
    pub const fn name(&self) -> Option<&MStr> {
        if let Some(name) = &self.name { Some(name.as_mstr()) } else { None }
    }

    /// Get the name of this structure mutably, if it has one.
    #[inline]
    #[must_use]
    pub const fn name_mut(&mut self) -> Option<&mut MString> {
        if let Some(name) = &mut self.name { Some(name) } else { None }
    }

    /// Set the name of this structure.
    #[inline]
    pub fn name_set(&mut self, name: Option<MString>) { self.name = name; }

    /// Get the root compound of this structure.
    #[inline]
    #[must_use]
    pub const fn compound(&self) -> &NbtCompound { &self.root }

    /// Get the root compound of this structure mutably.
    #[inline]
    #[must_use]
    pub const fn compound_mut(&mut self) -> &mut NbtCompound { &mut self.root }
}

impl AsRef<NbtCompound> for Nbt {
    #[inline]
    fn as_ref(&self) -> &NbtCompound { &self.root }
}
impl AsMut<NbtCompound> for Nbt {
    #[inline]
    fn as_mut(&mut self) -> &mut NbtCompound { &mut self.root }
}

impl Deref for Nbt {
    type Target = NbtCompound;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.root }
}
impl DerefMut for Nbt {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.root }
}

// -------------------------------------------------------------------------------------------------

/// An NBT compound.
#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NbtCompound {
    entries: Vec<(MString, NbtValue)>,
}

impl NbtCompound {
    /// Create a new, empty [`NbtCompound`].
    #[inline]
    #[must_use]
    pub const fn new() -> Self { Self { entries: Vec::new() } }

    /// Returns the number of elements in the compound,
    /// also referred to as its ‘length’
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize { self.entries.len() }

    /// Returns `true` if the compound contains no elements
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.entries.is_empty() }

    /// Get the value associated with the given key, if it exists.
    #[must_use]
    pub fn get<K: PartialEq<MStr> + ?Sized>(&self, key: &K) -> Option<&NbtValue> {
        self.entries.iter().find(|(k, _)| key == k.as_mstr()).map(|(_, v)| v)
    }

    /// Get the value associated with the given key mutably, if it exists.
    #[must_use]
    pub fn get_mut<K: PartialEq<MStr> + ?Sized>(&mut self, key: &K) -> Option<&mut NbtValue> {
        self.entries.iter_mut().find(|(k, _)| key == k.as_mstr()).map(|(_, v)| v)
    }

    /// Get the key-value pair at the given index, if it exists.
    #[must_use]
    pub fn get_index(&self, index: usize) -> Option<(&MStr, &NbtValue)> {
        self.entries.get(index).map(|(k, v)| (k.as_mstr(), v))
    }

    /// Get the key-value pair at the given index mutably, if it exists.
    #[must_use]
    pub fn get_index_mut(&mut self, index: usize) -> Option<(&mut MString, &mut NbtValue)> {
        self.entries.get_mut(index).map(|(k, v)| (k, v))
    }

    /// Insert a value into this compound,
    /// returning the previous value if it existed.
    pub fn insert<K: Into<MString>, V: Into<NbtValue>>(
        &mut self,
        key: K,
        value: V,
    ) -> Option<NbtValue> {
        let key = key.into();
        if let Some((_, v)) = self.entries.iter_mut().find(|(k, _)| *k == key) {
            Some(core::mem::replace(v, value.into()))
        } else {
            self.entries.push((key, value.into()));
            None
        }
    }

    /// Remove a value from this compound,
    /// returning it if it existed.
    pub fn remove<K: PartialEq<MStr> + ?Sized>(&mut self, key: &K) -> Option<NbtValue> {
        if let Some(pos) = self.entries.iter().position(|(k, _)| key == k.as_mstr()) {
            Some(self.entries.remove(pos).1)
        } else {
            None
        }
    }
}

impl<V: Into<NbtValue>> FromIterator<(MString, V)> for NbtCompound {
    fn from_iter<T: IntoIterator<Item = (MString, V)>>(iter: T) -> Self {
        Self { entries: iter.into_iter().map(|(k, v)| (k, v.into())).collect() }
    }
}
impl IntoIterator for NbtCompound {
    type IntoIter = alloc::vec::IntoIter<Self::Item>;
    type Item = (MString, NbtValue);

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.entries.into_iter() }
}

// -------------------------------------------------------------------------------------------------

macro_rules! impl_from_into {
    ($($ty:ty: $enum:ident::$variant:ident => $fn_name:ident, $fn_mut_name:ident),+) => {
        $(
            impl $enum {
                #[inline]
                #[must_use]
                #[doc = concat!("Get the value as a [`", stringify!($ty), "`], if it is one.")]
                pub const fn $fn_name(&self) -> Option<&$ty> {
                    if let $enum::$variant(value) = self { Some(value) } else { None }
                }

                #[inline]
                #[must_use]
                #[doc = concat!("Get the value as a mutable [`", stringify!($ty), "`], if it is one.")]
                pub const fn $fn_mut_name(&mut self) -> Option<&mut $ty> {
                    if let $enum::$variant(value) = self { Some(value) } else { None }
                }
            }
        )*

        $(
            impl From<$ty> for $enum {
                #[inline]
                fn from(value: $ty) -> Self { $enum::$variant(value) }
            }

            impl TryInto<$ty> for $enum {
                type Error = Self;

                #[inline]
                fn try_into(self) -> Result<$ty, Self::Error> {
                    if let $enum::$variant(value) = self { Ok(value) } else { Err(self) }
                }
            }
        )*
    };
}

// -------------------------------------------------------------------------------------------------

/// An NBT list.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum NbtList {
    /// An empty list.
    Empty,
    /// A list of [`u8`]s.
    Byte(Vec<u8>),
    /// A list of [`u16`]s.
    Short(Vec<u16>),
    /// A list of [`u32`]s.
    Int(Vec<u32>),
    /// A list of [`u64`]s.
    Long(Vec<u64>),
    /// A list of [`f32`]s.
    Float(Vec<f32>),
    /// A list of [`f64`]s.
    Double(Vec<f64>),
    /// A list of [`Vec<u8>`]s.
    ByteArray(Vec<Vec<u8>>),
    /// A list of [`MString`]s.
    String(Vec<MString>),
    /// A list of [`NbtList`]s.
    List(Vec<NbtList>),
    /// A list of [`NbtCompound`]s.
    Compound(Vec<NbtCompound>),
    /// A list of [`Vec<u32>`]s.
    IntArray(Vec<Vec<u32>>),
    /// A list of [`Vec<u64>`]s.
    LongArray(Vec<Vec<u64>>),
}

impl_from_into! {
    Vec<u8>: NbtList::Byte => as_byte, as_byte_mut,
    Vec<u16>: NbtList::Short => as_short, as_short_mut,
    Vec<u32>: NbtList::Int => as_int, as_int_mut,
    Vec<u64>: NbtList::Long => as_long, as_long_mut,
    Vec<f32>: NbtList::Float => as_float, as_float_mut,
    Vec<f64>: NbtList::Double => as_double, as_double_mut,
    Vec<Vec<u8>>: NbtList::ByteArray => as_byte_array, as_byte_array_mut,
    Vec<MString>: NbtList::String => as_string, as_string_mut,
    Vec<NbtList>: NbtList::List => as_list, as_list_mut,
    Vec<NbtCompound>: NbtList::Compound => as_compound, as_compound_mut,
    Vec<Vec<u32>>: NbtList::IntArray => as_int_array, as_int_array_mut,
    Vec<Vec<u64>>: NbtList::LongArray => as_long_array, as_long_array_mut
}

impl NbtList {
    /// Returns the number of elements in the list,
    /// also referred to as its ‘length’
    #[must_use]
    pub const fn len(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Byte(vec) => vec.len(),
            Self::Short(vec) => vec.len(),
            Self::Int(vec) => vec.len(),
            Self::Long(vec) => vec.len(),
            Self::Float(vec) => vec.len(),
            Self::Double(vec) => vec.len(),
            Self::ByteArray(vec) => vec.len(),
            Self::String(vec) => vec.len(),
            Self::List(vec) => vec.len(),
            Self::Compound(vec) => vec.len(),
            Self::IntArray(vec) => vec.len(),
            Self::LongArray(vec) => vec.len(),
        }
    }

    /// Returns `true` if the list contains no elements
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            Self::Byte(vec) => vec.is_empty(),
            Self::Short(vec) => vec.is_empty(),
            Self::Int(vec) => vec.is_empty(),
            Self::Long(vec) => vec.is_empty(),
            Self::Float(vec) => vec.is_empty(),
            Self::Double(vec) => vec.is_empty(),
            Self::ByteArray(vec) => vec.is_empty(),
            Self::String(vec) => vec.is_empty(),
            Self::List(vec) => vec.is_empty(),
            Self::Compound(vec) => vec.is_empty(),
            Self::IntArray(vec) => vec.is_empty(),
            Self::LongArray(vec) => vec.is_empty(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// An NBT value.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum NbtValue {
    /// A [`u8`] value.
    Byte(u8),
    /// A [`u16`] value.
    Short(u16),
    /// A [`u32`] value.
    Int(u32),
    /// A [`u64`] value.
    Long(u64),
    /// A [`f32`] value.
    Float(f32),
    /// A [`f64`] value.
    Double(f64),
    /// A [`Vec<u8>`] value.
    ByteArray(Vec<u8>),
    /// A [`MString`] value.
    String(MString),
    /// A [`NbtList`] value.
    List(NbtList),
    /// A [`NbtCompound`] value.
    Compound(NbtCompound),
    /// A [`Vec<u32>`] value.
    IntArray(Vec<u32>),
    /// A [`Vec<u64>`] value.
    LongArray(Vec<u64>),
}

impl_from_into! {
    u8: NbtValue::Byte => as_byte, as_byte_mut,
    u16: NbtValue::Short => as_short, as_short_mut,
    u32: NbtValue::Int => as_int, as_int_mut,
    u64: NbtValue::Long => as_long, as_long_mut,
    f32: NbtValue::Float => as_float, as_float_mut,
    f64: NbtValue::Double => as_double, as_double_mut,
    Vec<u8>: NbtValue::ByteArray => as_byte_array, as_byte_array_mut,
    MString: NbtValue::String => as_string, as_string_mut,
    NbtList: NbtValue::List => as_list, as_list_mut,
    NbtCompound: NbtValue::Compound => as_compound, as_compound_mut,
    Vec<u32>: NbtValue::IntArray => as_int_array, as_int_array_mut,
    Vec<u64>: NbtValue::LongArray => as_long_array, as_long_array_mut
}
