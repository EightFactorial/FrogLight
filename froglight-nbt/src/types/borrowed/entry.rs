#![expect(missing_docs, reason = "WIP")]

use alloc::borrow::Cow;
use core::marker::PhantomData;

use froglight_mutf8::prelude::*;

use crate::types::borrowed::{Mut, NbtIndex, NbtItem, NbtMut, Ref};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedCompound<'a, Mut: NbtMut> {
    root: Mut::Of<'a, [u8]>,
    entries: Cow<'a, [IndexedEntry<'a, Mut>]>,
    _phantom: PhantomData<Mut>,
}

impl<'a, Mut: NbtMut> IndexedCompound<'a, Mut> {
    /// Create a new [`IndexedCompound`] with the given root and entries.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `root` is valid for reads and/or writes,
    /// and that `entries` is a valid index of `root`.
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(
        root: Mut::Of<'a, [u8]>,
        entries: Cow<'a, [IndexedEntry<'a, Mut>]>,
    ) -> Self {
        Self { root, entries, _phantom: PhantomData }
    }

    /// Get the raw data of this [`IndexedCompound`].
    #[inline]
    #[must_use]
    pub fn raw_data(&self) -> &[u8] { &self.root }

    /// Shorten the lifetime of this [`IndexedCompound`] to `'b`.
    #[must_use]
    pub fn shrink<'b>(self) -> IndexedCompound<'b, Mut>
    where
        'a: 'b,
    {
        let owned = self.entries.into_owned();
        let shrink = owned.into_iter().map(IndexedEntry::shrink).collect();
        IndexedCompound {
            root: Mut::shrink(self.root),
            entries: Cow::Owned(shrink),
            _phantom: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexedEntry<'a, Mut: NbtMut> {
    name: NbtIndex<MStr>,
    entry: IndexedTag<'a, Mut>,
}

impl<'a, Mut: NbtMut> IndexedEntry<'a, Mut> {
    /// Create a new [`IndexedEntry`] with the given name and entry.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `name` and `entry` are valid indices for the
    /// [`IndexedCompound`] that contains this entry.
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(name: NbtIndex<MStr>, entry: IndexedTag<'a, Mut>) -> Self {
        Self { name, entry }
    }

    /// Shorten the lifetime of this [`IndexedEntry`] to `'b`.
    #[must_use]
    pub const fn shrink<'b>(self) -> IndexedEntry<'b, Mut>
    where
        'a: 'b,
    {
        IndexedEntry { name: self.name, entry: self.entry.shrink() }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexedTag<'a, Mut: NbtMut> {
    Byte(NbtIndex<u8>),
    Short(NbtIndex<u16>),
    Int(NbtIndex<u32>),
    Long(NbtIndex<u64>),
    Float(NbtIndex<f32>),
    Double(NbtIndex<f64>),
    ByteArray(NbtIndex<[u8]>),
    String(NbtIndex<MStr>),
    List(NbtIndex<[IndexedTag<'a, Mut>]>),
    Compound(NbtIndex<IndexedCompound<'a, Mut>>),
    IntArray(NbtIndex<[u32]>),
    LongArray(NbtIndex<[u64]>),
}

impl<'a, Mut: NbtMut> IndexedTag<'a, Mut> {
    /// Shorten the lifetime of this [`IndexedTag`] to `'b`.
    #[must_use]
    pub const fn shrink<'b>(self) -> IndexedTag<'b, Mut>
    where
        'a: 'b,
    {
        match self {
            Self::Byte(index) => IndexedTag::Byte(index),
            Self::Short(index) => IndexedTag::Short(index),
            Self::Int(index) => IndexedTag::Int(index),
            Self::Long(index) => IndexedTag::Long(index),
            Self::Float(index) => IndexedTag::Float(index),
            Self::Double(index) => IndexedTag::Double(index),
            Self::String(index) => IndexedTag::String(index),
            // SAFETY: `cast` is only used to shrink the lifetime
            Self::List(index) => IndexedTag::List(unsafe { index.cast() }),
            // SAFETY: `cast` is only used to shrink the lifetime
            Self::Compound(index) => IndexedTag::Compound(unsafe { index.cast() }),
            Self::ByteArray(index) => IndexedTag::ByteArray(index),
            Self::IntArray(index) => IndexedTag::IntArray(index),
            Self::LongArray(index) => IndexedTag::LongArray(index),
        }
    }

    /// Get this [`Mut`] as a [`Ref`].
    #[inline]
    #[must_use]
    pub const fn as_ref(&self) -> IndexedTag<'_, Ref> {
        match self {
            Self::Byte(index) => IndexedTag::Byte(*index),
            Self::Short(index) => IndexedTag::Short(*index),
            Self::Int(index) => IndexedTag::Int(*index),
            Self::Long(index) => IndexedTag::Long(*index),
            Self::Float(index) => IndexedTag::Float(*index),
            Self::Double(index) => IndexedTag::Double(*index),
            Self::String(index) => IndexedTag::String(*index),
            // SAFETY: `cast` is used to shrink the lifetime and convert from `Mut` to `Ref`
            Self::List(index) => IndexedTag::List(unsafe { (*index).cast() }),
            // SAFETY: `cast` is used to shrink the lifetime and convert from `Mut` to `Ref`
            Self::Compound(index) => IndexedTag::Compound(unsafe { (*index).cast() }),
            Self::ByteArray(index) => IndexedTag::ByteArray(*index),
            Self::IntArray(index) => IndexedTag::IntArray(*index),
            Self::LongArray(index) => IndexedTag::LongArray(*index),
        }
    }

    /// Convert this [`Mut`] to a [`Ref`].
    #[inline]
    #[must_use]
    pub const fn to_ref(self) -> IndexedTag<'a, Ref> {
        match self {
            Self::Byte(index) => IndexedTag::Byte(index),
            Self::Short(index) => IndexedTag::Short(index),
            Self::Int(index) => IndexedTag::Int(index),
            Self::Long(index) => IndexedTag::Long(index),
            Self::Float(index) => IndexedTag::Float(index),
            Self::Double(index) => IndexedTag::Double(index),
            Self::String(index) => IndexedTag::String(index),
            // SAFETY: `cast` is used to convert from `Mut` to `Ref`
            Self::List(index) => IndexedTag::List(unsafe { index.cast() }),
            // SAFETY: `cast` is used to convert from `Mut` to `Ref`
            Self::Compound(index) => IndexedTag::Compound(unsafe { index.cast() }),
            Self::ByteArray(index) => IndexedTag::ByteArray(index),
            Self::IntArray(index) => IndexedTag::IntArray(index),
            Self::LongArray(index) => IndexedTag::LongArray(index),
        }
    }
}

// -------------------------------------------------------------------------------------------------

pub enum IndexedTagItem<'a, Mut: NbtMut> {
    Byte(NbtItem<'a, u8, Mut>),
    Short(NbtItem<'a, u16, Mut>),
    Int(NbtItem<'a, u32, Mut>),
    Long(NbtItem<'a, u64, Mut>),
    Float(NbtItem<'a, f32, Mut>),
    Double(NbtItem<'a, f64, Mut>),
    ByteArray(NbtItem<'a, [u8], Mut>),
    String(NbtItem<'a, MStr, Mut>),
    List(NbtItem<'a, [IndexedTag<'a, Mut>], Mut>),
    Compound(NbtItem<'a, IndexedCompound<'a, Mut>, Mut>),
    IntArray(NbtItem<'a, [u32], Mut>),
    LongArray(NbtItem<'a, [u64], Mut>),
}

impl<'a, Mut: NbtMut> IndexedTagItem<'a, Mut> {
    const fn new_indexed(root: Mut::Of<'a, [u8]>, index: IndexedTag<'a, Mut>) -> Self {
        match index {
            IndexedTag::Byte(index) => Self::Byte(unsafe { NbtItem::new_indexed(root, index) }),
            IndexedTag::Short(index) => Self::Short(unsafe { NbtItem::new_indexed(root, index) }),
            IndexedTag::Int(index) => Self::Int(unsafe { NbtItem::new_indexed(root, index) }),
            IndexedTag::Long(index) => Self::Long(unsafe { NbtItem::new_indexed(root, index) }),
            IndexedTag::Float(index) => Self::Float(unsafe { NbtItem::new_indexed(root, index) }),
            IndexedTag::Double(index) => Self::Double(unsafe { NbtItem::new_indexed(root, index) }),
            IndexedTag::String(index) => Self::String(unsafe { NbtItem::new_indexed(root, index) }),
            IndexedTag::List(index) => Self::List(unsafe { NbtItem::new_indexed(root, index) }),
            IndexedTag::Compound(index) => {
                Self::Compound(unsafe { NbtItem::new_indexed(root, index) })
            }
            IndexedTag::ByteArray(index) => {
                Self::ByteArray(unsafe { NbtItem::new_indexed(root, index) })
            }
            IndexedTag::IntArray(index) => {
                Self::IntArray(unsafe { NbtItem::new_indexed(root, index) })
            }
            IndexedTag::LongArray(index) => {
                Self::LongArray(unsafe { NbtItem::new_indexed(root, index) })
            }
        }
    }

    pub fn as_byte(&self) -> Option<u8> {
        match self {
            Self::Byte(item) => Some(item.get()),
            _ => None,
        }
    }

    pub fn as_short(&self) -> Option<u16> {
        match self {
            Self::Short(item) => Some(item.get().to_be()),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<u32> {
        match self {
            Self::Int(item) => Some(item.get().to_be()),
            _ => None,
        }
    }

    pub fn as_long(&self) -> Option<u64> {
        match self {
            Self::Long(item) => Some(item.get().to_be()),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&MStr> {
        match self {
            Self::String(item) => Some(item.get()),
            _ => None,
        }
    }

    pub fn as_byte_array(&self) -> Option<&[u8]> {
        match self {
            Self::ByteArray(item) => Some(item.get()),
            _ => None,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<Mut: NbtMut> IndexedCompound<'_, Mut> {
    /// Get the entry with the given name, if it exists.
    ///
    /// Returns `None` if no entry with the given name exists.
    #[must_use]
    pub fn get<'b, T: PartialEq<MStr> + ?Sized>(
        &'b self,
        name: &T,
    ) -> Option<IndexedTagItem<'b, Ref>> {
        for entry in &*self.entries {
            // SAFETY: Valid NBT stores strings as MUTF-8.
            if name == unsafe { entry.name.read(&self.root) } {
                return Some(IndexedTagItem::new_indexed(&*self.root, entry.entry.as_ref()));
            }
        }
        None
    }

    /// Get an iterator over the entries in this [`IndexedCompound`].
    pub fn iter(&self) -> impl Iterator<Item = (NbtItem<'_, MStr, Ref>, IndexedTagItem<'_, Ref>)> {
        self.entries.iter().map(|entry| {
            let name = unsafe { NbtItem::new_indexed(&*self.root, entry.name) };
            let entry = match entry.entry {
                IndexedTag::Byte(index) => unsafe {
                    IndexedTagItem::Byte(NbtItem::new_indexed(&*self.root, index))
                },
                IndexedTag::Short(index) => unsafe {
                    IndexedTagItem::Short(NbtItem::new_indexed(&*self.root, index))
                },
                IndexedTag::Int(index) => unsafe {
                    IndexedTagItem::Int(NbtItem::new_indexed(&*self.root, index))
                },
                IndexedTag::Long(index) => unsafe {
                    IndexedTagItem::Long(NbtItem::new_indexed(&*self.root, index))
                },
                IndexedTag::Float(index) => unsafe {
                    IndexedTagItem::Float(NbtItem::new_indexed(&*self.root, index))
                },
                IndexedTag::Double(index) => unsafe {
                    IndexedTagItem::Double(NbtItem::new_indexed(&*self.root, index))
                },
                IndexedTag::String(index) => unsafe {
                    IndexedTagItem::String(NbtItem::new_indexed(&*self.root, index))
                },

                IndexedTag::List(_) | IndexedTag::Compound(_) => todo!(),

                IndexedTag::ByteArray(index) => unsafe {
                    IndexedTagItem::ByteArray(NbtItem::new_indexed(&*self.root, index))
                },
                IndexedTag::IntArray(index) => unsafe {
                    IndexedTagItem::IntArray(NbtItem::new_indexed(&*self.root, index))
                },
                IndexedTag::LongArray(index) => unsafe {
                    IndexedTagItem::LongArray(NbtItem::new_indexed(&*self.root, index))
                },
            };

            (name, entry)
        })
    }
}

impl<'a> IndexedCompound<'a, Mut> {
    /// Get the entry with the given name, if it exists.
    ///
    /// Returns `None` if no entry with the given name exists.
    #[must_use]
    pub fn get_mut<'b, T: PartialEq<MStr> + ?Sized>(
        &'b mut self,
        name: &T,
    ) -> Option<IndexedTagItem<'b, Mut>>
    where
        'a: 'b,
    {
        for entry in self.entries.to_mut().iter_mut() {
            // SAFETY: Valid NBT stores strings as MUTF-8.
            if name == unsafe { entry.name.read(self.root) } {
                return Some(IndexedTagItem::new_indexed(&mut *self.root, entry.entry.shrink()));
            }
        }
        None
    }
}
