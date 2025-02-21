use derive_more::{From, Into, IsVariant, TryInto, TryUnwrap, Unwrap};
use indexmap::IndexMap;

use crate::mutf8::Mutf8String;

/// A map of named NBT tags.
#[derive(Debug, Default, Clone, PartialEq, From, Into)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
pub struct NbtCompound(IndexMap<Mutf8String, NbtTag>);

impl NbtCompound {
    /// Create a new empty [`NbtCompound`].
    #[inline]
    #[must_use]
    pub fn new() -> Self { Self(IndexMap::new()) }

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
#[cfg_attr(feature = "serde", expect(clippy::unsafe_derive_deserialize))]
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
    /// The end of a [`NbtTag::Compound`] or [`NbtTag::List`].
    pub const END: u8 = 0;
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

    /// Unwrap this value to the [`NbtTag::Byte`] variant. Panics if the value
    /// is of any other type.
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    pub fn unwrap_unsigned_byte(self) -> u8 { self.unwrap_byte() as u8 }

    /// Unwrap this value to the [`NbtTag::Short`] variant. Panics if the value
    /// is of any other type.
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    pub fn unwrap_unsigned_short(self) -> u16 { self.unwrap_short() as u16 }

    /// Unwrap this value to the [`NbtTag::Int`] variant. Panics if the value
    /// is of any other type.
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    pub fn unwrap_unsigned_int(self) -> u32 { self.unwrap_int() as u32 }

    /// Unwrap this value to the [`NbtTag::Long`] variant. Panics if the value
    /// is of any other type.
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    pub fn unwrap_unsigned_long(self) -> u64 { self.unwrap_long() as u64 }

    /// Unwrap this value to the [`NbtTag::ByteArray`] variant. Panics if the
    /// value is of any other type.
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    pub fn unwrap_unsigned_byte_array(self) -> Vec<u8> {
        self.unwrap_byte_array().into_iter().map(|b| b as u8).collect()
    }

    /// Unwrap this value to the [`NbtTag::IntArray`] variant. Panics if the
    /// value is of any other type.
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    pub fn unwrap_unsigned_int_array(self) -> Vec<u32> {
        self.unwrap_int_array().into_iter().map(|i| i as u32).collect()
    }

    /// Unwrap this value to the [`NbtTag::LongArray`] variant. Panics if the
    /// value is of any other type.
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    pub fn unwrap_unsigned_long_array(self) -> Vec<u64> {
        self.unwrap_long_array().into_iter().map(|l| l as u64).collect()
    }

    /// Get the value of a [`NbtTag::Byte`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_byte(&self) -> Option<i8> {
        if let NbtTag::Byte(byte) = self { Some(*byte) } else { None }
    }

    /// Get the value of a [`NbtTag::Short`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[inline]
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    pub fn as_unsigned_byte(&self) -> Option<u8> { self.as_byte().map(|byte| byte as u8) }

    /// Get the value of a [`NbtTag::Short`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_short(&self) -> Option<i16> {
        if let NbtTag::Short(short) = self { Some(*short) } else { None }
    }

    /// Get the value of a [`NbtTag::Short`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[inline]
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    pub fn as_unsigned_short(&self) -> Option<u16> { self.as_short().map(|short| short as u16) }

    /// Get the value of a [`NbtTag::Int`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_int(&self) -> Option<i32> {
        if let NbtTag::Int(int) = self { Some(*int) } else { None }
    }

    /// Get the value of a [`NbtTag::Int`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[inline]
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    pub fn as_unsigned_int(&self) -> Option<u32> { self.as_int().map(|int| int as u32) }

    /// Get the value of a [`NbtTag::Long`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_long(&self) -> Option<i64> {
        if let NbtTag::Long(long) = self { Some(*long) } else { None }
    }

    /// Get the value of a [`NbtTag::Long`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[inline]
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    pub fn as_unsigned_long(&self) -> Option<u64> { self.as_long().map(|long| long as u64) }

    /// Get the value of a [`NbtTag::Float`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_float(&self) -> Option<f32> {
        if let NbtTag::Float(float) = self { Some(*float) } else { None }
    }

    /// Get the value of a [`NbtTag::Double`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_double(&self) -> Option<f64> {
        if let NbtTag::Double(double) = self { Some(*double) } else { None }
    }

    /// Get the value of a [`NbtTag::ByteArray`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_byte_array(&self) -> Option<&[i8]> {
        if let NbtTag::ByteArray(array) = self { Some(array) } else { None }
    }

    /// Get the value of a [`NbtTag::ByteArray`] variant mutably, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_byte_array_mut(&mut self) -> Option<&mut [i8]> {
        if let NbtTag::ByteArray(array) = self { Some(array) } else { None }
    }

    /// Get the value of a [`NbtTag::ByteArray`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    ///
    /// TODO: Check if this is safe
    #[must_use]
    pub fn as_unsigned_byte_array(&self) -> Option<&[u8]> {
        self.as_byte_array().map(|array| unsafe {
            std::slice::from_raw_parts(array.as_ptr().cast::<u8>(), array.len())
        })
    }

    /// Get the value of a [`NbtTag::ByteArray`] variant mutably, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    ///
    /// TODO: Check if this is safe
    #[must_use]
    pub fn as_unsigned_byte_array_mut(&mut self) -> Option<&mut [u8]> {
        self.as_byte_array_mut().map(|array| unsafe {
            std::slice::from_raw_parts_mut(array.as_mut_ptr().cast::<u8>(), array.len())
        })
    }

    /// Get the value of a [`NbtTag::String`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_string(&self) -> Option<&Mutf8String> {
        if let NbtTag::String(string) = self { Some(string) } else { None }
    }

    /// Get the value of a [`NbtTag::String`] variant mutably, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_string_mut(&mut self) -> Option<&mut Mutf8String> {
        if let NbtTag::String(string) = self { Some(string) } else { None }
    }

    /// Get the value of a [`NbtTag::List`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_list(&self) -> Option<&NbtListTag> {
        if let NbtTag::List(list) = self { Some(list) } else { None }
    }

    /// Get the value of a [`NbtTag::List`] variant mutably, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_list_mut(&mut self) -> Option<&mut NbtListTag> {
        if let NbtTag::List(list) = self { Some(list) } else { None }
    }

    /// Get the value of a [`NbtTag::Compound`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_compound(&self) -> Option<&NbtCompound> {
        if let NbtTag::Compound(compound) = self { Some(compound) } else { None }
    }

    /// Get the value of a [`NbtTag::Compound`] variant mutably, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_compound_mut(&mut self) -> Option<&mut NbtCompound> {
        if let NbtTag::Compound(compound) = self { Some(compound) } else { None }
    }

    /// Get the value of a [`NbtTag::IntArray`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_int_array(&self) -> Option<&[i32]> {
        if let NbtTag::IntArray(array) = self { Some(array) } else { None }
    }

    /// Get the value of a [`NbtTag::IntArray`] variant mutably, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_int_array_mut(&mut self) -> Option<&mut [i32]> {
        if let NbtTag::IntArray(array) = self { Some(array) } else { None }
    }

    /// Get the value of a [`NbtTag::IntArray`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    ///
    /// TODO: Check if this is safe
    #[must_use]
    pub fn as_unsigned_int_array(&self) -> Option<&[u32]> {
        self.as_int_array().map(|array| unsafe {
            std::slice::from_raw_parts(array.as_ptr().cast::<u32>(), array.len())
        })
    }

    /// Get the value of a [`NbtTag::IntArray`] variant mutably, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    ///
    /// TODO: Check if this is safe
    #[must_use]
    pub fn as_unsigned_int_array_mut(&mut self) -> Option<&mut [u32]> {
        self.as_int_array_mut().map(|array| unsafe {
            std::slice::from_raw_parts_mut(array.as_mut_ptr().cast::<u32>(), array.len())
        })
    }

    /// Get the value of a [`NbtTag::LongArray`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_long_array(&self) -> Option<&[i64]> {
        if let NbtTag::LongArray(array) = self { Some(array) } else { None }
    }

    /// Get the value of a [`NbtTag::LongArray`] variant mutably, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_long_array_mut(&mut self) -> Option<&mut [i64]> {
        if let NbtTag::LongArray(array) = self { Some(array) } else { None }
    }

    /// Get the value of a [`NbtTag::LongArray`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    ///
    /// TODO: Check if this is safe
    #[must_use]
    pub fn as_unsigned_long_array(&self) -> Option<&[u64]> {
        self.as_long_array().map(|array| unsafe {
            std::slice::from_raw_parts(array.as_ptr().cast::<u64>(), array.len())
        })
    }

    /// Get the value of a [`NbtTag::LongArray`] variant mutably, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    ///
    /// TODO: Check if this is safe
    #[must_use]
    pub fn as_unsigned_long_array_mut(&mut self) -> Option<&mut [u64]> {
        self.as_long_array_mut().map(|array| unsafe {
            std::slice::from_raw_parts_mut(array.as_mut_ptr().cast::<u64>(), array.len())
        })
    }
}

impl<'a> std::ops::Index<&'a str> for NbtTag {
    type Output = NbtTag;
    fn index(&self, key: &'a str) -> &Self::Output {
        if let NbtTag::Compound(compound) = self {
            &compound[key]
        } else {
            panic!("NbtTag is not a Compound")
        }
    }
}
impl<'a> std::ops::IndexMut<&'a str> for NbtTag {
    fn index_mut(&mut self, key: &'a str) -> &mut Self::Output {
        if let NbtTag::Compound(compound) = self {
            &mut compound[key]
        } else {
            panic!("NbtTag is not a Compound")
        }
    }
}

impl std::ops::Index<usize> for NbtTag {
    type Output = NbtTag;
    fn index(&self, index: usize) -> &Self::Output {
        if let NbtTag::Compound(compound) = self {
            &compound[index]
        } else {
            panic!("NbtTag is not a Compound")
        }
    }
}
impl std::ops::IndexMut<usize> for NbtTag {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if let NbtTag::Compound(compound) = self {
            &mut compound[index]
        } else {
            panic!("NbtTag is not a Compound")
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A list of NBT tag values.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, From, TryInto, IsVariant, Unwrap, TryUnwrap)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(untagged))]
pub enum NbtListTag {
    /// An empty list.
    Empty = NbtTag::END,
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
            NbtListTag::Empty => NbtTag::END,
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

    /// Get the value of a [`NbtListTag::Byte`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_byte(&self) -> Option<&[i8]> {
        if let NbtListTag::Byte(byte) = self { Some(byte) } else { None }
    }

    /// Get the value of a [`NbtListTag::Byte`] variant mutably, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_byte_mut(&mut self) -> Option<&mut Vec<i8>> {
        if let NbtListTag::Byte(byte) = self { Some(byte) } else { None }
    }

    /// Get the value of a [`NbtListTag::Short`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_short(&self) -> Option<&[i16]> {
        if let NbtListTag::Short(short) = self { Some(short) } else { None }
    }

    /// Get the value of a [`NbtListTag::Short`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_short_mut(&mut self) -> Option<&mut Vec<i16>> {
        if let NbtListTag::Short(short) = self { Some(short) } else { None }
    }

    /// Get the value of a [`NbtListTag::Int`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_int(&self) -> Option<&[i32]> {
        if let NbtListTag::Int(int) = self { Some(int) } else { None }
    }

    /// Get the value of a [`NbtListTag::Int`] variant mutably, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_int_mut(&mut self) -> Option<&mut Vec<i32>> {
        if let NbtListTag::Int(int) = self { Some(int) } else { None }
    }

    /// Get the value of a [`NbtListTag::Long`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_long(&self) -> Option<&[i64]> {
        if let NbtListTag::Long(long) = self { Some(long) } else { None }
    }

    /// Get the value of a [`NbtListTag::Long`] variant mutably, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_long_mut(&mut self) -> Option<&mut Vec<i64>> {
        if let NbtListTag::Long(long) = self { Some(long) } else { None }
    }

    /// Get the value of a [`NbtListTag::Float`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_float(&self) -> Option<&[f32]> {
        if let NbtListTag::Float(float) = self { Some(float) } else { None }
    }

    /// Get the value of a [`NbtListTag::Float`] variant mutably, if it is one.
    ///
    ///    Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_float_mut(&mut self) -> Option<&mut Vec<f32>> {
        if let NbtListTag::Float(float) = self { Some(float) } else { None }
    }

    /// Get the value of a [`NbtListTag::Double`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_double(&self) -> Option<&[f64]> {
        if let NbtListTag::Double(double) = self { Some(double) } else { None }
    }

    /// Get the value of a [`NbtListTag::Double`] variant mutably, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_double_mut(&mut self) -> Option<&mut Vec<f64>> {
        if let NbtListTag::Double(double) = self { Some(double) } else { None }
    }

    /// Get the value of a [`NbtListTag::ByteArray`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_byte_array(&self) -> Option<&[Vec<i8>]> {
        if let NbtListTag::ByteArray(array) = self { Some(array) } else { None }
    }

    /// Get the value of a [`NbtListTag::ByteArray`] variant mutably, if it is
    /// one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_byte_array_mut(&mut self) -> Option<&mut Vec<Vec<i8>>> {
        if let NbtListTag::ByteArray(array) = self { Some(array) } else { None }
    }

    /// Get the value of a [`NbtListTag::String`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_string(&self) -> Option<&[Mutf8String]> {
        if let NbtListTag::String(string) = self { Some(string) } else { None }
    }

    /// Get the value of a [`NbtListTag::String`] variant mutably, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_string_mut(&mut self) -> Option<&mut Vec<Mutf8String>> {
        if let NbtListTag::String(string) = self { Some(string) } else { None }
    }

    /// Get the value of a [`NbtListTag::List`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_list(&self) -> Option<&[NbtListTag]> {
        if let NbtListTag::List(list) = self { Some(list) } else { None }
    }

    /// Get the value of a [`NbtListTag::List`] variant mutably, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_list_mut(&mut self) -> Option<&mut Vec<NbtListTag>> {
        if let NbtListTag::List(list) = self { Some(list) } else { None }
    }

    /// Get the value of a [`NbtListTag::Compound`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_compound(&self) -> Option<&[NbtCompound]> {
        if let NbtListTag::Compound(compound) = self { Some(compound) } else { None }
    }

    /// Get the value of a [`NbtListTag::Compound`] variant mutably, if it is
    /// one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_compound_mut(&mut self) -> Option<&mut Vec<NbtCompound>> {
        if let NbtListTag::Compound(compound) = self { Some(compound) } else { None }
    }

    /// Get the value of a [`NbtListTag::IntArray`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_int_array(&self) -> Option<&[Vec<i32>]> {
        if let NbtListTag::IntArray(array) = self { Some(array) } else { None }
    }

    /// Get the value of a [`NbtListTag::IntArray`] variant mutably, if it is
    /// one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_int_array_mut(&mut self) -> Option<&mut Vec<Vec<i32>>> {
        if let NbtListTag::IntArray(array) = self { Some(array) } else { None }
    }

    /// Get the value of a [`NbtListTag::LongArray`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_long_array(&self) -> Option<&[Vec<i64>]> {
        if let NbtListTag::LongArray(array) = self { Some(array) } else { None }
    }

    /// Get the value of a [`NbtListTag::LongArray`] variant mutably, if it is
    /// one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_long_array_mut(&mut self) -> Option<&mut Vec<Vec<i64>>> {
        if let NbtListTag::LongArray(array) = self { Some(array) } else { None }
    }
}
