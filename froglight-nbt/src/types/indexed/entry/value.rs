use froglight_mutf8::prelude::MStr;

use crate::types::indexed::{
    compound::IndexedCompound,
    core::{IndexCore, Mut, NbtAccess, Ref},
    entry::IndexedValue,
    index::{Index, ValueIndex},
    list::{IndexedList, ValueList},
    reference::{IndexableValue, IndexedReference, ValueReference},
    types::IndexedListType,
};

impl<'data, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A> + 'data> IndexedValue<'data, A, C> {
    /// Return a reference to the stored value.
    #[must_use]
    pub fn as_value(&self) -> ValueReference<'_, Ref, C> {
        let root = <C as IndexCore<A>>::root(&self.core);
        match self.index {
            ValueIndex::Byte(index) => {
                ValueReference::Byte(unsafe { IndexedReference::<u8, Ref>::new(root, index) })
            }
            ValueIndex::Short(index) => {
                ValueReference::Short(unsafe { IndexedReference::<u16, Ref>::new(root, index) })
            }
            ValueIndex::Int(index) => {
                ValueReference::Int(unsafe { IndexedReference::<u32, Ref>::new(root, index) })
            }
            ValueIndex::Long(index) => {
                ValueReference::Long(unsafe { IndexedReference::<u64, Ref>::new(root, index) })
            }
            ValueIndex::Float(index) => {
                ValueReference::Float(unsafe { IndexedReference::<f32, Ref>::new(root, index) })
            }
            ValueIndex::Double(index) => {
                ValueReference::Double(unsafe { IndexedReference::<f64, Ref>::new(root, index) })
            }
            ValueIndex::ByteArray(index) => ValueReference::ByteArray(unsafe {
                IndexedReference::<[u8], Ref>::new(root, index)
            }),
            ValueIndex::String(index) => {
                ValueReference::String(unsafe { IndexedReference::<MStr, Ref>::new(root, index) })
            }
            ValueIndex::IntArray(index) => ValueReference::IntArray(unsafe {
                IndexedReference::<[u32], Ref>::new(root, index)
            }),
            ValueIndex::LongArray(index) => ValueReference::LongArray(unsafe {
                IndexedReference::<[u64], Ref>::new(root, index)
            }),

            ValueIndex::List(index) => {
                ValueReference::List(create_list::<C, Ref>(&self.core, index))
            }
            ValueIndex::Compound(index) => ValueReference::Compound(unsafe {
                IndexedCompound::<Ref, C>::new(&self.core, index.value())
            }),
        }
    }

    /// Return the stored value.
    #[must_use]
    pub fn into_value(self) -> ValueReference<'data, Ref, C> {
        match self.index {
            ValueIndex::Byte(index) => ValueReference::Byte(unsafe {
                let root = <C as IndexCore<A>>::root(A::into_core(self.core));
                IndexedReference::<u8, Ref>::new(root, index)
            }),
            ValueIndex::Short(index) => ValueReference::Short(unsafe {
                let root = <C as IndexCore<Ref>>::root(A::into_core(self.core));
                IndexedReference::<u16, Ref>::new(root, index)
            }),
            ValueIndex::Int(index) => ValueReference::Int(unsafe {
                let root = <C as IndexCore<Ref>>::root(A::into_core(self.core));
                IndexedReference::<u32, Ref>::new(root, index)
            }),
            ValueIndex::Long(index) => ValueReference::Long(unsafe {
                let root = <C as IndexCore<Ref>>::root(A::into_core(self.core));
                IndexedReference::<u64, Ref>::new(root, index)
            }),
            ValueIndex::Float(index) => ValueReference::Float(unsafe {
                let root = <C as IndexCore<Ref>>::root(A::into_core(self.core));
                IndexedReference::<f32, Ref>::new(root, index)
            }),
            ValueIndex::Double(index) => ValueReference::Double(unsafe {
                let root = <C as IndexCore<Ref>>::root(A::into_core(self.core));
                IndexedReference::<f64, Ref>::new(root, index)
            }),
            ValueIndex::ByteArray(index) => ValueReference::ByteArray(unsafe {
                let root = <C as IndexCore<Ref>>::root(A::into_core(self.core));
                IndexedReference::<[u8], Ref>::new(root, index)
            }),
            ValueIndex::String(index) => ValueReference::String(unsafe {
                let root = <C as IndexCore<Ref>>::root(A::into_core(self.core));
                IndexedReference::<MStr, Ref>::new(root, index)
            }),
            ValueIndex::IntArray(index) => ValueReference::IntArray(unsafe {
                let root = <C as IndexCore<Ref>>::root(A::into_core(self.core));
                IndexedReference::<[u32], Ref>::new(root, index)
            }),
            ValueIndex::LongArray(index) => ValueReference::LongArray(unsafe {
                let root = <C as IndexCore<Ref>>::root(A::into_core(self.core));
                IndexedReference::<[u64], Ref>::new(root, index)
            }),
            ValueIndex::List(index) => {
                let core = A::into_core(self.core);
                ValueReference::List(create_list::<C, Ref>(core, index))
            }
            ValueIndex::Compound(index) => ValueReference::Compound(unsafe {
                let core = A::into_core(self.core);
                IndexedCompound::<Ref, C>::new(core, index.value())
            }),
        }
    }
}

impl<'data, C: IndexCore<Mut> + 'data> IndexedValue<'data, Mut, C> {
    /// Return a reference to the stored value.
    #[must_use]
    pub fn as_value_mut(&mut self) -> ValueReference<'_, Mut, C> {
        match self.index {
            ValueIndex::Byte(index) => ValueReference::Byte(unsafe {
                let root = <C as IndexCore<Mut>>::root_mut(self.core);
                IndexedReference::<u8, Mut>::new(root, index)
            }),
            ValueIndex::Short(index) => ValueReference::Short(unsafe {
                let root = <C as IndexCore<Mut>>::root_mut(self.core);
                IndexedReference::<u16, Mut>::new(root, index)
            }),
            ValueIndex::Int(index) => ValueReference::Int(unsafe {
                let root = <C as IndexCore<Mut>>::root_mut(self.core);
                IndexedReference::<u32, Mut>::new(root, index)
            }),
            ValueIndex::Long(index) => ValueReference::Long(unsafe {
                let root = <C as IndexCore<Mut>>::root_mut(self.core);
                IndexedReference::<u64, Mut>::new(root, index)
            }),
            ValueIndex::Float(index) => ValueReference::Float(unsafe {
                let root = <C as IndexCore<Mut>>::root_mut(self.core);
                IndexedReference::<f32, Mut>::new(root, index)
            }),
            ValueIndex::Double(index) => ValueReference::Double(unsafe {
                let root = <C as IndexCore<Mut>>::root_mut(self.core);
                IndexedReference::<f64, Mut>::new(root, index)
            }),
            ValueIndex::ByteArray(index) => ValueReference::ByteArray(unsafe {
                let root = <C as IndexCore<Mut>>::root_mut(self.core);
                IndexedReference::<[u8], Mut>::new(root, index)
            }),
            ValueIndex::String(index) => ValueReference::String(unsafe {
                let root = <C as IndexCore<Mut>>::root_mut(self.core);
                IndexedReference::<MStr, Mut>::new(root, index)
            }),
            ValueIndex::IntArray(index) => ValueReference::IntArray(unsafe {
                let root = <C as IndexCore<Mut>>::root_mut(self.core);
                IndexedReference::<[u32], Mut>::new(root, index)
            }),
            ValueIndex::LongArray(index) => ValueReference::LongArray(unsafe {
                let root = <C as IndexCore<Mut>>::root_mut(self.core);
                IndexedReference::<[u64], Mut>::new(root, index)
            }),

            ValueIndex::List(index) => ValueReference::List(create_list_mut(self.core, index)),
            ValueIndex::Compound(index) => ValueReference::Compound(unsafe {
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
                            Some(unsafe { IndexedReference::<$ty, Ref>::new(root, value) }.get())
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
    /// [`IndexedCompound`], else `None`.
    #[must_use]
    pub fn as_compound(&self) -> Option<IndexedCompound<'_, Ref, C>> {
        if let ValueIndex::Compound(value) = self.index {
            Some(unsafe { IndexedCompound::<Ref, C>::new(&self.core, value.value()) })
        } else {
            None
        }
    }

    /// Return the stored value if it is of type [`IndexedCompound`], else
    /// `None`.
    #[must_use]
    pub fn into_compound(self) -> Option<IndexedCompound<'data, A, C>> {
        if let ValueIndex::Compound(value) = self.index {
            Some(unsafe { IndexedCompound::<A, C>::new(self.core, value.value()) })
        } else {
            None
        }
    }

    /// Return a reference to the stored value if it is of type
    /// [`IndexedValueList`], else `None`.
    #[must_use]
    pub fn as_list(&self) -> Option<ValueList<'_, Ref, C>> {
        if let ValueIndex::List(index) = self.index {
            Some(create_list::<C, Ref>(&self.core, index))
        } else {
            None
        }
    }

    /// Return the stored value if it is of type [`IndexedValueList`], else
    /// `None`.
    #[must_use]
    pub fn into_list(self) -> Option<ValueList<'data, A, C>> {
        if let ValueIndex::List(index) = self.index {
            Some(create_list::<C, A>(self.core, index))
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
    pub fn as_list_mut(&mut self) -> Option<ValueList<'_, Mut, C>> {
        if let ValueIndex::List(index) = self.index {
            Some(create_list_mut(self.core, index))
        } else {
            None
        }
    }
}

// -------------------------------------------------------------------------------------------------

pub(in crate::types::indexed) fn create_list<C: IndexCore<A>, A: NbtAccess>(
    core: A::CORE<'_, C>,
    index: Index<IndexedListType>,
) -> ValueList<'_, A, C> {
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
                0 => ValueList::Empty,
                1 => ValueList::Byte(IndexedList::<_, _, C>::new(core, index.cast())),
                2 => ValueList::Short(IndexedList::<_, _, C>::new(core, index.cast())),
                3 => ValueList::Int(IndexedList::<_, _, C>::new(core, index.cast())),
                4 => ValueList::Long(IndexedList::<_, _, C>::new(core, index.cast())),
                5 => ValueList::Float(IndexedList::<_, _, C>::new(core, index.cast())),
                6 => ValueList::Double(IndexedList::<_, _, C>::new(core, index.cast())),
                #[cfg(debug_assertions)]
                _ => unreachable!("Invalid byte-index tag \"{tag}\"!"),
                #[cfg(not(debug_assertions))]
                _ => core::hint::unreachable_unchecked(),
            }
        }
    } else {
        // Flag, determine from entries in range
        let entries = unsafe { core.entry_range(index.value()) };
        let Some(first) = entries.first() else { return ValueList::Empty };

        unsafe {
            match first.value() {
                ValueIndex::ByteArray(..) => {
                    ValueList::ByteArray(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                ValueIndex::String(..) => {
                    ValueList::String(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                ValueIndex::List(..) => {
                    ValueList::List(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                ValueIndex::Compound(..) => {
                    ValueList::Compound(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                ValueIndex::IntArray(..) => {
                    ValueList::IntArray(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                ValueIndex::LongArray(..) => {
                    ValueList::LongArray(IndexedList::<_, _, C>::new(core, index.cast()))
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
) -> ValueList<'_, Mut, C> {
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
                0 => ValueList::Empty,
                1 => ValueList::Byte(IndexedList::<_, _, C>::new(core, index.cast())),
                2 => ValueList::Short(IndexedList::<_, _, C>::new(core, index.cast())),
                3 => ValueList::Int(IndexedList::<_, _, C>::new(core, index.cast())),
                4 => ValueList::Long(IndexedList::<_, _, C>::new(core, index.cast())),
                5 => ValueList::Float(IndexedList::<_, _, C>::new(core, index.cast())),
                6 => ValueList::Double(IndexedList::<_, _, C>::new(core, index.cast())),
                #[cfg(debug_assertions)]
                _ => unreachable!("Invalid byte-index tag \"{tag}\"!"),
                #[cfg(not(debug_assertions))]
                _ => core::hint::unreachable_unchecked(),
            }
        }
    } else {
        // Flag, determine from entries in range
        let entries = unsafe { core.entry_range(index.value()) };
        let Some(first) = entries.first() else { return ValueList::Empty };

        unsafe {
            match first.value() {
                ValueIndex::ByteArray(..) => {
                    ValueList::ByteArray(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                ValueIndex::String(..) => {
                    ValueList::String(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                ValueIndex::List(..) => {
                    ValueList::List(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                ValueIndex::Compound(..) => {
                    ValueList::Compound(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                ValueIndex::IntArray(..) => {
                    ValueList::IntArray(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                ValueIndex::LongArray(..) => {
                    ValueList::LongArray(IndexedList::<_, _, C>::new(core, index.cast()))
                }
                #[cfg(debug_assertions)]
                _ => unreachable!("Invalid range-entry tag \"{:?}\"!", first.value()),
                #[cfg(not(debug_assertions))]
                _ => core::hint::unreachable_unchecked(),
            }
        }
    }
}
