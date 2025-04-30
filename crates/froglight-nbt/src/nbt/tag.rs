// #[cfg(not(feature = "std"))]
// use alloc::{string::String, vec::Vec};

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{From, IsVariant, TryUnwrap, Unwrap};

use super::{ByteArray, DoubleArray, FloatArray, IntArray, LongArray, NbtCompound, ShortArray};
use crate::mutf8::Mutf8String;

/// A NBT tag.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, From, IsVariant, Unwrap, TryUnwrap)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(untagged))]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(no_field_bounds, Debug, PartialEq))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
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
    ByteArray(ByteArray) = NbtTag::BYTE_ARRAY,
    /// A MUTF-8 string.
    String(Mutf8String) = NbtTag::STRING,
    /// A [`NbtListTag`].
    List(NbtListTag) = NbtTag::LIST,
    /// An [`NbtCompound`].
    Compound(NbtCompound) = NbtTag::COMPOUND,
    /// An array of signed 32-bit integers.
    IntArray(IntArray) = NbtTag::INT_ARRAY,
    /// An array of signed 64-bit integers.
    LongArray(LongArray) = NbtTag::LONG_ARRAY,
}

#[rustfmt::skip]
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
}

impl NbtTag {
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
        Into::<Vec<i8>>::into(self.unwrap_byte_array()).into_iter().map(|b| b as u8).collect()
    }

    /// Unwrap this value to the [`NbtTag::IntArray`] variant. Panics if the
    /// value is of any other type.
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    pub fn unwrap_unsigned_int_array(self) -> Vec<u32> {
        Into::<Vec<i32>>::into(self.unwrap_int_array()).into_iter().map(|i| i as u32).collect()
    }

    /// Unwrap this value to the [`NbtTag::LongArray`] variant. Panics if the
    /// value is of any other type.
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    pub fn unwrap_unsigned_long_array(self) -> Vec<u64> {
        Into::<Vec<i64>>::into(self.unwrap_long_array()).into_iter().map(|l| l as u64).collect()
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
    pub fn as_byte_array(&self) -> Option<&ByteArray> {
        if let NbtTag::ByteArray(array) = self { Some(array) } else { None }
    }

    /// Get the value of a [`NbtTag::ByteArray`] variant mutably, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_byte_array_mut(&mut self) -> Option<&mut ByteArray> {
        if let NbtTag::ByteArray(array) = self { Some(array) } else { None }
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
    pub fn as_int_array(&self) -> Option<&IntArray> {
        if let NbtTag::IntArray(array) = self { Some(array) } else { None }
    }

    /// Get the value of a [`NbtTag::IntArray`] variant mutably, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_int_array_mut(&mut self) -> Option<&mut IntArray> {
        if let NbtTag::IntArray(array) = self { Some(array) } else { None }
    }

    /// Get the value of a [`NbtTag::LongArray`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_long_array(&self) -> Option<&LongArray> {
        if let NbtTag::LongArray(array) = self { Some(array) } else { None }
    }

    /// Get the value of a [`NbtTag::LongArray`] variant mutably, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_long_array_mut(&mut self) -> Option<&mut LongArray> {
        if let NbtTag::LongArray(array) = self { Some(array) } else { None }
    }
}

impl From<u8> for NbtTag {
    #[expect(clippy::cast_possible_wrap)]
    fn from(value: u8) -> Self { NbtTag::Byte(value as i8) }
}
impl From<u16> for NbtTag {
    #[expect(clippy::cast_possible_wrap)]
    fn from(value: u16) -> Self { NbtTag::Short(value as i16) }
}
impl From<u32> for NbtTag {
    #[expect(clippy::cast_possible_wrap)]
    fn from(value: u32) -> Self { NbtTag::Int(value as i32) }
}
impl From<u64> for NbtTag {
    #[expect(clippy::cast_possible_wrap)]
    fn from(value: u64) -> Self { NbtTag::Long(value as i64) }
}

impl From<Vec<i8>> for NbtTag {
    fn from(value: Vec<i8>) -> Self { NbtTag::ByteArray(ByteArray::from(value)) }
}
impl From<Vec<u8>> for NbtTag {
    fn from(value: Vec<u8>) -> Self { NbtTag::ByteArray(ByteArray::from(value)) }
}
impl From<Vec<i32>> for NbtTag {
    fn from(value: Vec<i32>) -> Self { NbtTag::IntArray(IntArray::from(value)) }
}
impl From<Vec<u32>> for NbtTag {
    fn from(value: Vec<u32>) -> Self { NbtTag::IntArray(IntArray::from(value)) }
}
impl From<Vec<i64>> for NbtTag {
    fn from(value: Vec<i64>) -> Self { NbtTag::LongArray(LongArray::from(value)) }
}
impl From<Vec<u64>> for NbtTag {
    fn from(value: Vec<u64>) -> Self { NbtTag::LongArray(LongArray::from(value)) }
}

impl From<String> for NbtTag {
    fn from(value: String) -> Self { NbtTag::String(value.into()) }
}
impl<'a> From<&'a str> for NbtTag {
    fn from(value: &'a str) -> Self { NbtTag::String(value.into()) }
}

impl<'a> core::ops::Index<&'a str> for NbtTag {
    type Output = NbtTag;

    fn index(&self, key: &'a str) -> &Self::Output {
        if let NbtTag::Compound(compound) = self {
            &compound[key]
        } else {
            panic!("NbtTag is not a Compound")
        }
    }
}
impl<'a> core::ops::IndexMut<&'a str> for NbtTag {
    fn index_mut(&mut self, key: &'a str) -> &mut Self::Output {
        if let NbtTag::Compound(compound) = self {
            &mut compound[key]
        } else {
            panic!("NbtTag is not a Compound")
        }
    }
}

impl core::ops::Index<usize> for NbtTag {
    type Output = NbtTag;

    fn index(&self, index: usize) -> &Self::Output {
        if let NbtTag::Compound(compound) = self {
            &compound[index]
        } else {
            panic!("NbtTag is not a Compound")
        }
    }
}
impl core::ops::IndexMut<usize> for NbtTag {
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
#[derive(Debug, Clone, IsVariant, Unwrap, TryUnwrap)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(untagged))]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(no_field_bounds, Debug, PartialEq))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub enum NbtListTag {
    /// An empty list.
    Empty = NbtTag::END,
    /// A list of signed 8-bit integers.
    Byte(ByteArray) = NbtTag::BYTE,
    /// A list of signed 16-bit integers.
    Short(ShortArray) = NbtTag::SHORT,
    /// A list of signed 32-bit integers.
    Int(IntArray) = NbtTag::INT,
    /// A list of signed 64-bit integers.
    Long(LongArray) = NbtTag::LONG,
    /// A list of 32-bit floating point numbers.
    Float(FloatArray) = NbtTag::FLOAT,
    /// A list of 64-bit floating point numbers.
    Double(DoubleArray) = NbtTag::DOUBLE,
    /// A list of byte arrays.
    ByteArray(Vec<ByteArray>) = NbtTag::BYTE_ARRAY,
    /// A list of MUTF-8 strings.
    String(Vec<Mutf8String>) = NbtTag::STRING,
    /// A list of [`NbtListTag`]s.
    List(Vec<NbtListTag>) = NbtTag::LIST,
    /// A list of [`NbtCompound`]s.
    Compound(Vec<NbtCompound>) = NbtTag::COMPOUND,
    /// A list of signed 32-bit integers.
    IntArray(Vec<IntArray>) = NbtTag::INT_ARRAY,
    /// A list of signed 64-bit integers.
    LongArray(Vec<LongArray>) = NbtTag::LONG_ARRAY,
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
    pub fn as_byte(&self) -> Option<&ByteArray> {
        if let NbtListTag::Byte(byte) = self { Some(byte) } else { None }
    }

    /// Get the value of a [`NbtListTag::Byte`] variant mutably, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_byte_mut(&mut self) -> Option<&mut ByteArray> {
        if let NbtListTag::Byte(byte) = self { Some(byte) } else { None }
    }

    /// Get the value of a [`NbtListTag::Short`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_short(&self) -> Option<&ShortArray> {
        if let NbtListTag::Short(short) = self { Some(short) } else { None }
    }

    /// Get the value of a [`NbtListTag::Short`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_short_mut(&mut self) -> Option<&mut ShortArray> {
        if let NbtListTag::Short(short) = self { Some(short) } else { None }
    }

    /// Get the value of a [`NbtListTag::Int`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_int(&self) -> Option<&IntArray> {
        if let NbtListTag::Int(int) = self { Some(int) } else { None }
    }

    /// Get the value of a [`NbtListTag::Int`] variant mutably, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_int_mut(&mut self) -> Option<&mut IntArray> {
        if let NbtListTag::Int(int) = self { Some(int) } else { None }
    }

    /// Get the value of a [`NbtListTag::Long`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_long(&self) -> Option<&LongArray> {
        if let NbtListTag::Long(long) = self { Some(long) } else { None }
    }

    /// Get the value of a [`NbtListTag::Long`] variant mutably, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_long_mut(&mut self) -> Option<&mut LongArray> {
        if let NbtListTag::Long(long) = self { Some(long) } else { None }
    }

    /// Get the value of a [`NbtListTag::Float`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_float(&self) -> Option<&FloatArray> {
        if let NbtListTag::Float(float) = self { Some(float) } else { None }
    }

    /// Get the value of a [`NbtListTag::Float`] variant mutably, if it is one.
    ///
    ///    Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_float_mut(&mut self) -> Option<&mut FloatArray> {
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
    pub fn as_double_mut(&mut self) -> Option<&mut DoubleArray> {
        if let NbtListTag::Double(double) = self { Some(double) } else { None }
    }

    /// Get the value of a [`NbtListTag::ByteArray`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_byte_array(&self) -> Option<&[ByteArray]> {
        if let NbtListTag::ByteArray(array) = self { Some(array) } else { None }
    }

    /// Get the value of a [`NbtListTag::ByteArray`] variant mutably, if it is
    /// one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_byte_array_mut(&mut self) -> Option<&mut Vec<ByteArray>> {
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
    pub fn as_int_array(&self) -> Option<&[IntArray]> {
        if let NbtListTag::IntArray(array) = self { Some(array) } else { None }
    }

    /// Get the value of a [`NbtListTag::IntArray`] variant mutably, if it is
    /// one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_int_array_mut(&mut self) -> Option<&mut Vec<IntArray>> {
        if let NbtListTag::IntArray(array) = self { Some(array) } else { None }
    }

    /// Get the value of a [`NbtListTag::LongArray`] variant, if it is one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_long_array(&self) -> Option<&[LongArray]> {
        if let NbtListTag::LongArray(array) = self { Some(array) } else { None }
    }

    /// Get the value of a [`NbtListTag::LongArray`] variant mutably, if it is
    /// one.
    ///
    /// Returns `None` if the value is of any other type.
    #[must_use]
    pub fn as_long_array_mut(&mut self) -> Option<&mut Vec<LongArray>> {
        if let NbtListTag::LongArray(array) = self { Some(array) } else { None }
    }
}

impl From<Vec<i8>> for NbtListTag {
    fn from(value: Vec<i8>) -> Self { NbtListTag::Byte(ByteArray::from(value)) }
}
impl From<Vec<u8>> for NbtListTag {
    fn from(value: Vec<u8>) -> Self { NbtListTag::Byte(ByteArray::from(value)) }
}
impl From<Vec<i16>> for NbtListTag {
    fn from(value: Vec<i16>) -> Self { NbtListTag::Short(ShortArray::from(value)) }
}
impl From<Vec<u16>> for NbtListTag {
    fn from(value: Vec<u16>) -> Self { NbtListTag::Short(ShortArray::from(value)) }
}
impl From<Vec<i32>> for NbtListTag {
    fn from(value: Vec<i32>) -> Self { NbtListTag::Int(IntArray::from(value)) }
}
impl From<Vec<u32>> for NbtListTag {
    fn from(value: Vec<u32>) -> Self { NbtListTag::Int(IntArray::from(value)) }
}
impl From<Vec<i64>> for NbtListTag {
    fn from(value: Vec<i64>) -> Self { NbtListTag::Long(LongArray::from(value)) }
}
impl From<Vec<u64>> for NbtListTag {
    fn from(value: Vec<u64>) -> Self { NbtListTag::Long(LongArray::from(value)) }
}
impl From<Vec<f32>> for NbtListTag {
    fn from(value: Vec<f32>) -> Self { NbtListTag::Float(FloatArray::from(value)) }
}
impl From<Vec<f64>> for NbtListTag {
    fn from(value: Vec<f64>) -> Self { NbtListTag::Double(DoubleArray::from(value)) }
}

impl From<Vec<String>> for NbtListTag {
    fn from(value: Vec<String>) -> Self {
        NbtListTag::String(value.into_iter().map(Mutf8String::from).collect())
    }
}
impl<'a> From<Vec<&'a str>> for NbtListTag {
    fn from(value: Vec<&'a str>) -> Self {
        NbtListTag::String(value.into_iter().map(Mutf8String::from).collect())
    }
}

impl From<Vec<Vec<i8>>> for NbtListTag {
    fn from(value: Vec<Vec<i8>>) -> Self {
        NbtListTag::ByteArray(value.into_iter().map(ByteArray::from).collect())
    }
}
impl From<Vec<Vec<u8>>> for NbtListTag {
    fn from(value: Vec<Vec<u8>>) -> Self {
        NbtListTag::ByteArray(value.into_iter().map(ByteArray::from).collect())
    }
}
impl From<Vec<Vec<i32>>> for NbtListTag {
    fn from(value: Vec<Vec<i32>>) -> Self {
        NbtListTag::IntArray(value.into_iter().map(IntArray::from).collect())
    }
}
impl From<Vec<Vec<u32>>> for NbtListTag {
    fn from(value: Vec<Vec<u32>>) -> Self {
        NbtListTag::IntArray(value.into_iter().map(IntArray::from).collect())
    }
}
impl From<Vec<Vec<i64>>> for NbtListTag {
    fn from(value: Vec<Vec<i64>>) -> Self {
        NbtListTag::LongArray(value.into_iter().map(LongArray::from).collect())
    }
}
impl From<Vec<Vec<u64>>> for NbtListTag {
    fn from(value: Vec<Vec<u64>>) -> Self {
        NbtListTag::LongArray(value.into_iter().map(LongArray::from).collect())
    }
}

impl PartialEq for NbtListTag {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NbtListTag::Empty, NbtListTag::Empty) => true,
            (NbtListTag::Byte(f0_self), NbtListTag::Byte(f0_other)) => f0_self.eq(f0_other),
            (NbtListTag::Short(f0_self), NbtListTag::Short(f0_other)) => f0_self.eq(f0_other),
            (NbtListTag::Int(f0_self), NbtListTag::Int(f0_other)) => f0_self.eq(f0_other),
            (NbtListTag::Long(f0_self), NbtListTag::Long(f0_other)) => f0_self.eq(f0_other),
            (NbtListTag::Float(f0_self), NbtListTag::Float(f0_other)) => f0_self.eq(f0_other),
            (NbtListTag::Double(f0_self), NbtListTag::Double(f0_other)) => f0_self.eq(f0_other),
            (NbtListTag::ByteArray(f0_self), NbtListTag::ByteArray(f0_other)) => {
                f0_self.eq(f0_other)
            }
            (NbtListTag::String(f0_self), NbtListTag::String(f0_other)) => f0_self.eq(f0_other),
            (NbtListTag::List(f0_self), NbtListTag::List(f0_other)) => f0_self.eq(f0_other),
            (NbtListTag::Compound(f0_self), NbtListTag::Compound(f0_other)) => f0_self.eq(f0_other),
            (NbtListTag::IntArray(f0_self), NbtListTag::IntArray(f0_other)) => f0_self.eq(f0_other),
            (NbtListTag::LongArray(f0_self), NbtListTag::LongArray(f0_other)) => {
                f0_self.eq(f0_other)
            }

            // Make `Empty` equal to everything *only if* the other list is empty.
            (NbtListTag::Empty, NbtListTag::Byte(list))
            | (NbtListTag::Byte(list), NbtListTag::Empty) => list.is_empty(),
            (NbtListTag::Empty, NbtListTag::Short(list))
            | (NbtListTag::Short(list), NbtListTag::Empty) => list.is_empty(),
            (NbtListTag::Empty, NbtListTag::Int(list))
            | (NbtListTag::Int(list), NbtListTag::Empty) => list.is_empty(),
            (NbtListTag::Empty, NbtListTag::Long(list))
            | (NbtListTag::Long(list), NbtListTag::Empty) => list.is_empty(),
            (NbtListTag::Empty, NbtListTag::Float(list))
            | (NbtListTag::Float(list), NbtListTag::Empty) => list.is_empty(),
            (NbtListTag::Empty, NbtListTag::Double(list))
            | (NbtListTag::Double(list), NbtListTag::Empty) => list.is_empty(),
            (NbtListTag::Empty, NbtListTag::ByteArray(list))
            | (NbtListTag::ByteArray(list), NbtListTag::Empty) => list.is_empty(),
            (NbtListTag::Empty, NbtListTag::String(list))
            | (NbtListTag::String(list), NbtListTag::Empty) => list.is_empty(),
            (NbtListTag::Empty, NbtListTag::List(list))
            | (NbtListTag::List(list), NbtListTag::Empty) => list.is_empty(),
            (NbtListTag::Empty, NbtListTag::Compound(list))
            | (NbtListTag::Compound(list), NbtListTag::Empty) => list.is_empty(),
            (NbtListTag::Empty, NbtListTag::IntArray(list))
            | (NbtListTag::IntArray(list), NbtListTag::Empty) => list.is_empty(),
            (NbtListTag::Empty, NbtListTag::LongArray(list))
            | (NbtListTag::LongArray(list), NbtListTag::Empty) => list.is_empty(),

            _unused => false,
        }
    }
}
