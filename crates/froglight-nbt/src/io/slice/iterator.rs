use super::{NbtCompoundRef, NbtTagRef};
use crate::{mutf8::Mutf8Str, nbt::NbtCompound};

/// An iterator over the tags in an [`NbtCompoundRef`].
pub struct NbtRefIterator<'a>(NbtCompoundRef<'a>);

impl<'a> Iterator for NbtRefIterator<'a> {
    type Item = (&'a Mutf8Str, NbtTagRef<'a>);

    fn next(&mut self) -> Option<Self::Item> { self.0.next_tag() }
}

// -------------------------------------------------------------------------------------------------

impl<'a> NbtCompoundRef<'a> {
    /// Create an iterator over the tags in the [`NbtCompoundRef`].
    #[inline]
    #[must_use]
    pub fn iter(&self) -> NbtRefIterator<'a> { self.into_iter() }
}
impl<'a> IntoIterator for &NbtCompoundRef<'a> {
    type IntoIter = NbtRefIterator<'a>;
    type Item = (&'a Mutf8Str, NbtTagRef<'a>);

    fn into_iter(self) -> Self::IntoIter { NbtRefIterator(*self) }
}

impl<'a> FromIterator<(&'a Mutf8Str, NbtTagRef<'a>)> for NbtCompound {
    fn from_iter<T: IntoIterator<Item = (&'a Mutf8Str, NbtTagRef<'a>)>>(iter: T) -> Self {
        iter.into_iter().fold(NbtCompound::new(), |mut c, (k, v)| {
            c.insert(k.to_mutf8_string(), v.as_owned());
            c
        })
    }
}
