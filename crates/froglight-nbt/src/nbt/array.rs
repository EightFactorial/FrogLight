#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};

/// A wrapper around a [`Vec<i8>`] that represents a NBT byte array.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, From, Into, AsRef, AsMut, Deref, DerefMut)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct ByteArray(Vec<i8>);

impl PartialEq<[i8]> for ByteArray {
    #[inline]
    fn eq(&self, other: &[i8]) -> bool { self.0.as_slice() == other }
}

/// A wrapper around a [`Vec<i16>`] that represents a NBT short array.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, From, Into, AsRef, AsMut, Deref, DerefMut)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct ShortArray(Vec<i16>);

impl PartialEq<[i16]> for ShortArray {
    #[inline]
    fn eq(&self, other: &[i16]) -> bool { self.0.as_slice() == other }
}

/// A wrapper around a [`Vec<i32>`] that represents a NBT int array.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, From, Into, AsRef, AsMut, Deref, DerefMut)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct IntArray(Vec<i32>);

impl PartialEq<[i32]> for IntArray {
    #[inline]
    fn eq(&self, other: &[i32]) -> bool { self.0.as_slice() == other }
}

/// A wrapper around a [`Vec<i64>`] that represents a NBT long array.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, From, Into, AsRef, AsMut, Deref, DerefMut)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct LongArray(Vec<i64>);

impl PartialEq<[i64]> for LongArray {
    #[inline]
    fn eq(&self, other: &[i64]) -> bool { self.0.as_slice() == other }
}

/// A wrapper around a [`Vec<f32>`] that represents a NBT float array.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, From, Into, AsRef, AsMut, Deref, DerefMut)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct FloatArray(Vec<f32>);

impl PartialEq<[f32]> for FloatArray {
    #[inline]
    fn eq(&self, other: &[f32]) -> bool { self.0.as_slice() == other }
}

/// A wrapper around a [`Vec<f64>`] that represents a NBT double array.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, From, Into, AsRef, AsMut, Deref, DerefMut)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct DoubleArray(Vec<f64>);

impl PartialEq<[f64]> for DoubleArray {
    #[inline]
    fn eq(&self, other: &[f64]) -> bool { self.0.as_slice() == other }
}
