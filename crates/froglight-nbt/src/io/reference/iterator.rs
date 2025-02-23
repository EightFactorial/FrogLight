use super::NbtCompoundRef;
use crate::io::{NamedNbtRef, UnnamedNbtRef};

/// An iterator over the tags in an NBT reference.
#[expect(dead_code)]
pub struct NbtRefIterator<'a>(NbtRefType<'a>);

impl<'a> Iterator for NbtRefIterator<'a> {
    type Item = NbtCompoundRef<'a>;

    fn next(&mut self) -> Option<Self::Item> { todo!() }
}

// -------------------------------------------------------------------------------------------------

/// The type of NBT reference.
#[expect(dead_code)]
enum NbtRefType<'a> {
    Named(NamedNbtRef<'a>),
    Unnamed(UnnamedNbtRef<'a>),
}

impl<'a> NamedNbtRef<'a> {
    /// Create an iterator over the tags in the [`NamedNbtRef`].
    #[inline]
    #[must_use]
    pub fn iter(&self) -> NbtRefIterator<'a> { NbtRefIterator(NbtRefType::Named(*self)) }
}
impl<'a> IntoIterator for &NamedNbtRef<'a> {
    type Item = NbtCompoundRef<'a>;
    type IntoIter = NbtRefIterator<'a>;
    fn into_iter(self) -> Self::IntoIter { NbtRefIterator(NbtRefType::Named(*self)) }
}

impl<'a> UnnamedNbtRef<'a> {
    /// Create an iterator over the tags in the [`UnnamedNbtRef`].
    #[inline]
    #[must_use]
    pub fn iter(&self) -> NbtRefIterator<'a> { NbtRefIterator(NbtRefType::Unnamed(*self)) }
}
impl<'a> IntoIterator for &UnnamedNbtRef<'a> {
    type Item = NbtCompoundRef<'a>;
    type IntoIter = NbtRefIterator<'a>;
    fn into_iter(self) -> Self::IntoIter { NbtRefIterator(NbtRefType::Unnamed(*self)) }
}
