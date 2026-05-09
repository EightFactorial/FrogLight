//! TODO

use froglight_mutf8::prelude::MStr;

use crate::types::borrowed::{
    compound::IndexedCompound,
    core::{IndexCore, Mut, NbtAccess, Ref},
    index::{EntryIndex, IndexedListType, IndexedMapType, ValueIndex},
    list::{IndexedList, IndexedValueList},
    reference::IndexedReference,
};

cfg_select! {
    feature = "alloc" => {
        /// An NBT entry that is indexed by an [`IndexCore`].
        pub struct IndexedEntry<'data, A: NbtAccess, C: IndexCore<A> + 'data = super::core::BorrowedCore<'data, A>> {
            core: A::CORE<'data, C>,
            index: EntryIndex,
        }

        /// An NBT value that is indexed by an [`IndexCore`].
        pub struct IndexedValue<'data, A: NbtAccess, C: IndexCore<A> + 'data = super::core::BorrowedCore<'data, A>> {
            core: A::CORE<'data, C>,
            index: ValueIndex,
        }

        /// A reference to an NBT value that is indexed by an [`IndexCore`].
        pub enum IndexedValueReference<'data, A: NbtAccess, C: IndexCore<A> + 'data = super::core::BorrowedCore<'data, A>> {
            /// A [`u8`] value.
            Byte(IndexedReference<'data, u8, A>),
            /// A [`u16`] value.
            Short(IndexedReference<'data, u16, A>),
            /// A [`u32`] value.
            Int(IndexedReference<'data, u32, A>),
            /// A [`u64`] value.
            Long(IndexedReference<'data, u64, A>),
            /// A [`f32`] value.
            Float(IndexedReference<'data, f32, A>),
            /// A [`f64`] value.
            Double(IndexedReference<'data, f64, A>),
            /// A [`u8`] array.
            ByteArray(IndexedList<'data, [u8], A, C>),
            /// An [`MStr`] string.
            String(IndexedReference<'data, MStr, A>),
            /// A list of values.
            List(IndexedValueList<'data, A, C>),
            /// A compound of named entries.
            Compound(IndexedCompound<'data, A, C>),
            /// A [`u32`] array.
            IntArray(IndexedList<'data, [u32], A, C>),
            /// A [`u64`] array.
            LongArray(IndexedList<'data, [u64], A, C>),
        }
    }
    _ => {
        /// An NBT entry that is indexed by an [`IndexCore`].
        pub struct IndexedEntry<'data, A: NbtAccess, C: IndexCore<A> + 'data> {
            core: A::CORE<'data, C>,
            index: EntryIndex,
        }

        /// An NBT value that is indexed by an [`IndexCore`].
        pub struct IndexedValue<'data, A: NbtAccess, C: IndexCore<A> + 'data> {
            core: A::CORE<'data, C>,
            index: ValueIndex,
        }

        /// A reference to an NBT value that is indexed by an [`IndexCore`].
        pub enum IndexedValueReference<'data, A: NbtAccess, C: IndexCore<A> + 'data> {
            /// A [`u8`] value.
            Byte(IndexedReference<'data, u8, A>),
            /// A [`u16`] value.
            Short(IndexedReference<'data, u16, A>),
            /// A [`u32`] value.
            Int(IndexedReference<'data, u32, A>),
            /// A [`u64`] value.
            Long(IndexedReference<'data, u64, A>),
            /// A [`f32`] value.
            Float(IndexedReference<'data, f32, A>),
            /// A [`f64`] value.
            Double(IndexedReference<'data, f64, A>),
            /// A [`u8`] array.
            ByteArray(IndexedList<'data, u8, A, C>),
            /// An [`MStr`] string.
            String(IndexedReference<'data, MStr, A>),
            /// A list of values.
            List(IndexedValueList<'data, A, C>),
            /// A compound of named entries.
            Compound(IndexedCompound<'data, A, C>),
            /// A [`u32`] array.
            IntArray(IndexedList<'data, u32, A, C>),
            /// A [`u64`] array.
            LongArray(IndexedList<'data, u64, A, C>),
        }
    }
}

impl<'data, A: NbtAccess, C: IndexCore<A> + 'data> IndexedEntry<'data, A, C> {
    /// Create a new [`IndexedEntry`] from the given core and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the indexes are valid for the given core.
    #[inline]
    #[must_use]
    pub const unsafe fn new(core: A::CORE<'data, C>, index: EntryIndex) -> Self {
        Self { core, index }
    }
}

impl<'data, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A> + 'data> IndexedEntry<'data, A, C> {
    /// Get the [`IndexedReference`] to the name of this entry.
    #[inline]
    #[must_use]
    pub fn name(&self) -> IndexedReference<'_, MStr, Ref> {
        // SAFETY: `IndexedEntry` guarantees that `index` is valid for `core`.
        unsafe { IndexedReference::new(<C as IndexCore<A>>::root(&self.core), self.index.name()) }
    }

    /// Get the [`IndexedValue`] of this entry.
    #[inline]
    #[must_use]
    pub fn value(&self) -> IndexedValue<'_, Ref, C> {
        // SAFETY: `IndexedEntry` guarantees that `index` is valid for `core`.
        unsafe { IndexedValue::<Ref, C>::new(&self.core, self.index.value()) }
    }
}

impl<'data, C: IndexCore<Mut> + 'data> IndexedEntry<'data, Mut, C> {
    /// Get the [`IndexedValue`] of this entry.
    #[inline]
    #[must_use]
    pub fn value_mut(&mut self) -> IndexedValue<'_, Mut, C> {
        // SAFETY: `IndexedEntry` guarantees that `index` is valid for `core`.
        unsafe { IndexedValue::<Mut, C>::new(self.core, self.index.value()) }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'data, A: NbtAccess, C: IndexCore<A> + 'data> IndexedValue<'data, A, C> {
    /// Create a new [`IndexedValue`] from the given core and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is valid for the given core.
    #[inline]
    #[must_use]
    pub const unsafe fn new(core: A::CORE<'data, C>, index: ValueIndex) -> Self {
        Self { core, index }
    }
}

impl<'data, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A> + 'data> IndexedValue<'data, A, C> {
    /// Get the value of this entry as an [`IndexedValueReference`].
    #[must_use]
    #[expect(clippy::too_many_lines, reason = "Many cases")]
    pub fn as_value(&self) -> IndexedValueReference<'_, Ref, C> {
        match self.index {
            ValueIndex::Byte(index) => IndexedValueReference::Byte(unsafe {
                IndexedReference::new(<C as IndexCore<A>>::root(&self.core), index)
            }),
            ValueIndex::Short(index) => IndexedValueReference::Short(unsafe {
                IndexedReference::new(<C as IndexCore<A>>::root(&self.core), index)
            }),
            ValueIndex::Int(index) => IndexedValueReference::Int(unsafe {
                IndexedReference::new(<C as IndexCore<A>>::root(&self.core), index)
            }),
            ValueIndex::Long(index) => IndexedValueReference::Long(unsafe {
                IndexedReference::new(<C as IndexCore<A>>::root(&self.core), index)
            }),
            ValueIndex::Float(index) => IndexedValueReference::Float(unsafe {
                IndexedReference::new(<C as IndexCore<A>>::root(&self.core), index)
            }),
            ValueIndex::Double(index) => IndexedValueReference::Double(unsafe {
                IndexedReference::new(<C as IndexCore<A>>::root(&self.core), index)
            }),
            ValueIndex::String(index) => IndexedValueReference::String(unsafe {
                IndexedReference::new(<C as IndexCore<A>>::root(&self.core), index)
            }),

            ValueIndex::ByteArray(index) => IndexedValueReference::ByteArray(unsafe {
                IndexedList::<[u8], Ref, C>::new(&self.core, index)
            }),
            ValueIndex::IntArray(index) => IndexedValueReference::IntArray(unsafe {
                IndexedList::<[u32], Ref, C>::new(&self.core, index)
            }),
            ValueIndex::LongArray(index) => IndexedValueReference::LongArray(unsafe {
                IndexedList::<[u64], Ref, C>::new(&self.core, index)
            }),

            ValueIndex::Compound(index) => IndexedValueReference::Compound(unsafe {
                IndexedCompound::<Ref, C>::new(&self.core, index.value())
            }),

            ValueIndex::List(index) => {
                let entries =
                    unsafe { <C as IndexCore<A>>::entry_range(&self.core, index.value()) };

                // Get the first entry, or return an empty list.
                let Some(first) = entries.first() else {
                    return IndexedValueReference::List(IndexedValueList::Empty);
                };

                match first.value() {
                    ValueIndex::Byte(index) => {
                        IndexedValueReference::List(IndexedValueList::Byte(unsafe {
                            IndexedList::<u8, Ref, C>::new(&self.core, index)
                        }))
                    }
                    ValueIndex::Short(index) => {
                        IndexedValueReference::List(IndexedValueList::Short(unsafe {
                            IndexedList::<u16, Ref, C>::new(&self.core, index)
                        }))
                    }
                    ValueIndex::Int(index) => {
                        IndexedValueReference::List(IndexedValueList::Int(unsafe {
                            IndexedList::<u32, Ref, C>::new(&self.core, index)
                        }))
                    }
                    ValueIndex::Long(index) => {
                        IndexedValueReference::List(IndexedValueList::Long(unsafe {
                            IndexedList::<u64, Ref, C>::new(&self.core, index)
                        }))
                    }
                    ValueIndex::Float(index) => {
                        IndexedValueReference::List(IndexedValueList::Float(unsafe {
                            IndexedList::<f32, Ref, C>::new(&self.core, index)
                        }))
                    }
                    ValueIndex::Double(index) => {
                        IndexedValueReference::List(IndexedValueList::Double(unsafe {
                            IndexedList::<f64, Ref, C>::new(&self.core, index)
                        }))
                    }
                    ValueIndex::ByteArray(index) => {
                        IndexedValueReference::List(IndexedValueList::ByteArray(unsafe {
                            IndexedList::<[u8], Ref, C>::new(&self.core, index)
                        }))
                    }
                    ValueIndex::String(index) => {
                        IndexedValueReference::List(IndexedValueList::String(unsafe {
                            IndexedList::<MStr, Ref, C>::new(&self.core, index)
                        }))
                    }
                    ValueIndex::List(index) => {
                        IndexedValueReference::List(IndexedValueList::List(unsafe {
                            IndexedList::<IndexedListType, Ref, C>::new(&self.core, index)
                        }))
                    }
                    ValueIndex::Compound(index) => {
                        IndexedValueReference::List(IndexedValueList::Compound(unsafe {
                            IndexedList::<IndexedMapType, Ref, C>::new(&self.core, index)
                        }))
                    }
                    ValueIndex::IntArray(index) => {
                        IndexedValueReference::List(IndexedValueList::IntArray(unsafe {
                            IndexedList::<[u32], Ref, C>::new(&self.core, index)
                        }))
                    }
                    ValueIndex::LongArray(index) => {
                        IndexedValueReference::List(IndexedValueList::LongArray(unsafe {
                            IndexedList::<[u64], Ref, C>::new(&self.core, index)
                        }))
                    }
                }
            }
        }
    }

    /// Get this value as a byte, else `None`.
    #[must_use]
    pub fn as_byte(&self) -> Option<IndexedReference<'_, u8, Ref>> {
        if let ValueIndex::Byte(index) = self.index {
            Some(unsafe { IndexedReference::new(<C as IndexCore<A>>::root(&self.core), index) })
        } else {
            None
        }
    }

    /// Get this value as a short, else `None`.
    #[must_use]
    pub fn as_short(&self) -> Option<IndexedReference<'_, u16, Ref>> {
        if let ValueIndex::Short(index) = self.index {
            Some(unsafe { IndexedReference::new(<C as IndexCore<A>>::root(&self.core), index) })
        } else {
            None
        }
    }

    /// Get this value as an int, else `None`.
    #[must_use]
    pub fn as_int(&self) -> Option<IndexedReference<'_, u32, Ref>> {
        if let ValueIndex::Int(index) = self.index {
            Some(unsafe { IndexedReference::new(<C as IndexCore<A>>::root(&self.core), index) })
        } else {
            None
        }
    }

    /// Get this value as a long, else `None`.
    #[must_use]
    pub fn as_long(&self) -> Option<IndexedReference<'_, u64, Ref>> {
        if let ValueIndex::Long(index) = self.index {
            Some(unsafe { IndexedReference::new(<C as IndexCore<A>>::root(&self.core), index) })
        } else {
            None
        }
    }

    /// Get this value as a float, else `None`.
    #[must_use]
    pub fn as_float(&self) -> Option<IndexedReference<'_, f32, Ref>> {
        if let ValueIndex::Float(index) = self.index {
            Some(unsafe { IndexedReference::new(<C as IndexCore<A>>::root(&self.core), index) })
        } else {
            None
        }
    }

    /// Get this value as a double, else `None`.
    #[must_use]
    pub fn as_double(&self) -> Option<IndexedReference<'_, f64, Ref>> {
        if let ValueIndex::Double(index) = self.index {
            Some(unsafe { IndexedReference::new(<C as IndexCore<A>>::root(&self.core), index) })
        } else {
            None
        }
    }

    /// Get this value as a string, else `None`.
    #[must_use]
    pub fn as_string(&self) -> Option<IndexedReference<'_, MStr, Ref>> {
        if let ValueIndex::String(index) = self.index {
            Some(unsafe { IndexedReference::new(<C as IndexCore<A>>::root(&self.core), index) })
        } else {
            None
        }
    }

    /// Get this value as a byte array, else `None`.
    #[must_use]
    pub fn as_byte_array(&self) -> Option<IndexedList<'_, [u8], Ref, C>> {
        if let ValueIndex::ByteArray(index) = self.index {
            Some(unsafe { IndexedList::<[u8], Ref, C>::new(&self.core, index) })
        } else {
            None
        }
    }

    /// Get this value as an int array, else `None`.
    #[must_use]
    pub fn as_int_array(&self) -> Option<IndexedList<'_, [u32], Ref, C>> {
        if let ValueIndex::IntArray(index) = self.index {
            Some(unsafe { IndexedList::<[u32], Ref, C>::new(&self.core, index) })
        } else {
            None
        }
    }

    /// Get this value as a long array, else `None`.
    #[must_use]
    pub fn as_long_array(&self) -> Option<IndexedList<'_, [u64], Ref, C>> {
        if let ValueIndex::LongArray(index) = self.index {
            Some(unsafe { IndexedList::<[u64], Ref, C>::new(&self.core, index) })
        } else {
            None
        }
    }

    /// Get this value as a compound, else `None`.
    #[must_use]
    pub fn as_compound(&self) -> Option<IndexedCompound<'_, Ref, C>> {
        if let ValueIndex::Compound(index) = self.index {
            Some(unsafe { IndexedCompound::<Ref, C>::new(&self.core, index.value()) })
        } else {
            None
        }
    }

    /// Get this value as a list, else `None`.
    #[must_use]
    pub fn as_list(&self) -> Option<IndexedValueList<'_, Ref, C>> {
        if let ValueIndex::List(index) = self.index {
            let entries = unsafe { <C as IndexCore<A>>::entry_range(&self.core, index.value()) };

            // Get the first entry, or return an empty list.
            let Some(first) = entries.first() else {
                return Some(IndexedValueList::Empty);
            };

            match first.value() {
                ValueIndex::Byte(index) => Some(IndexedValueList::Byte(unsafe {
                    IndexedList::<u8, Ref, C>::new(&self.core, index)
                })),
                ValueIndex::Short(index) => Some(IndexedValueList::Short(unsafe {
                    IndexedList::<u16, Ref, C>::new(&self.core, index)
                })),
                ValueIndex::Int(index) => Some(IndexedValueList::Int(unsafe {
                    IndexedList::<u32, Ref, C>::new(&self.core, index)
                })),
                ValueIndex::Long(index) => Some(IndexedValueList::Long(unsafe {
                    IndexedList::<u64, Ref, C>::new(&self.core, index)
                })),
                ValueIndex::Float(index) => Some(IndexedValueList::Float(unsafe {
                    IndexedList::<f32, Ref, C>::new(&self.core, index)
                })),
                ValueIndex::Double(index) => Some(IndexedValueList::Double(unsafe {
                    IndexedList::<f64, Ref, C>::new(&self.core, index)
                })),
                ValueIndex::ByteArray(index) => Some(IndexedValueList::ByteArray(unsafe {
                    IndexedList::<[u8], Ref, C>::new(&self.core, index)
                })),
                ValueIndex::String(index) => Some(IndexedValueList::String(unsafe {
                    IndexedList::<MStr, Ref, C>::new(&self.core, index)
                })),
                ValueIndex::List(index) => Some(IndexedValueList::List(unsafe {
                    IndexedList::<IndexedListType, Ref, C>::new(&self.core, index)
                })),
                ValueIndex::Compound(index) => Some(IndexedValueList::Compound(unsafe {
                    IndexedList::<IndexedMapType, Ref, C>::new(&self.core, index)
                })),
                ValueIndex::IntArray(index) => Some(IndexedValueList::IntArray(unsafe {
                    IndexedList::<[u32], Ref, C>::new(&self.core, index)
                })),
                ValueIndex::LongArray(index) => Some(IndexedValueList::LongArray(unsafe {
                    IndexedList::<[u64], Ref, C>::new(&self.core, index)
                })),
            }
        } else {
            None
        }
    }
}

impl<'data, C: IndexCore<Mut> + 'data> IndexedValue<'data, Mut, C> {
    /// Get the value of this entry as a mutable [`IndexedValueReference`].
    #[must_use]
    #[expect(clippy::too_many_lines, reason = "Many cases")]
    pub fn as_value_mut(&mut self) -> IndexedValueReference<'_, Mut, C> {
        match self.index {
            ValueIndex::Byte(index) => IndexedValueReference::Byte(unsafe {
                IndexedReference::new(self.core.root_mut(), index)
            }),
            ValueIndex::Short(index) => IndexedValueReference::Short(unsafe {
                IndexedReference::new(self.core.root_mut(), index)
            }),
            ValueIndex::Int(index) => IndexedValueReference::Int(unsafe {
                IndexedReference::new(self.core.root_mut(), index)
            }),
            ValueIndex::Long(index) => IndexedValueReference::Long(unsafe {
                IndexedReference::new(self.core.root_mut(), index)
            }),
            ValueIndex::Float(index) => IndexedValueReference::Float(unsafe {
                IndexedReference::new(self.core.root_mut(), index)
            }),
            ValueIndex::Double(index) => IndexedValueReference::Double(unsafe {
                IndexedReference::new(self.core.root_mut(), index)
            }),
            ValueIndex::String(index) => IndexedValueReference::String(unsafe {
                IndexedReference::new(self.core.root_mut(), index)
            }),

            ValueIndex::ByteArray(index) => IndexedValueReference::ByteArray(unsafe {
                IndexedList::<[u8], Mut, C>::new(&mut self.core, index)
            }),
            ValueIndex::IntArray(index) => IndexedValueReference::IntArray(unsafe {
                IndexedList::<[u32], Mut, C>::new(&mut self.core, index)
            }),
            ValueIndex::LongArray(index) => IndexedValueReference::LongArray(unsafe {
                IndexedList::<[u64], Mut, C>::new(&mut self.core, index)
            }),

            ValueIndex::Compound(index) => IndexedValueReference::Compound(unsafe {
                IndexedCompound::<Mut, C>::new(self.core, index.value())
            }),
            ValueIndex::List(index) => {
                let entries = unsafe { self.core.entry_range(index.value()) };

                let Some(first) = entries.first() else {
                    return IndexedValueReference::List(IndexedValueList::Empty);
                };

                match first.value() {
                    ValueIndex::Byte(index) => {
                        IndexedValueReference::List(IndexedValueList::Byte(unsafe {
                            IndexedList::<u8, Mut, C>::new(&mut self.core, index)
                        }))
                    }
                    ValueIndex::Short(index) => {
                        IndexedValueReference::List(IndexedValueList::Short(unsafe {
                            IndexedList::<u16, Mut, C>::new(&mut self.core, index)
                        }))
                    }
                    ValueIndex::Int(index) => {
                        IndexedValueReference::List(IndexedValueList::Int(unsafe {
                            IndexedList::<u32, Mut, C>::new(&mut self.core, index)
                        }))
                    }
                    ValueIndex::Long(index) => {
                        IndexedValueReference::List(IndexedValueList::Long(unsafe {
                            IndexedList::<u64, Mut, C>::new(&mut self.core, index)
                        }))
                    }
                    ValueIndex::Float(index) => {
                        IndexedValueReference::List(IndexedValueList::Float(unsafe {
                            IndexedList::<f32, Mut, C>::new(&mut self.core, index)
                        }))
                    }
                    ValueIndex::Double(index) => {
                        IndexedValueReference::List(IndexedValueList::Double(unsafe {
                            IndexedList::<f64, Mut, C>::new(&mut self.core, index)
                        }))
                    }
                    ValueIndex::ByteArray(index) => {
                        IndexedValueReference::List(IndexedValueList::ByteArray(unsafe {
                            IndexedList::<[u8], Mut, C>::new(&mut self.core, index)
                        }))
                    }
                    ValueIndex::String(index) => {
                        IndexedValueReference::List(IndexedValueList::String(unsafe {
                            IndexedList::<MStr, Mut, C>::new(&mut self.core, index)
                        }))
                    }
                    ValueIndex::List(index) => {
                        IndexedValueReference::List(IndexedValueList::List(unsafe {
                            IndexedList::<IndexedListType, Mut, C>::new(&mut self.core, index)
                        }))
                    }
                    ValueIndex::Compound(index) => {
                        IndexedValueReference::List(IndexedValueList::Compound(unsafe {
                            IndexedList::<IndexedMapType, Mut, C>::new(&mut self.core, index)
                        }))
                    }
                    ValueIndex::IntArray(index) => {
                        IndexedValueReference::List(IndexedValueList::IntArray(unsafe {
                            IndexedList::<[u32], Mut, C>::new(&mut self.core, index)
                        }))
                    }
                    ValueIndex::LongArray(index) => {
                        IndexedValueReference::List(IndexedValueList::LongArray(unsafe {
                            IndexedList::<[u64], Mut, C>::new(&mut self.core, index)
                        }))
                    }
                }
            }
        }
    }

    /// Get this value as a mutable byte, else `None`.
    #[must_use]
    pub fn as_byte_mut(&mut self) -> Option<IndexedReference<'_, u8, Mut>> {
        if let ValueIndex::Byte(index) = self.index {
            Some(unsafe { IndexedReference::new(self.core.root_mut(), index) })
        } else {
            None
        }
    }

    /// Get this value as a mutable short, else `None`.
    #[must_use]
    pub fn as_short_mut(&mut self) -> Option<IndexedReference<'_, u16, Mut>> {
        if let ValueIndex::Short(index) = self.index {
            Some(unsafe { IndexedReference::new(self.core.root_mut(), index) })
        } else {
            None
        }
    }

    /// Get this value as a mutable int, else `None`.
    #[must_use]
    pub fn as_int_mut(&mut self) -> Option<IndexedReference<'_, u32, Mut>> {
        if let ValueIndex::Int(index) = self.index {
            Some(unsafe { IndexedReference::new(self.core.root_mut(), index) })
        } else {
            None
        }
    }

    /// Get this value as a mutable long, else `None`.
    #[must_use]
    pub fn as_long_mut(&mut self) -> Option<IndexedReference<'_, u64, Mut>> {
        if let ValueIndex::Long(index) = self.index {
            Some(unsafe { IndexedReference::new(self.core.root_mut(), index) })
        } else {
            None
        }
    }

    /// Get this value as a mutable float, else `None`.
    #[must_use]
    pub fn as_float_mut(&mut self) -> Option<IndexedReference<'_, f32, Mut>> {
        if let ValueIndex::Float(index) = self.index {
            Some(unsafe { IndexedReference::new(self.core.root_mut(), index) })
        } else {
            None
        }
    }

    /// Get this value as a mutable double, else `None`.
    #[must_use]
    pub fn as_double_mut(&mut self) -> Option<IndexedReference<'_, f64, Mut>> {
        if let ValueIndex::Double(index) = self.index {
            Some(unsafe { IndexedReference::new(self.core.root_mut(), index) })
        } else {
            None
        }
    }

    /// Get this value as a mutable string, else `None`.
    #[must_use]
    pub fn as_string_mut(&mut self) -> Option<IndexedReference<'_, MStr, Mut>> {
        if let ValueIndex::String(index) = self.index {
            Some(unsafe { IndexedReference::new(self.core.root_mut(), index) })
        } else {
            None
        }
    }

    /// Get this value as a mutable byte array, else `None`.
    #[must_use]
    pub fn as_byte_array_mut(&mut self) -> Option<IndexedList<'_, [u8], Mut, C>> {
        if let ValueIndex::ByteArray(index) = self.index {
            Some(unsafe { IndexedList::<[u8], Mut, C>::new(&mut self.core, index) })
        } else {
            None
        }
    }

    /// Get this value as a mutable int array, else `None`.
    #[must_use]
    pub fn as_int_array_mut(&mut self) -> Option<IndexedList<'_, [u32], Mut, C>> {
        if let ValueIndex::IntArray(index) = self.index {
            Some(unsafe { IndexedList::<[u32], Mut, C>::new(&mut self.core, index) })
        } else {
            None
        }
    }

    /// Get this value as a mutable long array, else `None`.
    #[must_use]
    pub fn as_long_array_mut(&mut self) -> Option<IndexedList<'_, [u64], Mut, C>> {
        if let ValueIndex::LongArray(index) = self.index {
            Some(unsafe { IndexedList::<[u64], Mut, C>::new(&mut self.core, index) })
        } else {
            None
        }
    }

    /// Get this value as a mutable compound, else `None`.
    #[must_use]
    pub fn as_compound_mut(&mut self) -> Option<IndexedCompound<'_, Mut, C>> {
        if let ValueIndex::Compound(index) = self.index {
            Some(unsafe { IndexedCompound::<Mut, C>::new(self.core, index.value()) })
        } else {
            None
        }
    }

    /// Get this value as a mutable list, else `None`.
    #[must_use]
    pub fn as_list_mut(&mut self) -> Option<IndexedValueList<'_, Mut, C>> {
        if let ValueIndex::List(index) = self.index {
            let entries = unsafe { self.core.entry_range(index.value()) };

            let Some(first) = entries.first() else {
                return Some(IndexedValueList::Empty);
            };

            match first.value() {
                ValueIndex::Byte(index) => Some(IndexedValueList::Byte(unsafe {
                    IndexedList::<u8, Mut, C>::new(&mut self.core, index)
                })),
                ValueIndex::Short(index) => Some(IndexedValueList::Short(unsafe {
                    IndexedList::<u16, Mut, C>::new(&mut self.core, index)
                })),
                ValueIndex::Int(index) => Some(IndexedValueList::Int(unsafe {
                    IndexedList::<u32, Mut, C>::new(&mut self.core, index)
                })),
                ValueIndex::Long(index) => Some(IndexedValueList::Long(unsafe {
                    IndexedList::<u64, Mut, C>::new(&mut self.core, index)
                })),
                ValueIndex::Float(index) => Some(IndexedValueList::Float(unsafe {
                    IndexedList::<f32, Mut, C>::new(&mut self.core, index)
                })),
                ValueIndex::Double(index) => Some(IndexedValueList::Double(unsafe {
                    IndexedList::<f64, Mut, C>::new(&mut self.core, index)
                })),
                ValueIndex::ByteArray(index) => Some(IndexedValueList::ByteArray(unsafe {
                    IndexedList::<[u8], Mut, C>::new(&mut self.core, index)
                })),
                ValueIndex::String(index) => Some(IndexedValueList::String(unsafe {
                    IndexedList::<MStr, Mut, C>::new(&mut self.core, index)
                })),
                ValueIndex::List(index) => Some(IndexedValueList::List(unsafe {
                    IndexedList::<IndexedListType, Mut, C>::new(&mut self.core, index)
                })),
                ValueIndex::Compound(index) => Some(IndexedValueList::Compound(unsafe {
                    IndexedList::<IndexedMapType, Mut, C>::new(&mut self.core, index)
                })),
                ValueIndex::IntArray(index) => Some(IndexedValueList::IntArray(unsafe {
                    IndexedList::<[u32], Mut, C>::new(&mut self.core, index)
                })),
                ValueIndex::LongArray(index) => Some(IndexedValueList::LongArray(unsafe {
                    IndexedList::<[u64], Mut, C>::new(&mut self.core, index)
                })),
            }
        } else {
            None
        }
    }
}
