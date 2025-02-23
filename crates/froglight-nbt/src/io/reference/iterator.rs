use super::{NbtCompoundRef, NbtTagRef};
use crate::mutf8::Mutf8Str;

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
    type Item = (&'a Mutf8Str, NbtTagRef<'a>);
    type IntoIter = NbtRefIterator<'a>;
    fn into_iter(self) -> Self::IntoIter { NbtRefIterator(*self) }
}
