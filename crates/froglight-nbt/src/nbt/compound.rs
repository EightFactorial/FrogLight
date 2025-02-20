use derive_more::{From, Into, IsVariant, TryInto, TryUnwrap, Unwrap};
use indexmap::IndexMap;

use crate::mutf8::Mutf8String;

/// A map of named NBT tags.
#[derive(Debug, Default, Clone, PartialEq, From, Into)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
pub struct NbtCompound(IndexMap<Mutf8String, NbtTag>);

impl NbtCompound {
    /// Return the number of tags in the [`NbtCompound`].
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize { self.0.len() }

    /// Return `true` if the [`NbtCompound`] contains no tags.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Get a reference to a [`NbtTag`] by it's key.
    #[inline]
    #[must_use]
    pub fn get_tag<Q: AsRef<str> + ?Sized>(&self, key: &Q) -> Option<&NbtTag> {
        self.0.get(key.as_ref().as_bytes())
    }

    /// Get a mutable reference to a [`NbtTag`] by it's key.
    #[inline]
    #[must_use]
    pub fn get_tag_mut<Q: AsRef<str> + ?Sized>(&mut self, key: &Q) -> Option<&mut NbtTag> {
        self.0.get_mut(key.as_ref().as_bytes())
    }

    /// Get a reference to a [`NbtTag`] by it's index.
    #[inline]
    #[must_use]
    pub fn get_index(&self, index: usize) -> Option<(&Mutf8String, &NbtTag)> {
        self.0.get_index(index)
    }

    /// Get a mutable reference to a [`NbtTag`] by it's index.
    #[inline]
    #[must_use]
    pub fn get_index_mut(&mut self, index: usize) -> Option<(&Mutf8String, &mut NbtTag)> {
        self.0.get_index_mut(index)
    }

    /// Get the key's entry in the [`NbtCompound`] for
    /// insertion or in-place manipulation.
    #[inline]
    #[must_use]
    pub fn entry(
        &mut self,
        key: impl Into<Mutf8String>,
    ) -> indexmap::map::Entry<Mutf8String, NbtTag> {
        self.0.entry(key.into())
    }

    /// Get the index's entry in the [`NbtCompound`] for
    /// in-place manipulation.
    ///
    /// Returns `None` if the index is out of bounds.
    #[inline]
    #[must_use]
    pub fn entry_index(
        &mut self,
        index: usize,
    ) -> Option<indexmap::map::IndexedEntry<Mutf8String, NbtTag>> {
        self.0.get_index_entry(index)
    }

    /// Insert a new key-value pair into the [`NbtCompound`].
    ///
    /// Returns the previous value associated with the key, if any.
    #[inline]
    pub fn insert(&mut self, key: impl Into<Mutf8String>, tag: NbtTag) -> Option<NbtTag> {
        self.0.insert(key.into(), tag)
    }

    /// Insert a new key-value pair into the [`NbtCompound`] at the given index,
    /// shifting all other elements to the right.
    ///
    /// Returns the previous value associated with the key, if any.
    ///
    /// See [`IndexMap::shift_insert`] for more information.
    #[inline]
    pub fn shift_insert(
        &mut self,
        index: usize,
        key: impl Into<Mutf8String>,
        tag: NbtTag,
    ) -> Option<NbtTag> {
        self.0.shift_insert(index, key.into(), tag)
    }

    /// Remove a key-value pair from the [`NbtCompound`] by it's key,
    /// shifting all other elements to the left.
    ///
    /// Returns the removed value, if any.
    ///
    /// See [`IndexMap::shift_remove`] for more information.
    #[inline]
    pub fn shift_remove<Q: ?Sized + AsRef<str>>(&mut self, key: &Q) -> Option<NbtTag> {
        self.0.shift_remove(key.as_ref().as_bytes())
    }

    /// Remove a key-value pair from the [`NbtCompound`] by it's index,
    /// swapping the last element into it's place.
    ///
    /// Returns the removed value, if any.
    ///
    /// See [`IndexMap::swap_remove`] for more information.
    #[inline]
    pub fn swap_remove<Q: ?Sized + AsRef<str>>(&mut self, key: &Q) -> Option<NbtTag> {
        self.0.swap_remove(key.as_ref().as_bytes())
    }

    /// Return an iterator over the name-tag pairs of the compound,
    /// in their order
    #[inline]
    #[must_use]
    pub fn iter(&self) -> indexmap::map::Iter<Mutf8String, NbtTag> { self.0.iter() }

    /// Return an iterator over the key-value pairs of the map,
    /// in their order
    #[inline]
    #[must_use]
    pub fn iter_mut(&mut self) -> indexmap::map::IterMut<Mutf8String, NbtTag> { self.0.iter_mut() }
}

impl<'a> std::iter::IntoIterator for &'a NbtCompound {
    type Item = (&'a Mutf8String, &'a NbtTag);
    type IntoIter = indexmap::map::Iter<'a, Mutf8String, NbtTag>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl<'a> std::iter::IntoIterator for &'a mut NbtCompound {
    type Item = (&'a Mutf8String, &'a mut NbtTag);
    type IntoIter = indexmap::map::IterMut<'a, Mutf8String, NbtTag>;
    fn into_iter(self) -> Self::IntoIter { self.iter_mut() }
}

impl FromIterator<(Mutf8String, NbtTag)> for NbtCompound {
    fn from_iter<T: IntoIterator<Item = (Mutf8String, NbtTag)>>(iter: T) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}

impl<'a> std::ops::Index<&'a str> for NbtCompound {
    type Output = NbtTag;
    fn index(&self, key: &'a str) -> &Self::Output {
        self.get_tag(key).expect("Compound does not contain key")
    }
}
impl<'a> std::ops::IndexMut<&'a str> for NbtCompound {
    fn index_mut(&mut self, key: &'a str) -> &mut Self::Output {
        self.get_tag_mut(key).expect("Compound does not contain key")
    }
}

impl std::ops::Index<usize> for NbtCompound {
    type Output = NbtTag;
    fn index(&self, index: usize) -> &Self::Output {
        self.get_index(index).map(|(_, tag)| tag).expect("Compound does not contain index")
    }
}
impl std::ops::IndexMut<usize> for NbtCompound {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_index_mut(index).map(|(_, tag)| tag).expect("Compound does not contain index")
    }
}

// -------------------------------------------------------------------------------------------------

/// A NBT tag.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, From, TryInto, IsVariant, Unwrap, TryUnwrap)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(untagged))]
pub enum NbtTag {
    /// A signed 8-bit integer.
    Byte(i8) = NbtTag::BYTE,
    /// A signed 16-bit integer.
    Short(i16) = NbtTag::SHORT,
    /// A signed 32-bit integer.
    Int(i32) = NbtTag::INT,
    /// A signed 64-bit integer.
    Long(i64) = NbtTag::LONG,
    /// A 32-bit floating point number.
    Float(f32) = NbtTag::FLOAT,
    /// A 64-bit floating point number.
    Double(f64) = NbtTag::DOUBLE,
    /// An array of signed 8-bit integers.
    ByteArray(Vec<i8>) = NbtTag::BYTE_ARRAY,
    /// A MUTF-8 string.
    String(Mutf8String) = NbtTag::STRING,
    /// A [`NbtListTag`].
    List(NbtListTag) = NbtTag::LIST,
    /// An [`NbtCompound`].
    Compound(NbtCompound) = NbtTag::COMPOUND,
    /// An array of signed 32-bit integers.
    IntArray(Vec<i32>) = NbtTag::INT_ARRAY,
    /// An array of signed 64-bit integers.
    LongArray(Vec<i64>) = NbtTag::LONG_ARRAY,
}

impl NbtTag {
    /// The tag of a [`NbtTag::Byte`].
    pub const BYTE: u8 = 1;
    /// The tag of a [`NbtTag::Short`].
    pub const SHORT: u8 = 2;
    /// The tag of a [`NbtTag::Int`].
    pub const INT: u8 = 3;
    /// The tag of a [`NbtTag::Long`].
    pub const LONG: u8 = 4;
    /// The tag of a [`NbtTag::Float`].
    pub const FLOAT: u8 = 5;
    /// The tag of a [`NbtTag::Double`].
    pub const DOUBLE: u8 = 6;
    /// The tag of a [`NbtTag::ByteArray`].
    pub const BYTE_ARRAY: u8 = 7;
    /// The tag of a [`NbtTag::String`].
    pub const STRING: u8 = 8;
    /// The tag of a [`NbtTag::List`].
    pub const LIST: u8 = 9;
    /// The tag of a [`NbtTag::Compound`].
    pub const COMPOUND: u8 = 10;
    /// The tag of a [`NbtTag::IntArray`].
    pub const INT_ARRAY: u8 = 11;
    /// The tag of a [`NbtTag::LongArray`].
    pub const LONG_ARRAY: u8 = 12;

    /// Get the tag ID of the [`NbtTag`].
    #[must_use]
    pub const fn tag_id(&self) -> u8 {
        match self {
            NbtTag::Byte(_) => NbtTag::BYTE,
            NbtTag::Short(_) => NbtTag::SHORT,
            NbtTag::Int(_) => NbtTag::INT,
            NbtTag::Long(_) => NbtTag::LONG,
            NbtTag::Float(_) => NbtTag::FLOAT,
            NbtTag::Double(_) => NbtTag::DOUBLE,
            NbtTag::ByteArray(_) => NbtTag::BYTE_ARRAY,
            NbtTag::String(_) => NbtTag::STRING,
            NbtTag::List(_) => NbtTag::LIST,
            NbtTag::Compound(_) => NbtTag::COMPOUND,
            NbtTag::IntArray(_) => NbtTag::INT_ARRAY,
            NbtTag::LongArray(_) => NbtTag::LONG_ARRAY,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A list of NBT tag values.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, From, TryInto, IsVariant, Unwrap, TryUnwrap)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(untagged))]
pub enum NbtListTag {
    /// A list of signed 8-bit integers.
    Byte(Vec<i8>) = NbtTag::BYTE,
    /// A list of signed 16-bit integers.
    Short(Vec<i16>) = NbtTag::SHORT,
    /// A list of signed 32-bit integers.
    Int(Vec<i32>) = NbtTag::INT,
    /// A list of signed 64-bit integers.
    Long(Vec<i64>) = NbtTag::LONG,
    /// A list of 32-bit floating point numbers.
    Float(Vec<f32>) = NbtTag::FLOAT,
    /// A list of 64-bit floating point numbers.
    Double(Vec<f64>) = NbtTag::DOUBLE,
    /// A list of byte arrays.
    ByteArray(Vec<Vec<i8>>) = NbtTag::BYTE_ARRAY,
    /// A list of MUTF-8 strings.
    String(Vec<Mutf8String>) = NbtTag::STRING,
    /// A list of [`NbtTagList`]s.
    List(Vec<NbtListTag>) = NbtTag::LIST,
    /// A list of [`NbtCompound`]s.
    Compound(Vec<NbtCompound>) = NbtTag::COMPOUND,
    /// A list of signed 32-bit integers.
    IntArray(Vec<Vec<i32>>) = NbtTag::INT_ARRAY,
    /// A list of signed 64-bit integers.
    LongArray(Vec<Vec<i64>>) = NbtTag::LONG_ARRAY,
}

impl NbtListTag {
    /// Get the tag ID of the [`NbtListTag`].
    #[must_use]
    pub const fn tag_id(&self) -> u8 {
        match self {
            NbtListTag::Byte(_) => NbtTag::BYTE,
            NbtListTag::Short(_) => NbtTag::SHORT,
            NbtListTag::Int(_) => NbtTag::INT,
            NbtListTag::Long(_) => NbtTag::LONG,
            NbtListTag::Float(_) => NbtTag::FLOAT,
            NbtListTag::Double(_) => NbtTag::DOUBLE,
            NbtListTag::ByteArray(_) => NbtTag::BYTE_ARRAY,
            NbtListTag::String(_) => NbtTag::STRING,
            NbtListTag::List(_) => NbtTag::LIST,
            NbtListTag::Compound(_) => NbtTag::COMPOUND,
            NbtListTag::IntArray(_) => NbtTag::INT_ARRAY,
            NbtListTag::LongArray(_) => NbtTag::LONG_ARRAY,
        }
    }
}
