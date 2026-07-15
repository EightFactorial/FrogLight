use froglight_mutf8::prelude::MStr;

use crate::types::indexed::{
    compound::IndexedCompound,
    core::{IndexCore, Mut, NbtAccess, Ref},
    entry::IndexedValue,
    index::{Index, ValueIndex},
    list::{IndexedList, IndexedValueList},
    reference::{IndexableValue, IndexedReference, IndexedValueReference},
    types::IndexedListType,
};

impl<'data, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A> + 'data> IndexedValue<'data, A, C> {
    /// Return a reference to the stored value.
    #[must_use]
    pub fn as_value(&self) -> IndexedValueReference<'_, Ref, C> {
        let root = <C as IndexCore<A>>::root(&self.core);
        match self.index {
            ValueIndex::Byte(index) => IndexedValueReference::Byte(unsafe {
                IndexedReference::<u8, Ref>::new(root, index)
            }),
            ValueIndex::Short(index) => IndexedValueReference::Short(unsafe {
                IndexedReference::<u16, Ref>::new(root, index)
            }),
            ValueIndex::Int(index) => IndexedValueReference::Int(unsafe {
                IndexedReference::<u32, Ref>::new(root, index)
            }),
            ValueIndex::Long(index) => IndexedValueReference::Long(unsafe {
                IndexedReference::<u64, Ref>::new(root, index)
            }),
            ValueIndex::Float(index) => IndexedValueReference::Float(unsafe {
                IndexedReference::<f32, Ref>::new(root, index)
            }),
            ValueIndex::Double(index) => IndexedValueReference::Double(unsafe {
                IndexedReference::<f64, Ref>::new(root, index)
            }),
            ValueIndex::ByteArray(index) => IndexedValueReference::ByteArray(unsafe {
                IndexedReference::<[u8], Ref>::new(root, index)
            }),
            ValueIndex::String(index) => IndexedValueReference::String(unsafe {
                IndexedReference::<MStr, Ref>::new(root, index)
            }),
            ValueIndex::IntArray(index) => IndexedValueReference::IntArray(unsafe {
                IndexedReference::<[u32], Ref>::new(root, index)
            }),
            ValueIndex::LongArray(index) => IndexedValueReference::LongArray(unsafe {
                IndexedReference::<[u64], Ref>::new(root, index)
            }),

            ValueIndex::List(index) => IndexedValueReference::List(create_list(&self.core, index)),
            ValueIndex::Compound(index) => IndexedValueReference::Compound(unsafe {
                IndexedCompound::<Ref, C>::new(&self.core, index.value())
            }),
        }
    }
}

impl<'data, C: IndexCore<Mut> + 'data> IndexedValue<'data, Mut, C> {
    /// Return a reference to the stored value.
    #[must_use]
    pub fn as_value_mut(&mut self) -> IndexedValueReference<'_, Mut, C> {
        match self.index {
            ValueIndex::Byte(index) => IndexedValueReference::Byte(unsafe {
                let root = <C as IndexCore<Mut>>::root_mut(self.core);
                IndexedReference::<u8, Mut>::new(root, index)
            }),
            ValueIndex::Short(index) => IndexedValueReference::Short(unsafe {
                let root = <C as IndexCore<Mut>>::root_mut(self.core);
                IndexedReference::<u16, Mut>::new(root, index)
            }),
            ValueIndex::Int(index) => IndexedValueReference::Int(unsafe {
                let root = <C as IndexCore<Mut>>::root_mut(self.core);
                IndexedReference::<u32, Mut>::new(root, index)
            }),
            ValueIndex::Long(index) => IndexedValueReference::Long(unsafe {
                let root = <C as IndexCore<Mut>>::root_mut(self.core);
                IndexedReference::<u64, Mut>::new(root, index)
            }),
            ValueIndex::Float(index) => IndexedValueReference::Float(unsafe {
                let root = <C as IndexCore<Mut>>::root_mut(self.core);
                IndexedReference::<f32, Mut>::new(root, index)
            }),
            ValueIndex::Double(index) => IndexedValueReference::Double(unsafe {
                let root = <C as IndexCore<Mut>>::root_mut(self.core);
                IndexedReference::<f64, Mut>::new(root, index)
            }),
            ValueIndex::ByteArray(index) => IndexedValueReference::ByteArray(unsafe {
                let root = <C as IndexCore<Mut>>::root_mut(self.core);
                IndexedReference::<[u8], Mut>::new(root, index)
            }),
            ValueIndex::String(index) => IndexedValueReference::String(unsafe {
                let root = <C as IndexCore<Mut>>::root_mut(self.core);
                IndexedReference::<MStr, Mut>::new(root, index)
            }),
            ValueIndex::IntArray(index) => IndexedValueReference::IntArray(unsafe {
                let root = <C as IndexCore<Mut>>::root_mut(self.core);
                IndexedReference::<[u32], Mut>::new(root, index)
            }),
            ValueIndex::LongArray(index) => IndexedValueReference::LongArray(unsafe {
                let root = <C as IndexCore<Mut>>::root_mut(self.core);
                IndexedReference::<[u64], Mut>::new(root, index)
            }),

            ValueIndex::List(index) => {
                IndexedValueReference::List(create_list_mut(self.core, index))
            }
            ValueIndex::Compound(index) => IndexedValueReference::Compound(unsafe {
                IndexedCompound::<Mut, C>::new(self.core, index.value())
            }),
        }
    }
}

// -------------------------------------------------------------------------------------------------

macro_rules! create_fns {
    (@ref $($as_ident:ident $(& $into_ident:ident)?: $ty:ty => $variant:ident),*) => {
        impl<'data, A: NbtAccess, C: IndexCore<A> + 'data> IndexedValue<'data, A, C> {
            $(
                #[must_use]
                #[doc = concat!("Return a reference to the stored value if it is of type [`", stringify!($ty), "`], else `None`.")]
                pub fn $as_ident(&self) -> Option<IndexedReference<'_, $ty, Ref>> {
                    if let ValueIndex::$variant(value) = self.index {
                        let root = <C as IndexCore<A>>::root(&self.core);
                        Some(unsafe { IndexedReference::<$ty, Ref>::new(root, value) })
                    } else {
                        None
                    }
                }

                $(
                    #[inline]
                    #[must_use]
                    #[doc = concat!("Return the stored value if it is of type [`", stringify!($ty), "`], else `None`.")]
                    pub fn $into_ident(self) -> Option<<$ty as IndexableValue>::Value<'data>> {
                        if let ValueIndex::$variant(value) = self.index {
                            let root = <C as IndexCore<A>>::root(A::into_core(self.core));
                            Some(unsafe { IndexedReference::<$ty, Ref>::new(root, value) }.into_value())
                        } else {
                            None
                        }
                    }
                )?
            )*
        }
    };
    (@mut $($ident:ident: $ty:ty => $variant:ident),*) => {
        impl<'data, C: IndexCore<Mut> + 'data> IndexedValue<'data, Mut, C> {
            $(
                #[must_use]
                #[doc = concat!("Return a mutable reference to the stored value if it is of type [`", stringify!($ty), "`], else `None`.")]
                pub fn $ident(&mut self) -> Option<IndexedReference<'_, $ty, Mut>> {
                    if let ValueIndex::$variant(value) = self.index {
                        let root = <C as IndexCore<Mut>>::root_mut(self.core);
                        Some(unsafe { IndexedReference::<$ty, Mut>::new(root, value) })
                    } else {
                        None
                    }
                }
            )*
        }
    };
}

create_fns! {
    @ref
    as_byte & into_byte: u8 => Byte,
    as_short & into_short: u16 => Short,
    as_int & into_int: u32 => Int,
    as_long & into_long: u64 => Long,
    as_float & into_float: f32 => Float,
    as_double & into_double: f64 => Double,
    as_byte_array & into_byte_array: [u8] => ByteArray,
    as_string & into_string: MStr => String,
    as_int_array & into_int_array: [u32] => IntArray,
    as_long_array & into_long_array: [u64] => LongArray
}
create_fns! {
    @mut
    as_byte_mut: u8 => Byte,
    as_short_mut: u16 => Short,
    as_int_mut: u32 => Int,
    as_long_mut: u64 => Long,
    as_float_mut: f32 => Float,
    as_double_mut: f64 => Double,
    as_byte_array_mut: [u8] => ByteArray,
    as_string_mut: MStr => String,
    as_int_array_mut: [u32] => IntArray,
    as_long_array_mut: [u64] => LongArray
}

// -------------------------------------------------------------------------------------------------

impl<'data, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A> + 'data> IndexedValue<'data, A, C> {
    /// Return a reference to the stored value if it is of type
    /// [`IndexedCompound`], else else `None`.
    #[must_use]
    pub fn as_compound(&self) -> Option<IndexedCompound<'_, Ref, C>> {
        if let ValueIndex::Compound(value) = self.index {
            Some(unsafe { IndexedCompound::<Ref, C>::new(&self.core, value.value()) })
        } else {
            None
        }
    }

    /// Return a reference to the stored value if it is of type
    /// [`IndexedValueList`], else else `None`.
    #[must_use]
    pub fn as_list(&self) -> Option<IndexedValueList<'_, Ref, C>> {
        if let ValueIndex::List(index) = self.index {
            Some(create_list(&self.core, index))
        } else {
            None
        }
    }
}

impl<'data, C: IndexCore<Mut> + 'data> IndexedValue<'data, Mut, C> {
    /// Return a mutable reference to the stored value if it is of type
    /// [`IndexedCompound`], else else `None`.
    pub fn as_compound_mut(&mut self) -> Option<IndexedCompound<'_, Mut, C>> {
        if let ValueIndex::Compound(value) = self.index {
            Some(unsafe { IndexedCompound::<Mut, C>::new(self.core, value.value()) })
        } else {
            None
        }
    }

    /// Return a mutable reference to the stored value if it is of type
    /// [`IndexedValueList`], else else `None`.
    pub fn as_list_mut(&mut self) -> Option<IndexedValueList<'_, Mut, C>> {
        if let ValueIndex::List(index) = self.index {
            Some(create_list_mut(self.core, index))
        } else {
            None
        }
    }
}

// -------------------------------------------------------------------------------------------------

pub(in crate::types::indexed) fn create_list<C: IndexCore<Ref>>(
    core: &C,
    index: Index<IndexedListType>,
) -> IndexedValueList<'_, Ref, C> {
    const UNRESERVED_BITS: usize = usize::BITS as usize - 1;
    const BITMASK: usize = (1 << UNRESERVED_BITS) - 1;

    // Extract the flag from the highest bit
    let range_or_byte_index = index.value() >> UNRESERVED_BITS;
    let index = Index::<IndexedListType>::new(index.value() & BITMASK);

    if range_or_byte_index == 0 {
        // No flag, determine from byte index
        let tag = unsafe { *core.root().get_unchecked(index.value()) };

        unsafe {
            match tag {
                0 => IndexedValueList::Empty,
                1 => IndexedValueList::Byte(IndexedList::<_, _, C>::new(core, index.cast())),
                2 => IndexedValueList::Short(IndexedList::<_, _, C>::new(core, index.cast())),
                3 => IndexedValueList::Int(IndexedList::<_, _, C>::new(core, index.cast())),
                4 => IndexedValueList::Long(IndexedList::<_, _, C>::new(core, index.cast())),
                5 => IndexedValueList::Float(IndexedList::<_, _, C>::new(core, index.cast())),
                6 => IndexedValueList::Double(IndexedList::<_, _, C>::new(core, index.cast())),
                #[cfg(debug_assertions)]
                _ => unreachable!("Invalid byte-index tag \"{tag}\"!"),
                #[cfg(not(debug_assertions))]
                _ => core::hint::unreachable_unchecked(),
            }
        }
    } else {
        // Flag, determine from entries in range
        let entries = unsafe { core.entry_range(index.value()) };
        let Some(first) = entries.first() else { return IndexedValueList::Empty };

        unsafe {
            match first.value() {
                ValueIndex::ByteArray(..) => {
                    IndexedValueList::ByteArray(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                ValueIndex::String(..) => {
                    IndexedValueList::String(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                ValueIndex::List(..) => {
                    IndexedValueList::List(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                ValueIndex::Compound(..) => {
                    IndexedValueList::Compound(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                ValueIndex::IntArray(..) => {
                    IndexedValueList::IntArray(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                ValueIndex::LongArray(..) => {
                    IndexedValueList::LongArray(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                #[cfg(debug_assertions)]
                _ => unreachable!("Invalid range-entry tag \"{:?}\"!", first.value()),
                #[cfg(not(debug_assertions))]
                _ => core::hint::unreachable_unchecked(),
            }
        }
    }
}

pub(in crate::types::indexed) fn create_list_mut<C: IndexCore<Mut>>(
    core: &mut C,
    index: Index<IndexedListType>,
) -> IndexedValueList<'_, Mut, C> {
    const UNRESERVED_BITS: usize = usize::BITS as usize - 1;
    const BITMASK: usize = (1 << UNRESERVED_BITS) - 1;

    // Extract the flag from the highest bit
    let range_or_byte_index = index.value() >> UNRESERVED_BITS;
    let index = Index::<IndexedListType>::new(index.value() & BITMASK);

    if range_or_byte_index == 0 {
        // No flag, determine from byte index
        let tag = unsafe { *core.root().get_unchecked(index.value()) };

        unsafe {
            match tag {
                0 => IndexedValueList::Empty,
                1 => IndexedValueList::Byte(IndexedList::<_, _, C>::new(core, index.cast())),
                2 => IndexedValueList::Short(IndexedList::<_, _, C>::new(core, index.cast())),
                3 => IndexedValueList::Int(IndexedList::<_, _, C>::new(core, index.cast())),
                4 => IndexedValueList::Long(IndexedList::<_, _, C>::new(core, index.cast())),
                5 => IndexedValueList::Float(IndexedList::<_, _, C>::new(core, index.cast())),
                6 => IndexedValueList::Double(IndexedList::<_, _, C>::new(core, index.cast())),
                #[cfg(debug_assertions)]
                _ => unreachable!("Invalid byte-index tag \"{tag}\"!"),
                #[cfg(not(debug_assertions))]
                _ => core::hint::unreachable_unchecked(),
            }
        }
    } else {
        // Flag, determine from entries in range
        let entries = unsafe { core.entry_range(index.value()) };
        let Some(first) = entries.first() else { return IndexedValueList::Empty };

        unsafe {
            match first.value() {
                ValueIndex::ByteArray(..) => {
                    IndexedValueList::ByteArray(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                ValueIndex::String(..) => {
                    IndexedValueList::String(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                ValueIndex::List(..) => {
                    IndexedValueList::List(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                ValueIndex::Compound(..) => {
                    IndexedValueList::Compound(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                ValueIndex::IntArray(..) => {
                    IndexedValueList::IntArray(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                ValueIndex::LongArray(..) => {
                    IndexedValueList::LongArray(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                #[cfg(debug_assertions)]
                _ => unreachable!("Invalid range-entry tag \"{:?}\"!", first.value()),
                #[cfg(not(debug_assertions))]
                _ => core::hint::unreachable_unchecked(),
            }
        }
    }
}
