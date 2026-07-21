use core::fmt;

use crate::{
    prelude::IndexedNbt,
    types::indexed::{
        compound::IndexedCompound,
        core::{IndexCore, NbtAccess, Ref},
        entry::IndexedValue,
        list::{IndexedList, IndexedValueList},
        reference::{IndexableValue, IndexedReference, IndexedValueReference},
    },
};

impl<C: IndexCore<Ref>> fmt::Debug for IndexedNbt<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IndexedNbt").field("content", &self.as_compound()).finish()
    }
}

impl<A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> fmt::Debug for IndexedCompound<'_, A, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_map();
        for entry in self {
            debug.entry(&entry.name(), &entry.value());
        }
        debug.finish()
    }
}

// -------------------------------------------------------------------------------------------------

impl<A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> fmt::Debug for IndexedValueList<'_, A, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IndexedValueList::Empty => f.debug_tuple("Empty").finish(),
            IndexedValueList::Byte(list) => f.debug_tuple("Byte").field(&list).finish(),
            IndexedValueList::Short(list) => f.debug_tuple("Short").field(&list).finish(),
            IndexedValueList::Int(list) => f.debug_tuple("Int").field(&list).finish(),
            IndexedValueList::Long(list) => f.debug_tuple("Long").field(&list).finish(),
            IndexedValueList::Float(list) => f.debug_tuple("Float").field(&list).finish(),
            IndexedValueList::Double(list) => f.debug_tuple("Double").field(&list).finish(),
            IndexedValueList::ByteArray(list) => f.debug_tuple("ByteArray").field(&list).finish(),
            IndexedValueList::String(list) => f.debug_tuple("String").field(&list).finish(),
            IndexedValueList::List(list) => f.debug_tuple("List").field(&list).finish(),
            IndexedValueList::Compound(list) => f.debug_tuple("Compound").field(&list).finish(),
            IndexedValueList::IntArray(list) => f.debug_tuple("IntArray").field(&list).finish(),
            IndexedValueList::LongArray(list) => f.debug_tuple("LongArray").field(&list).finish(),
        }
    }
}

impl<T: ?Sized, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> fmt::Debug
    for IndexedList<'_, T, A, C>
where
    for<'a> &'a Self: IntoIterator,
    for<'a> <&'a Self as IntoIterator>::Item: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

// -------------------------------------------------------------------------------------------------

impl<A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> fmt::Debug for IndexedValue<'_, A, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.as_value() {
            IndexedValueReference::Byte(val) => f.debug_tuple("Byte").field(&val).finish(),
            IndexedValueReference::Short(val) => f.debug_tuple("Short").field(&val).finish(),
            IndexedValueReference::Int(val) => f.debug_tuple("Int").field(&val).finish(),
            IndexedValueReference::Long(val) => f.debug_tuple("Long").field(&val).finish(),
            IndexedValueReference::Float(val) => f.debug_tuple("Float").field(&val).finish(),
            IndexedValueReference::Double(val) => f.debug_tuple("Double").field(&val).finish(),
            IndexedValueReference::ByteArray(val) => {
                f.debug_tuple("ByteArray").field(&val).finish()
            }
            IndexedValueReference::String(val) => f.debug_tuple("String").field(&val).finish(),
            IndexedValueReference::List(val) => f.debug_tuple("List").field(&val).finish(),
            IndexedValueReference::Compound(val) => f.debug_tuple("Compound").field(&val).finish(),
            IndexedValueReference::IntArray(val) => f.debug_tuple("IntArray").field(&val).finish(),
            IndexedValueReference::LongArray(val) => {
                f.debug_tuple("LongArray").field(&val).finish()
            }
        }
    }
}

impl<T: IndexableValue + ?Sized, A: NbtAccess> fmt::Debug for IndexedReference<'_, T, A>
where
    for<'a> T::Value<'a>: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(&self.get(), f) }
}
