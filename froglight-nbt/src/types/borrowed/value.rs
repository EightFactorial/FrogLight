//! TODO

use core::{fmt, num::NonZeroUsize};

use froglight_mutf8::prelude::MStr;

use crate::{
    prelude::{IndexedCompoundMut, IndexedCompoundRef},
    types::borrowed::{
        IndexedCore,
        list::{IndexedListMut, IndexedListRef},
        reference::{BorrowedIndex, BorrowedMut, BorrowedRef},
    },
};

/// A reference to an [`IndexedValue`].
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct BorrowedValueRef<'data> {
    root: &'data [u8],
    core: &'data IndexedCore,
    value: BorrowedValueIndex,
    index: Option<NonZeroUsize>,
}

impl<'data> BorrowedValueRef<'data> {
    /// Create a new [`BorrowedValueRef`] using the given data and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - The `root` slice is valid for reads
    /// - `root` contains the value in `value`
    /// - `index` is either zero or a valid index into `core`
    #[inline]
    #[must_use]
    pub(super) const unsafe fn new(
        root: &'data [u8],
        core: &'data IndexedCore,
        value: BorrowedValueIndex,
        index: usize,
    ) -> Self {
        Self { root, core, value, index: NonZeroUsize::new(index) }
    }

    /// Create a new [`BorrowedValueRef`] using the given data and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - The `root` slice is valid for reads
    /// - `root` contains the value in `value`
    /// - `index` is a valid index into `core`
    #[inline]
    #[must_use]
    pub(super) const unsafe fn new_nonzero(
        root: &'data [u8],
        core: &'data IndexedCore,
        value: BorrowedValueIndex,
        index: Option<NonZeroUsize>,
    ) -> Self {
        Self { root, core, value, index }
    }

    /// Get the [`IndexedValueType`] of this value.
    #[inline]
    #[must_use]
    pub const fn ty(&self) -> BorrowedValueType { self.value.ty() }

    /// Get the value as a [`BorrowedValue`].
    #[must_use]
    pub fn as_value(&self) -> BorrowedValue<'data> {
        match self.value {
            BorrowedValueIndex::Byte(..) => {
                BorrowedValue::Byte(unsafe { self.as_byte().unwrap_unchecked() })
            }
            BorrowedValueIndex::Short(..) => {
                BorrowedValue::Short(unsafe { self.as_short().unwrap_unchecked() })
            }
            BorrowedValueIndex::Int(..) => {
                BorrowedValue::Int(unsafe { self.as_int().unwrap_unchecked() })
            }
            BorrowedValueIndex::Long(..) => {
                BorrowedValue::Long(unsafe { self.as_long().unwrap_unchecked() })
            }
            BorrowedValueIndex::Float(..) => {
                BorrowedValue::Float(unsafe { self.as_float().unwrap_unchecked() })
            }
            BorrowedValueIndex::Double(..) => {
                BorrowedValue::Double(unsafe { self.as_double().unwrap_unchecked() })
            }
            BorrowedValueIndex::ByteArray(..) => {
                BorrowedValue::ByteArray(unsafe { self.as_byte_array().unwrap_unchecked() })
            }
            BorrowedValueIndex::String(..) => {
                BorrowedValue::String(unsafe { self.as_string().unwrap_unchecked() })
            }
            BorrowedValueIndex::List(..) => {
                BorrowedValue::List(unsafe { self.as_list().unwrap_unchecked() })
            }
            BorrowedValueIndex::Compound(..) => {
                BorrowedValue::Compound(unsafe { self.as_compound().unwrap_unchecked() })
            }
            BorrowedValueIndex::IntArray(..) => {
                BorrowedValue::IntArray(unsafe { self.as_int_array().unwrap_unchecked() })
            }
            BorrowedValueIndex::LongArray(..) => {
                BorrowedValue::LongArray(unsafe { self.as_long_array().unwrap_unchecked() })
            }
        }
    }

    /// Get the value as a [`u8`].
    ///
    /// Returns `None` if the value is not a [`u8`].
    #[must_use]
    pub fn as_byte(&self) -> Option<u8> {
        match self.value {
            BorrowedValueIndex::Byte(index) => {
                Some(unsafe { BorrowedRef::new(self.root, index) }.get_value())
            }
            _ => None,
        }
    }

    /// Get the value as a [`u16`].
    ///
    /// Returns `None` if the value is not a [`u16`].
    #[must_use]
    pub fn as_short(&self) -> Option<u16> {
        match self.value {
            BorrowedValueIndex::Short(index) => {
                Some(unsafe { BorrowedRef::new(self.root, index) }.get_value())
            }
            _ => None,
        }
    }

    /// Get the value as a [`u32`].
    ///
    /// Returns `None` if the value is not a [`u32`].
    #[must_use]
    pub fn as_int(&self) -> Option<u32> {
        match self.value {
            BorrowedValueIndex::Int(index) => {
                Some(unsafe { BorrowedRef::new(self.root, index) }.get_value())
            }
            _ => None,
        }
    }

    /// Get the value as a [`u64`].
    ///
    /// Returns `None` if the value is not a [`u64`].
    #[must_use]
    pub fn as_long(&self) -> Option<u64> {
        match self.value {
            BorrowedValueIndex::Long(index) => {
                Some(unsafe { BorrowedRef::new(self.root, index) }.get_value())
            }
            _ => None,
        }
    }

    /// Get the value as a [`f32`].
    ///
    /// Returns `None` if the value is not a [`f32`].
    #[must_use]
    pub fn as_float(&self) -> Option<f32> {
        match self.value {
            BorrowedValueIndex::Float(index) => {
                Some(unsafe { BorrowedRef::new(self.root, index) }.get_value())
            }
            _ => None,
        }
    }

    /// Get the value as a [`f64`].
    ///
    /// Returns `None` if the value is not a [`f64`].
    #[must_use]
    pub fn as_double(&self) -> Option<f64> {
        match self.value {
            BorrowedValueIndex::Double(index) => {
                Some(unsafe { BorrowedRef::new(self.root, index) }.get_value())
            }
            _ => None,
        }
    }

    /// Get the value as a byte array.
    ///
    /// Returns `None` if the value is not a byte array.
    #[must_use]
    pub fn as_byte_array(&self) -> Option<&'data [u8]> {
        match self.value {
            BorrowedValueIndex::ByteArray(index) => {
                Some(unsafe { BorrowedRef::new(self.root, index) }.get_ref())
            }
            _ => None,
        }
    }

    /// Get the value as a string.
    ///
    /// Returns `None` if the value is not a string.
    #[must_use]
    pub fn as_string(&self) -> Option<&'data MStr> {
        match self.value {
            BorrowedValueIndex::String(index) => {
                Some(unsafe { BorrowedRef::new(self.root, index) }.get_ref())
            }
            _ => None,
        }
    }

    /// Get the list of values.
    ///
    /// Returns `None` if the value is not a list.
    #[must_use]
    pub fn as_list(&self) -> Option<IndexedListRef<'data, IndexedList>> {
        let BorrowedValueIndex::List(_) = self.value else { return None };
        let Some(index) = self.index else {
            #[cfg(debug_assertions)]
            unreachable!("IndexedValue::List must have an index!");
            #[cfg(not(debug_assertions))]
            unsafe {
                core::hint::unreachable_unchecked()
            }
        };

        // SAFETY: The provided `index` is required to be valid for `core`
        Some(unsafe { IndexedListRef::new(self.root, self.core, index.get()) })
    }

    /// Get the compound of named entries.
    ///
    /// Returns `None` if the value is not a compound.
    #[must_use]
    pub fn as_compound(&self) -> Option<IndexedCompoundRef<'data>> {
        let BorrowedValueIndex::Compound(_) = self.value else { return None };
        let Some(index) = self.index else {
            #[cfg(debug_assertions)]
            unreachable!("IndexedValue::Compound must have an index!");
            #[cfg(not(debug_assertions))]
            unsafe {
                core::hint::unreachable_unchecked()
            }
        };

        // SAFETY: The provided `index` is required to be valid for `core`
        Some(unsafe { IndexedCompoundRef::new(self.root, self.core, index.get()) })
    }

    /// Get the value as a [`u32`] array.
    ///
    /// Returns `None` if the value is not a [`u32`] array.
    #[must_use]
    pub fn as_int_array(&self) -> Option<IndexedListRef<'data, u32>> {
        let BorrowedValueIndex::IntArray(_) = self.value else { return None };
        let Some(index) = self.index else {
            #[cfg(debug_assertions)]
            unreachable!("IndexedValue::IntArray must have an index!");
            #[cfg(not(debug_assertions))]
            unsafe {
                core::hint::unreachable_unchecked()
            }
        };

        // SAFETY: The provided `index` is required to be valid for `core`
        Some(unsafe { IndexedListRef::new(self.root, self.core, index.get()) })
    }

    /// Get the value as a [`u64`] array.
    ///
    /// Returns `None` if the value is not a [`u64`] array.
    #[must_use]
    pub fn as_long_array(&self) -> Option<IndexedListRef<'data, u64>> {
        let BorrowedValueIndex::LongArray(_) = self.value else { return None };
        let Some(index) = self.index else {
            #[cfg(debug_assertions)]
            unreachable!("IndexedValue::LongArray must have an index!");
            #[cfg(not(debug_assertions))]
            unsafe {
                core::hint::unreachable_unchecked()
            }
        };

        // SAFETY: The provided `index` is required to be valid for `core`
        Some(unsafe { IndexedListRef::new(self.root, self.core, index.get()) })
    }
}

impl fmt::Debug for BorrowedValueRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.as_value(), f)
    }
}

// -------------------------------------------------------------------------------------------------

/// A mutable reference to an [`IndexedValue`].
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct BorrowedValueMut<'data> {
    root: &'data mut [u8],
    core: &'data IndexedCore,
    value: BorrowedValueIndex,
    index: Option<NonZeroUsize>,
}

impl<'data> BorrowedValueMut<'data> {
    /// Create a new [`BorrowedValueMut`] using the given data and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `root` is valid for reads,
    /// and that it contains a valid value at the position specified by `index`.
    #[inline]
    #[must_use]
    pub(super) const unsafe fn new(
        root: &'data mut [u8],
        core: &'data IndexedCore,
        value: BorrowedValueIndex,
        index: usize,
    ) -> Self {
        Self { root, core, value, index: NonZeroUsize::new(index) }
    }

    /// Create a new [`BorrowedValueMut`] using the given data and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `root` is valid for reads,
    /// and that it contains a valid value at the position specified by `index`.
    #[inline]
    #[must_use]
    pub(super) const unsafe fn new_nonzero(
        root: &'data mut [u8],
        core: &'data IndexedCore,
        value: BorrowedValueIndex,
        index: Option<NonZeroUsize>,
    ) -> Self {
        Self { root, core, value, index }
    }

    /// Get the [`IndexedValueType`] of this value.
    #[inline]
    #[must_use]
    pub const fn ty(&self) -> BorrowedValueType { self.value.ty() }

    /// Get the value as a [`BorrowedValue`].
    #[must_use]
    pub fn as_value(&self) -> BorrowedValue<'_> {
        match self.value {
            BorrowedValueIndex::Byte(..) => {
                BorrowedValue::Byte(unsafe { self.as_byte().unwrap_unchecked() })
            }
            BorrowedValueIndex::Short(..) => {
                BorrowedValue::Short(unsafe { self.as_short().unwrap_unchecked() })
            }
            BorrowedValueIndex::Int(..) => {
                BorrowedValue::Int(unsafe { self.as_int().unwrap_unchecked() })
            }
            BorrowedValueIndex::Long(..) => {
                BorrowedValue::Long(unsafe { self.as_long().unwrap_unchecked() })
            }
            BorrowedValueIndex::Float(..) => {
                BorrowedValue::Float(unsafe { self.as_float().unwrap_unchecked() })
            }
            BorrowedValueIndex::Double(..) => {
                BorrowedValue::Double(unsafe { self.as_double().unwrap_unchecked() })
            }
            BorrowedValueIndex::ByteArray(..) => {
                BorrowedValue::ByteArray(unsafe { self.as_byte_array().unwrap_unchecked() })
            }
            BorrowedValueIndex::String(..) => {
                BorrowedValue::String(unsafe { self.as_string().unwrap_unchecked() })
            }
            BorrowedValueIndex::List(..) => {
                BorrowedValue::List(unsafe { self.as_list().unwrap_unchecked() })
            }
            BorrowedValueIndex::Compound(..) => {
                BorrowedValue::Compound(unsafe { self.as_compound().unwrap_unchecked() })
            }
            BorrowedValueIndex::IntArray(..) => {
                BorrowedValue::IntArray(unsafe { self.as_int_array().unwrap_unchecked() })
            }
            BorrowedValueIndex::LongArray(..) => {
                BorrowedValue::LongArray(unsafe { self.as_long_array().unwrap_unchecked() })
            }
        }
    }

    /// Get the value as a [`u8`].
    ///
    /// Returns `None` if the value is not a [`u8`].
    #[must_use]
    pub fn as_byte(&self) -> Option<u8> {
        match self.value {
            BorrowedValueIndex::Byte(index) => {
                Some(unsafe { BorrowedRef::new(self.root, index) }.get_value())
            }
            _ => None,
        }
    }

    /// Set the value as a [`u8`].
    ///
    /// Returns `true` if the value was set,
    /// or `false` if the value is not a [`u8`].
    pub fn set_byte(&mut self, value: u8) -> bool {
        match self.value {
            BorrowedValueIndex::Byte(index) => {
                unsafe { BorrowedMut::new(self.root, index) }.set_value(value);
                true
            }
            _ => false,
        }
    }

    /// Get the value as a [`u16`].
    ///
    /// Returns `None` if the value is not a [`u16`].
    #[must_use]
    pub fn as_short(&self) -> Option<u16> {
        match self.value {
            BorrowedValueIndex::Short(index) => {
                Some(unsafe { BorrowedRef::new(self.root, index) }.get_value())
            }
            _ => None,
        }
    }

    /// Set the value as a [`u16`].
    ///
    /// Returns `true` if the value was set,
    /// or `false` if the value is not a [`u16`].
    pub fn set_short(&mut self, value: u16) -> bool {
        match self.value {
            BorrowedValueIndex::Short(index) => {
                unsafe { BorrowedMut::new(self.root, index) }.set_value(value);
                true
            }
            _ => false,
        }
    }

    /// Get the value as a [`u32`].
    ///
    /// Returns `None` if the value is not a [`u32`].
    #[must_use]
    pub fn as_int(&self) -> Option<u32> {
        match self.value {
            BorrowedValueIndex::Int(index) => {
                Some(unsafe { BorrowedRef::new(self.root, index) }.get_value())
            }
            _ => None,
        }
    }

    /// Set the value as a [`u32`].
    ///
    /// Returns `true` if the value was set,
    /// or `false` if the value is not a [`u32`].
    pub fn set_int(&mut self, value: u32) -> bool {
        match self.value {
            BorrowedValueIndex::Int(index) => {
                unsafe { BorrowedMut::new(self.root, index) }.set_value(value);
                true
            }
            _ => false,
        }
    }

    /// Get the value as a [`u64`].
    ///
    /// Returns `None` if the value is not a [`u64`].
    #[must_use]
    pub fn as_long(&self) -> Option<u64> {
        match self.value {
            BorrowedValueIndex::Long(index) => {
                Some(unsafe { BorrowedRef::new(self.root, index) }.get_value())
            }
            _ => None,
        }
    }

    /// Set the value as a [`u64`].
    ///
    /// Returns `true` if the value was set,
    /// or `false` if the value is not a [`u64`].
    pub fn set_long(&mut self, value: u64) -> bool {
        match self.value {
            BorrowedValueIndex::Long(index) => {
                unsafe { BorrowedMut::new(self.root, index) }.set_value(value);
                true
            }
            _ => false,
        }
    }

    /// Get the value as a [`f32`].
    ///
    /// Returns `None` if the value is not a [`f32`].
    #[must_use]
    pub fn as_float(&self) -> Option<f32> {
        match self.value {
            BorrowedValueIndex::Float(index) => {
                Some(unsafe { BorrowedRef::new(self.root, index) }.get_value())
            }
            _ => None,
        }
    }

    /// Set the value as a [`f32`].
    ///
    /// Returns `true` if the value was set,
    /// or `false` if the value is not a [`f32`].
    pub fn set_float(&mut self, value: f32) -> bool {
        match self.value {
            BorrowedValueIndex::Float(index) => {
                unsafe { BorrowedMut::new(self.root, index) }.set_value(value);
                true
            }
            _ => false,
        }
    }

    /// Get the value as a [`f64`].
    ///
    /// Returns `None` if the value is not a [`f64`].
    #[must_use]
    pub fn as_double(&self) -> Option<f64> {
        match self.value {
            BorrowedValueIndex::Double(index) => {
                Some(unsafe { BorrowedRef::new(self.root, index) }.get_value())
            }
            _ => None,
        }
    }

    /// Set the value as a [`f64`].
    ///
    /// Returns `true` if the value was set,
    /// or `false` if the value is not a [`f64`].
    pub fn set_double(&mut self, value: f64) -> bool {
        match self.value {
            BorrowedValueIndex::Double(index) => {
                unsafe { BorrowedMut::new(self.root, index) }.set_value(value);
                true
            }
            _ => false,
        }
    }

    /// Get the value as a byte array.
    ///
    /// Returns `None` if the value is not a byte array.
    #[must_use]
    pub fn as_byte_array(&self) -> Option<&[u8]> {
        match self.value {
            BorrowedValueIndex::ByteArray(index) => {
                Some(unsafe { BorrowedRef::new(self.root, index) }.get_ref())
            }
            _ => None,
        }
    }

    /// Get the value as a string.
    ///
    /// Returns `None` if the value is not a string.
    #[must_use]
    pub fn as_string(&self) -> Option<&MStr> {
        match self.value {
            BorrowedValueIndex::String(index) => {
                Some(unsafe { BorrowedRef::new(self.root, index) }.get_ref())
            }
            _ => None,
        }
    }

    /// Get the list of values.
    ///
    /// Returns `None` if the value is not a list.
    #[must_use]
    pub fn as_list(&self) -> Option<IndexedListRef<'_, IndexedList>> {
        let BorrowedValueIndex::List(_) = self.value else { return None };
        let Some(index) = self.index else {
            #[cfg(debug_assertions)]
            unreachable!("IndexedValue::List must have an index!");
            #[cfg(not(debug_assertions))]
            unsafe {
                core::hint::unreachable_unchecked()
            }
        };

        // SAFETY: The provided `index` is required to be valid for `core`
        Some(unsafe { IndexedListRef::new(self.root, self.core, index.get()) })
    }

    /// Get the list of values mutably.
    ///
    /// Returns `None` if the value is not a list.
    #[must_use]
    pub fn as_list_mut(&mut self) -> Option<IndexedListMut<'_, IndexedList>> {
        let BorrowedValueIndex::List(_) = self.value else { return None };
        let Some(index) = self.index else {
            #[cfg(debug_assertions)]
            unreachable!("IndexedValue::List must have an index!");
            #[cfg(not(debug_assertions))]
            unsafe {
                core::hint::unreachable_unchecked()
            }
        };

        // SAFETY: The provided `index` is required to be valid for `core`
        Some(unsafe { IndexedListMut::new(self.root, self.core, index.get()) })
    }

    /// Get the compound of named entries.
    ///
    /// Returns `None` if the value is not a compound.
    #[must_use]
    pub fn as_compound(&self) -> Option<IndexedCompoundRef<'_>> {
        let BorrowedValueIndex::Compound(_) = self.value else { return None };
        let Some(index) = self.index else {
            #[cfg(debug_assertions)]
            unreachable!("IndexedValue::Compound must have an index!");
            #[cfg(not(debug_assertions))]
            unsafe {
                core::hint::unreachable_unchecked()
            }
        };

        // SAFETY: The provided `index` is required to be valid for `core`
        Some(unsafe { IndexedCompoundRef::new(self.root, self.core, index.get()) })
    }

    /// Get the compound of named entries mutably.
    ///
    /// Returns `None` if the value is not a compound.
    #[must_use]
    pub fn as_compound_mut(&mut self) -> Option<IndexedCompoundMut<'_>> {
        let BorrowedValueIndex::Compound(_) = self.value else { return None };
        let Some(index) = self.index else {
            #[cfg(debug_assertions)]
            unreachable!("IndexedValue::Compound must have an index!");
            #[cfg(not(debug_assertions))]
            unsafe {
                core::hint::unreachable_unchecked()
            }
        };

        // SAFETY: The provided `index` is required to be valid for `core`
        Some(unsafe { IndexedCompoundMut::new(self.root, self.core, index.get()) })
    }

    /// Get the value as a [`u32`] array.
    ///
    /// Returns `None` if the value is not a [`u32`] array.
    #[must_use]
    pub fn as_int_array(&self) -> Option<IndexedListRef<'data, u32>> {
        match self.value {
            BorrowedValueIndex::IntArray(_index) => todo!(),
            _ => None,
        }
    }

    /// Get the value as a [`u32`] array mutably.
    ///
    /// Returns `None` if the value is not a [`u32`] array.
    #[must_use]
    pub fn as_int_array_mut(&mut self) -> Option<IndexedListMut<'data, u32>> {
        match self.value {
            BorrowedValueIndex::IntArray(_index) => todo!(),
            _ => None,
        }
    }

    /// Get the value as a [`u64`] array.
    ///
    /// Returns `None` if the value is not a [`u64`] array.
    #[must_use]
    pub fn as_long_array(&self) -> Option<IndexedListRef<'data, u64>> {
        match self.value {
            BorrowedValueIndex::LongArray(_index) => todo!(),
            _ => None,
        }
    }

    /// Get the value as a [`u64`] array mutably.
    ///
    /// Returns `None` if the value is not a [`u64`] array.
    #[must_use]
    pub fn as_long_array_mut(&mut self) -> Option<IndexedListMut<'data, u64>> {
        match self.value {
            BorrowedValueIndex::LongArray(_index) => todo!(),
            _ => None,
        }
    }
}

impl fmt::Debug for BorrowedValueMut<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.as_value(), f)
    }
}

// -------------------------------------------------------------------------------------------------

/// An indexed value of any type.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum BorrowedValueIndex {
    /// A [`u8`] value.
    Byte(BorrowedIndex<u8>),
    /// A [`u16`] value.
    Short(BorrowedIndex<u16>),
    /// A [`u32`] value.
    Int(BorrowedIndex<u32>),
    /// A [`u64`] value.
    Long(BorrowedIndex<u64>),
    /// A [`f32`] value.
    Float(BorrowedIndex<f32>),
    /// A [`f64`] value.
    Double(BorrowedIndex<f64>),
    /// A [`u8`] array.
    ByteArray(BorrowedIndex<[u8]>),
    /// An [`MStr`] string.
    String(BorrowedIndex<MStr>),
    /// A list of values.
    List(BorrowedIndex<IndexedList>),
    /// A compound of named entries.
    Compound(BorrowedIndex<IndexedMap>),
    /// A [`u32`] array.
    IntArray(BorrowedIndex<[u32]>),
    /// A [`u64`] array.
    LongArray(BorrowedIndex<[u64]>),
}

/// A borrowed value of any type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BorrowedValue<'data> {
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
    /// A [`u8`] array.
    ByteArray(&'data [u8]),
    /// An [`MStr`] string.
    String(&'data MStr),
    /// A list of values.
    List(IndexedListRef<'data, IndexedList>),
    /// A compound of named entries.
    Compound(IndexedCompoundRef<'data>),
    /// A [`u32`] array.
    IntArray(IndexedListRef<'data, u32>),
    /// A [`u64`] array.
    LongArray(IndexedListRef<'data, u64>),
}

/// The type of an [`BorrowedValueIndex`].
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum BorrowedValueType {
    /// A [`u8`] value.
    Byte,
    /// A [`u16`] value.
    Short,
    /// A [`u32`] value.
    Int,
    /// A [`u64`] value.
    Long,
    /// A [`f32`] value.
    Float,
    /// A [`f64`] value.
    Double,
    /// A [`u8`] array.
    ByteArray,
    /// An [`MStr`] string.
    String,
    /// A list of values.
    List,
    /// A compound of named entries.
    Compound,
    /// A [`u32`] array.
    IntArray,
    /// A [`u64`] array.
    LongArray,
}

impl BorrowedValueIndex {
    /// Get the inner index of this value.
    #[must_use]
    pub(super) const fn index(&self) -> usize {
        match self {
            Self::Byte(index) => index.index(),
            Self::Short(index) => index.index(),
            Self::Int(index) => index.index(),
            Self::Long(index) => index.index(),
            Self::Float(index) => index.index(),
            Self::Double(index) => index.index(),
            Self::ByteArray(index) => index.index(),
            Self::String(index) => index.index(),
            Self::List(index) => index.index(),
            Self::Compound(index) => index.index(),
            Self::IntArray(index) => index.index(),
            Self::LongArray(index) => index.index(),
        }
    }

    /// Get the type of this value.
    #[must_use]
    pub const fn ty(&self) -> BorrowedValueType {
        match self {
            Self::Byte(_) => BorrowedValueType::Byte,
            Self::Short(_) => BorrowedValueType::Short,
            Self::Int(_) => BorrowedValueType::Int,
            Self::Long(_) => BorrowedValueType::Long,
            Self::Float(_) => BorrowedValueType::Float,
            Self::Double(_) => BorrowedValueType::Double,
            Self::ByteArray(_) => BorrowedValueType::ByteArray,
            Self::String(_) => BorrowedValueType::String,
            Self::List(_) => BorrowedValueType::List,
            Self::Compound(_) => BorrowedValueType::Compound,
            Self::IntArray(_) => BorrowedValueType::IntArray,
            Self::LongArray(_) => BorrowedValueType::LongArray,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A list of values.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct IndexedList;

/// A compound of named entries.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct IndexedMap;
