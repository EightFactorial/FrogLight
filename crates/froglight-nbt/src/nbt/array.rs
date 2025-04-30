#![allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};

/// A wrapper around a [`Vec<i8>`] that represents a NBT byte array.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, From, Into, AsRef, AsMut, Deref, DerefMut)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(all(feature = "reflect", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct ByteArray(Vec<i8>);

impl PartialEq<[i8]> for ByteArray {
    #[inline]
    fn eq(&self, other: &[i8]) -> bool { self.0.as_slice() == other }
}

impl From<Vec<u8>> for ByteArray {
    fn from(value: Vec<u8>) -> Self { Self(value.into_iter().map(|v| v as i8).collect()) }
}
impl From<ByteArray> for Vec<u8> {
    fn from(value: ByteArray) -> Self { value.0.into_iter().map(|v| v as u8).collect() }
}

/// A wrapper around a [`Vec<i16>`] that represents a NBT short array.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, From, Into, AsRef, AsMut, Deref, DerefMut)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(all(feature = "reflect", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct ShortArray(Vec<i16>);

impl PartialEq<[i16]> for ShortArray {
    #[inline]
    fn eq(&self, other: &[i16]) -> bool { self.0.as_slice() == other }
}

impl From<Vec<u16>> for ShortArray {
    fn from(value: Vec<u16>) -> Self { Self(value.into_iter().map(|v| v as i16).collect()) }
}
impl From<ShortArray> for Vec<u16> {
    fn from(value: ShortArray) -> Self { value.0.into_iter().map(|v| v as u16).collect() }
}

/// A wrapper around a [`Vec<i32>`] that represents a NBT int array.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, From, Into, AsRef, AsMut, Deref, DerefMut)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(all(feature = "reflect", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct IntArray(Vec<i32>);

impl PartialEq<[i32]> for IntArray {
    #[inline]
    fn eq(&self, other: &[i32]) -> bool { self.0.as_slice() == other }
}

impl From<Vec<u32>> for IntArray {
    fn from(value: Vec<u32>) -> Self { Self(value.into_iter().map(|v| v as i32).collect()) }
}
impl From<IntArray> for Vec<u32> {
    fn from(value: IntArray) -> Self { value.0.into_iter().map(|v| v as u32).collect() }
}

/// A wrapper around a [`Vec<i64>`] that represents a NBT long array.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, From, Into, AsRef, AsMut, Deref, DerefMut)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(all(feature = "reflect", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct LongArray(Vec<i64>);

impl PartialEq<[i64]> for LongArray {
    #[inline]
    fn eq(&self, other: &[i64]) -> bool { self.0.as_slice() == other }
}

impl From<Vec<u64>> for LongArray {
    fn from(value: Vec<u64>) -> Self { Self(value.into_iter().map(|v| v as i64).collect()) }
}
impl From<LongArray> for Vec<u64> {
    fn from(value: LongArray) -> Self { value.0.into_iter().map(|v| v as u64).collect() }
}

/// A wrapper around a [`Vec<f32>`] that represents a NBT float array.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, From, Into, AsRef, AsMut, Deref, DerefMut)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(all(feature = "reflect", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct FloatArray(Vec<f32>);

/// A wrapper around a [`Vec<f64>`] that represents a NBT double array.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, From, Into, AsRef, AsMut, Deref, DerefMut)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(all(feature = "reflect", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct DoubleArray(Vec<f64>);
