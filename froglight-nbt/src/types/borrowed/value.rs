//! TODO

use core::num::NonZeroUsize;

use froglight_mutf8::prelude::MStr;

use crate::{
    prelude::IndexedCompoundRef,
    types::borrowed::{
        IndexedCoreMut, IndexedCoreRef,
        list::{IndexedListMut, IndexedListRef},
        reference::{BorrowedIndex, BorrowedMut, BorrowedRef},
    },
};

/// A reference to an [`IndexedValue`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct BorrowedValueRef<'data> {
    core: IndexedCoreRef<'data>,
    value: IndexedValue,
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
        core: IndexedCoreRef<'data>,
        value: IndexedValue,
        index: usize,
    ) -> Self {
        Self { core, value, index: NonZeroUsize::new(index) }
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
        core: IndexedCoreRef<'data>,
        value: IndexedValue,
        index: Option<NonZeroUsize>,
    ) -> Self {
        Self { core, value, index }
    }

    /// Get the [`IndexedValueType`] of this value.
    #[inline]
    #[must_use]
    pub const fn ty(&self) -> IndexedValueType { self.value.ty() }

    /// Get the value as a [`u8`].
    ///
    /// Returns `None` if the value is not a [`u8`].
    #[must_use]
    pub fn as_byte(&self) -> Option<u8> {
        match self.value {
            IndexedValue::Byte(index) => {
                Some(unsafe { BorrowedRef::new(self.core.root(), index) }.get_value())
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
            IndexedValue::Short(index) => {
                Some(unsafe { BorrowedRef::new(self.core.root(), index) }.get_value())
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
            IndexedValue::Int(index) => {
                Some(unsafe { BorrowedRef::new(self.core.root(), index) }.get_value())
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
            IndexedValue::Long(index) => {
                Some(unsafe { BorrowedRef::new(self.core.root(), index) }.get_value())
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
            IndexedValue::Float(index) => {
                Some(unsafe { BorrowedRef::new(self.core.root(), index) }.get_value())
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
            IndexedValue::Double(index) => {
                Some(unsafe { BorrowedRef::new(self.core.root(), index) }.get_value())
            }
            _ => None,
        }
    }

    /// Get the value as a byte array.
    ///
    /// Returns `None` if the value is not a byte array.
    #[must_use]
    pub fn as_byte_array(&self) -> Option<&[u8]> {
        match self.value {
            IndexedValue::ByteArray(index) => {
                Some(unsafe { BorrowedRef::new(self.core.root(), index) }.get_ref())
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
            IndexedValue::String(index) => {
                Some(unsafe { BorrowedRef::new(self.core.root(), index) }.get_ref())
            }
            _ => None,
        }
    }

    /// Get the list of values.
    ///
    /// Returns `None` if the value is not a list.
    #[must_use]
    pub fn as_list(&self) -> Option<IndexedListRef<'_, IndexedList>> {
        let IndexedValue::List(_) = self.value else { return None };
        let Some(index) = self.index else {
            #[cfg(debug_assertions)]
            unreachable!("IndexedValue::List must have an index!");
            #[cfg(not(debug_assertions))]
            unsafe {
                core::hint::unreachable_unchecked()
            }
        };

        // SAFETY: The provided `index` is required to be valid for `core`
        Some(unsafe { IndexedListRef::new(self.core.reborrow(), index.get()) })
    }

    /// Get the compound of named entries.
    ///
    /// Returns `None` if the value is not a compound.
    #[must_use]
    pub fn as_compound(&self) -> Option<IndexedCompoundRef<'_>> {
        let IndexedValue::Compound(_) = self.value else { return None };
        let Some(index) = self.index else {
            #[cfg(debug_assertions)]
            unreachable!("IndexedValue::Compound must have an index!");
            #[cfg(not(debug_assertions))]
            unsafe {
                core::hint::unreachable_unchecked()
            }
        };

        // SAFETY: The provided `index` is required to be valid for `core`
        Some(unsafe { IndexedCompoundRef::new(self.core.reborrow(), index.get()) })
    }

    /// Get the value as a [`u32`] array.
    ///
    /// Returns `None` if the value is not a [`u32`] array.
    #[must_use]
    pub fn as_int_array(&self) -> Option<IndexedListRef<'_, u32>> {
        let IndexedValue::IntArray(_) = self.value else { return None };
        let Some(index) = self.index else {
            #[cfg(debug_assertions)]
            unreachable!("IndexedValue::IntArray must have an index!");
            #[cfg(not(debug_assertions))]
            unsafe {
                core::hint::unreachable_unchecked()
            }
        };

        // SAFETY: The provided `index` is required to be valid for `core`
        Some(unsafe { IndexedListRef::new(self.core.reborrow(), index.get()) })
    }

    /// Get the value as a [`u64`] array.
    ///
    /// Returns `None` if the value is not a [`u64`] array.
    #[must_use]
    pub fn as_long_array(&self) -> Option<IndexedListRef<'_, u64>> {
        let IndexedValue::LongArray(_) = self.value else { return None };
        let Some(index) = self.index else {
            #[cfg(debug_assertions)]
            unreachable!("IndexedValue::LongArray must have an index!");
            #[cfg(not(debug_assertions))]
            unsafe {
                core::hint::unreachable_unchecked()
            }
        };

        // SAFETY: The provided `index` is required to be valid for `core`
        Some(unsafe { IndexedListRef::new(self.core.reborrow(), index.get()) })
    }
}

// -------------------------------------------------------------------------------------------------

/// A mutable reference to an [`IndexedValue`].
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct BorrowedValueMut<'data> {
    core: IndexedCoreMut<'data>,
    value: IndexedValue,
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
        core: IndexedCoreMut<'data>,
        value: IndexedValue,
        index: usize,
    ) -> Self {
        Self { core, value, index: NonZeroUsize::new(index) }
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
        core: IndexedCoreMut<'data>,
        value: IndexedValue,
        index: Option<NonZeroUsize>,
    ) -> Self {
        Self { core, value, index }
    }

    /// Get the [`IndexedValueType`] of this value.
    #[inline]
    #[must_use]
    pub const fn ty(&self) -> IndexedValueType { self.value.ty() }

    /// Get the value as a [`u8`].
    ///
    /// Returns `None` if the value is not a [`u8`].
    #[must_use]
    pub fn as_byte(&self) -> Option<u8> {
        match self.value {
            IndexedValue::Byte(index) => {
                Some(unsafe { BorrowedRef::new(self.core.root(), index) }.get_value())
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
            IndexedValue::Byte(index) => {
                unsafe { BorrowedMut::new(self.core.root_mut(), index) }.set_value(value);
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
            IndexedValue::Short(index) => {
                Some(unsafe { BorrowedRef::new(self.core.root(), index) }.get_value())
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
            IndexedValue::Short(index) => {
                unsafe { BorrowedMut::new(self.core.root_mut(), index) }.set_value(value);
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
            IndexedValue::Int(index) => {
                Some(unsafe { BorrowedRef::new(self.core.root(), index) }.get_value())
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
            IndexedValue::Int(index) => {
                unsafe { BorrowedMut::new(self.core.root_mut(), index) }.set_value(value);
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
            IndexedValue::Long(index) => {
                Some(unsafe { BorrowedRef::new(self.core.root(), index) }.get_value())
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
            IndexedValue::Long(index) => {
                unsafe { BorrowedMut::new(self.core.root_mut(), index) }.set_value(value);
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
            IndexedValue::Float(index) => {
                Some(unsafe { BorrowedRef::new(self.core.root(), index) }.get_value())
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
            IndexedValue::Float(index) => {
                unsafe { BorrowedMut::new(self.core.root_mut(), index) }.set_value(value);
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
            IndexedValue::Double(index) => {
                Some(unsafe { BorrowedRef::new(self.core.root(), index) }.get_value())
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
            IndexedValue::Double(index) => {
                unsafe { BorrowedMut::new(self.core.root_mut(), index) }.set_value(value);
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
            IndexedValue::ByteArray(index) => {
                Some(unsafe { BorrowedRef::new(self.core.root(), index) }.get_ref())
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
            IndexedValue::String(index) => {
                Some(unsafe { BorrowedRef::new(self.core.root(), index) }.get_ref())
            }
            _ => None,
        }
    }

    /// Get the list of values.
    ///
    /// Returns `None` if the value is not a list.
    #[must_use]
    pub fn as_list(&self) -> Option<IndexedListRef<'data, ()>> {
        match self.value {
            IndexedValue::List(_index) => todo!(),
            _ => None,
        }
    }

    /// Get the list of values mutably.
    ///
    /// Returns `None` if the value is not a list.
    #[must_use]
    pub fn as_list_mut(&mut self) -> Option<IndexedListMut<'data, ()>> {
        match self.value {
            IndexedValue::List(_index) => todo!(),
            _ => None,
        }
    }

    /// Get the compound of named entries.
    ///
    /// Returns `None` if the value is not a compound.
    #[must_use]
    pub fn as_compound(&self) -> Option<IndexedCompoundRef<'data>> {
        match self.value {
            IndexedValue::Compound(_index) => todo!(),
            _ => None,
        }
    }

    /// Get the compound of named entries mutably.
    ///
    /// Returns `None` if the value is not a compound.
    #[must_use]
    pub fn as_compound_mut(&mut self) -> Option<IndexedCompoundRef<'data>> {
        match self.value {
            IndexedValue::Compound(_index) => todo!(),
            _ => None,
        }
    }

    /// Get the value as a [`u32`] array.
    ///
    /// Returns `None` if the value is not a [`u32`] array.
    #[must_use]
    pub fn as_int_array(&self) -> Option<IndexedListRef<'data, u32>> {
        match self.value {
            IndexedValue::IntArray(_index) => todo!(),
            _ => None,
        }
    }

    /// Get the value as a [`u32`] array mutably.
    ///
    /// Returns `None` if the value is not a [`u32`] array.
    #[must_use]
    pub fn as_int_array_mut(&mut self) -> Option<IndexedListMut<'data, u32>> {
        match self.value {
            IndexedValue::IntArray(_index) => todo!(),
            _ => None,
        }
    }

    /// Get the value as a [`u64`] array.
    ///
    /// Returns `None` if the value is not a [`u64`] array.
    #[must_use]
    pub fn as_long_array(&self) -> Option<IndexedListRef<'data, u64>> {
        match self.value {
            IndexedValue::LongArray(_index) => todo!(),
            _ => None,
        }
    }

    /// Get the value as a [`u64`] array mutably.
    ///
    /// Returns `None` if the value is not a [`u64`] array.
    #[must_use]
    pub fn as_long_array_mut(&mut self) -> Option<IndexedListMut<'data, u64>> {
        match self.value {
            IndexedValue::LongArray(_index) => todo!(),
            _ => None,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// An indexed value of any type.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum IndexedValue {
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

/// The type of an [`IndexedValue`].
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum IndexedValueType {
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

impl IndexedValue {
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
    pub const fn ty(&self) -> IndexedValueType {
        match self {
            Self::Byte(_) => IndexedValueType::Byte,
            Self::Short(_) => IndexedValueType::Short,
            Self::Int(_) => IndexedValueType::Int,
            Self::Long(_) => IndexedValueType::Long,
            Self::Float(_) => IndexedValueType::Float,
            Self::Double(_) => IndexedValueType::Double,
            Self::ByteArray(_) => IndexedValueType::ByteArray,
            Self::String(_) => IndexedValueType::String,
            Self::List(_) => IndexedValueType::List,
            Self::Compound(_) => IndexedValueType::Compound,
            Self::IntArray(_) => IndexedValueType::IntArray,
            Self::LongArray(_) => IndexedValueType::LongArray,
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
