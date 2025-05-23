#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::iter::{Extend, IntoIterator, Iterator};

#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use derive_more::{From, Into};

use super::CompoundMap;
use crate::{convert::IntoCompound, mutf8::Mutf8String, nbt::NbtTag};

/// A map of named NBT tags.
#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, From, Into)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(opaque, Debug, Default, Clone, PartialEq))]
pub struct NbtCompound(CompoundMap);

impl NbtCompound {
    /// Create a new empty [`NbtCompound`].
    #[inline]
    #[must_use]
    #[expect(clippy::default_trait_access)]
    pub fn new() -> Self { Self(CompoundMap::with_hasher(Default::default())) }

    /// Return the number of tags in the [`NbtCompound`].
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize { self.0.len() }

    /// Return `true` if the [`NbtCompound`] contains no tags.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Return `true` if the [`NbtCompound`] contains a tag with the given key.
    #[inline]
    #[must_use]
    pub fn contains_key<Q: AsRef<str>>(&self, key: Q) -> bool {
        self.contains_key_bytes(key.as_ref().as_bytes())
    }

    /// Return `true` if the [`NbtCompound`] contains a tag with the given key's
    /// bytes.
    #[inline]
    #[must_use]
    pub fn contains_key_bytes<Q: AsRef<[u8]>>(&self, key: Q) -> bool {
        self.0.contains_key(key.as_ref())
    }

    /// Get a reference to a [`NbtTag`] by it's key.
    #[inline]
    #[must_use]
    pub fn get_tag<Q: AsRef<str>>(&self, key: Q) -> Option<&NbtTag> {
        self.get_tag_bytes(key.as_ref().as_bytes())
    }

    /// Get a reference to a [`NbtTag`] by it's key's bytes.
    #[inline]
    #[must_use]
    pub fn get_tag_bytes<Q: AsRef<[u8]>>(&self, key: Q) -> Option<&NbtTag> {
        self.0.get(key.as_ref())
    }

    /// Get a mutable reference to a [`NbtTag`] by it's key.
    #[inline]
    #[must_use]
    pub fn get_tag_mut<Q: AsRef<str>>(&mut self, key: Q) -> Option<&mut NbtTag> {
        self.get_tag_bytes_mut(key.as_ref().as_bytes())
    }

    /// Get a mutable reference to a [`NbtTag`] by it's key's bytes.
    #[inline]
    #[must_use]
    pub fn get_tag_bytes_mut<Q: AsRef<[u8]>>(&mut self, key: Q) -> Option<&mut NbtTag> {
        self.0.get_mut(key.as_ref())
    }

    /// Get a reference to a [`NbtTag`] by it's index.
    #[inline]
    #[must_use]
    pub fn get_index(&self, index: usize) -> Option<(&Mutf8String, &NbtTag)> {
        self.0.get_index(index)
    }

    /// Get a mutable reference to a [`NbtTag`] by it's index.
    #[inline]
    #[must_use]
    pub fn get_index_mut(&mut self, index: usize) -> Option<(&Mutf8String, &mut NbtTag)> {
        self.0.get_index_mut(index)
    }

    /// Get the key's entry in the [`NbtCompound`] for
    /// insertion or in-place manipulation.
    #[inline]
    #[must_use]
    pub fn entry(
        &mut self,
        key: impl Into<Mutf8String>,
    ) -> indexmap::map::Entry<Mutf8String, NbtTag> {
        self.0.entry(key.into())
    }

    /// Get the index's entry in the [`NbtCompound`] for
    /// in-place manipulation.
    ///
    /// Returns `None` if the index is out of bounds.
    #[inline]
    #[must_use]
    pub fn entry_index(
        &mut self,
        index: usize,
    ) -> Option<indexmap::map::IndexedEntry<Mutf8String, NbtTag>> {
        self.0.get_index_entry(index)
    }

    /// Insert a new key-value pair into the [`NbtCompound`].
    ///
    /// Returns the previous value associated with the key, if any.
    #[inline]
    pub fn insert(
        &mut self,
        key: impl Into<Mutf8String>,
        tag: impl Into<NbtTag>,
    ) -> Option<NbtTag> {
        self.0.insert(key.into(), tag.into())
    }

    /// Insert a new key-value pair into the [`NbtCompound`] at the given index,
    /// shifting all other elements to the right.
    ///
    /// Returns the previous value associated with the key, if any.
    ///
    /// See [`IndexMap::shift_insert`](indexmap::IndexMap::shift_insert) for
    /// more information.
    #[inline]
    pub fn shift_insert(
        &mut self,
        index: usize,
        key: impl Into<Mutf8String>,
        tag: NbtTag,
    ) -> Option<NbtTag> {
        self.0.shift_insert(index, key.into(), tag)
    }

    /// Remove a key-value pair from the [`NbtCompound`] by it's key,
    /// shifting all other elements to the left.
    ///
    /// Returns the removed value, if any.
    ///
    /// See [`IndexMap::shift_remove`](indexmap::IndexMap::shift_remove) for
    /// more information.
    #[inline]
    pub fn shift_remove<Q: ?Sized + AsRef<str>>(&mut self, key: &Q) -> Option<NbtTag> {
        self.0.shift_remove(key.as_ref().as_bytes())
    }

    /// Remove a key-value pair from the [`NbtCompound`] by it's index,
    /// swapping the last element into it's place.
    ///
    /// Returns the removed value, if any.
    ///
    /// See [`IndexMap::swap_remove`](indexmap::IndexMap::swap_remove) for more
    /// information.
    #[inline]
    pub fn swap_remove<Q: ?Sized + AsRef<str>>(&mut self, key: &Q) -> Option<NbtTag> {
        self.0.swap_remove(key.as_ref().as_bytes())
    }

    /// Return an iterator over the name-tag pairs of the compound,
    /// in their order
    #[inline]
    #[must_use]
    pub fn iter(&self) -> indexmap::map::Iter<Mutf8String, NbtTag> { self.0.iter() }

    /// Return an iterator over the key-value pairs of the map,
    /// in their order
    #[inline]
    #[must_use]
    pub fn iter_mut(&mut self) -> indexmap::map::IterMut<Mutf8String, NbtTag> { self.0.iter_mut() }
}

impl IntoIterator for NbtCompound {
    type IntoIter = indexmap::map::IntoIter<Mutf8String, NbtTag>;
    type Item = (Mutf8String, NbtTag);

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}
impl<'a> IntoIterator for &'a NbtCompound {
    type IntoIter = indexmap::map::Iter<'a, Mutf8String, NbtTag>;
    type Item = (&'a Mutf8String, &'a NbtTag);

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl<'a> IntoIterator for &'a mut NbtCompound {
    type IntoIter = indexmap::map::IterMut<'a, Mutf8String, NbtTag>;
    type Item = (&'a Mutf8String, &'a mut NbtTag);

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.iter_mut() }
}

impl<A: Into<Mutf8String>, B: Into<NbtTag>> FromIterator<(A, B)> for NbtCompound {
    fn from_iter<T: IntoIterator<Item = (A, B)>>(iter: T) -> Self {
        Self(iter.into_iter().map(|(key, val)| (key.into(), val.into())).collect::<CompoundMap>())
    }
}
impl From<Vec<(Mutf8String, NbtTag)>> for NbtCompound {
    #[inline]
    fn from(pair: Vec<(Mutf8String, NbtTag)>) -> Self { Self::from_iter(pair) }
}

impl<A: Into<Mutf8String>, B: Into<NbtTag>> Extend<(A, B)> for NbtCompound {
    fn extend<T: IntoIterator<Item = (A, B)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(key, val)| {
            self.insert(key, val.into());
        });
    }
}
impl<A: IntoCompound> Extend<A> for NbtCompound {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        iter.into_iter().for_each(|item| match item.into_compound() {
            Ok(c) => self.extend(c),
            Err(err) => {
                panic!("Failed to extend NbtCompound with {}: {err}", core::any::type_name::<A>())
            }
        });
    }
}

impl<'a> core::ops::Index<&'a str> for NbtCompound {
    type Output = NbtTag;

    fn index(&self, key: &'a str) -> &Self::Output {
        self.get_tag(key).expect("NbtCompound does not contain key")
    }
}
impl<'a> core::ops::IndexMut<&'a str> for NbtCompound {
    fn index_mut(&mut self, key: &'a str) -> &mut Self::Output {
        self.get_tag_mut(key).expect("NbtCompound does not contain key")
    }
}

impl core::ops::Index<usize> for NbtCompound {
    type Output = NbtTag;

    fn index(&self, index: usize) -> &Self::Output {
        self.get_index(index).map(|(_, tag)| tag).expect("NbtCompound does not contain index")
    }
}
impl core::ops::IndexMut<usize> for NbtCompound {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_index_mut(index).map(|(_, tag)| tag).expect("NbtCompound does not contain index")
    }
}
