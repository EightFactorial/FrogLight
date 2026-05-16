use froglight_mutf8::prelude::MStr;

use crate::types::indexed::{
    compound::IndexedCompound,
    core::{IndexCore, Mut, NbtAccess, Ref},
    entry::{IndexedListType, IndexedMapType, IndexedValue},
    index::{Index, ValueIndex},
    list::{IndexedList, IndexedValueList},
    reference::{IndexedReference, IndexedValueReference},
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
                IndexedList::<[u8], Ref, C>::new(&self.core, index)
            }),
            ValueIndex::String(index) => IndexedValueReference::String(unsafe {
                IndexedReference::<MStr, Ref>::new(root, index)
            }),
            ValueIndex::IntArray(index) => IndexedValueReference::IntArray(unsafe {
                IndexedList::<[u32], Ref, C>::new(&self.core, index)
            }),
            ValueIndex::LongArray(index) => IndexedValueReference::LongArray(unsafe {
                IndexedList::<[u64], Ref, C>::new(&self.core, index)
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
                IndexedList::<[u8], Mut, C>::new(self.core, index)
            }),
            ValueIndex::String(index) => IndexedValueReference::String(unsafe {
                let root = <C as IndexCore<Mut>>::root_mut(self.core);
                IndexedReference::<MStr, Mut>::new(root, index)
            }),
            ValueIndex::IntArray(index) => IndexedValueReference::IntArray(unsafe {
                IndexedList::<[u32], Mut, C>::new(self.core, index)
            }),
            ValueIndex::LongArray(index) => IndexedValueReference::LongArray(unsafe {
                IndexedList::<[u64], Mut, C>::new(self.core, index)
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
    (@ref $($ident:ident: $ty:ty => $variant:ident),*) => {
        impl<'data, A: NbtAccess, C: IndexCore<A> + 'data> IndexedValue<'data, A, C> {
            $(
                #[must_use]
                #[doc = concat!("Return a reference to the stored value if it is of type [`", stringify!($ty), "`], else `None`.")]
                pub fn $ident(&self) -> Option<IndexedReference<'_, $ty, Ref>> {
                    if let ValueIndex::$variant(value) = self.index {
                        let root = <C as IndexCore<A>>::root(&self.core);
                        Some(unsafe { IndexedReference::<$ty, Ref>::new(root, value) })
                    } else {
                        None
                    }
                }
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
                        let root = <C as IndexCore<Mut>>::root_mut(&mut self.core);
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
    as_byte: u8 => Byte,
    as_short: u16 => Short,
    as_int: u32 => Int,
    as_long: u64 => Long,
    as_float: f32 => Float,
    as_double: f64 => Double,
    as_byte_array: [u8] => ByteArray,
    as_string: MStr => String,
    as_int_array: [u32] => IntArray,
    as_long_array: [u64] => LongArray
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
    pub fn as_compound(&self) -> Option<IndexedCompound<'_, Ref, C>> {
        if let ValueIndex::Compound(value) = self.index {
            Some(unsafe { IndexedCompound::<Ref, C>::new(&self.core, value.value()) })
        } else {
            None
        }
    }

    /// Return a reference to the stored value if it is of type
    /// [`IndexedValueList`], else else `None`.
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
            Some(unsafe { IndexedCompound::<Mut, C>::new(&mut self.core, value.value()) })
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
    // SAFETY: The `index` for lists is an `entry_range` index.
    let entries = unsafe { <C as IndexCore<Ref>>::entry_range(core, index.value()) };
    let Some(first) = entries.first() else { return IndexedValueList::Empty };

    // SAFETY: Every value in a list is guaranteed to be the same type.
    unsafe {
        match first.value() {
            ValueIndex::Byte(..) => {
                IndexedValueList::Byte(IndexedList::<u8, Ref, C>::new(core, index.cast()))
            }
            ValueIndex::Short(..) => {
                IndexedValueList::Short(IndexedList::<u16, Ref, C>::new(core, index.cast()))
            }
            ValueIndex::Int(..) => {
                IndexedValueList::Int(IndexedList::<u32, Ref, C>::new(core, index.cast()))
            }
            ValueIndex::Long(..) => {
                IndexedValueList::Long(IndexedList::<u64, Ref, C>::new(core, index.cast()))
            }
            ValueIndex::Float(..) => {
                IndexedValueList::Float(IndexedList::<f32, Ref, C>::new(core, index.cast()))
            }
            ValueIndex::Double(..) => {
                IndexedValueList::Double(IndexedList::<f64, Ref, C>::new(core, index.cast()))
            }

            ValueIndex::ByteArray(..) => {
                IndexedValueList::ByteArray(IndexedList::<[u8], Ref, C>::new(core, index.cast()))
            }
            ValueIndex::String(..) => {
                IndexedValueList::String(IndexedList::<MStr, Ref, C>::new(core, index.cast()))
            }
            ValueIndex::List(..) => IndexedValueList::List(
                IndexedList::<IndexedListType, Ref, C>::new(core, index.cast()),
            ),
            ValueIndex::Compound(..) => {
                IndexedValueList::Compound(IndexedList::<IndexedMapType, Ref, C>::new(
                    core,
                    index.cast(),
                ))
            }
            ValueIndex::IntArray(..) => {
                IndexedValueList::IntArray(IndexedList::<[u32], Ref, C>::new(core, index.cast()))
            }
            ValueIndex::LongArray(..) => {
                IndexedValueList::LongArray(IndexedList::<[u64], Ref, C>::new(core, index.cast()))
            }
        }
    }
}

pub(in crate::types::indexed) fn create_list_mut<C: IndexCore<Mut>>(
    core: &mut C,
    index: Index<IndexedListType>,
) -> IndexedValueList<'_, Mut, C> {
    // SAFETY: The `index` for lists is an `entry_range` index.
    let entries = unsafe { <C as IndexCore<Mut>>::entry_range(core, index.value()) };
    let Some(first) = entries.first() else { return IndexedValueList::Empty };

    // SAFETY: Every value in a list is guaranteed to be the same type.
    unsafe {
        match first.value() {
            ValueIndex::Byte(..) => {
                IndexedValueList::Byte(IndexedList::<u8, Mut, C>::new(core, index.cast()))
            }
            ValueIndex::Short(..) => {
                IndexedValueList::Short(IndexedList::<u16, Mut, C>::new(core, index.cast()))
            }
            ValueIndex::Int(..) => {
                IndexedValueList::Int(IndexedList::<u32, Mut, C>::new(core, index.cast()))
            }
            ValueIndex::Long(..) => {
                IndexedValueList::Long(IndexedList::<u64, Mut, C>::new(core, index.cast()))
            }
            ValueIndex::Float(..) => {
                IndexedValueList::Float(IndexedList::<f32, Mut, C>::new(core, index.cast()))
            }
            ValueIndex::Double(..) => {
                IndexedValueList::Double(IndexedList::<f64, Mut, C>::new(core, index.cast()))
            }
            ValueIndex::ByteArray(..) => {
                IndexedValueList::ByteArray(IndexedList::<[u8], Mut, C>::new(core, index.cast()))
            }
            ValueIndex::String(..) => {
                IndexedValueList::String(IndexedList::<MStr, Mut, C>::new(core, index.cast()))
            }
            ValueIndex::List(..) => IndexedValueList::List(
                IndexedList::<IndexedListType, Mut, C>::new(core, index.cast()),
            ),
            ValueIndex::Compound(..) => {
                IndexedValueList::Compound(IndexedList::<IndexedMapType, Mut, C>::new(
                    core,
                    index.cast(),
                ))
            }
            ValueIndex::IntArray(..) => {
                IndexedValueList::IntArray(IndexedList::<[u32], Mut, C>::new(core, index.cast()))
            }
            ValueIndex::LongArray(..) => {
                IndexedValueList::LongArray(IndexedList::<[u64], Mut, C>::new(core, index.cast()))
            }
        }
    }
}
