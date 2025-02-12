//! !TODO

use std::marker::PhantomData;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use downcast_rs::Downcast;
use froglight_common::{Identifier, Version};

use super::{BlockConvert, BlockType, BlockTypeExt};
use crate::{
    resolve::BlockResolver,
    storage::{Attribute, BlockAttributes, BlockWrapper, RelativeBlockState},
};

/// A block with a state.
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(no_field_bounds, from_reflect = false, PartialEq))]
pub struct Block<B: BlockTypeExt<V>, V: Version> {
    state: RelativeBlockState,
    #[cfg_attr(feature = "bevy", reflect(ignore))]
    _phantom: PhantomData<(B, V)>,
}

impl<B: BlockTypeExt<V>, V: Version> Block<B, V> {
    /// Create a new [`Block`] from the given [`RelativeBlockState`].
    #[inline]
    #[must_use]
    pub(crate) const fn new(state: RelativeBlockState) -> Self {
        Self { state, _phantom: PhantomData }
    }

    /// Get the internal [`RelativeBlockState`] of the [`Block`].
    #[inline]
    #[must_use]
    pub(crate) const fn state(&self) -> &RelativeBlockState { &self.state }

    /// Convert a [`Block`] from another [`Version`] into this [`Version`].
    #[inline]
    #[must_use]
    pub fn from_version<V2: Version>(block: Block<B, V2>) -> Block<B, V>
    where
        B: BlockConvert<V, V2>,
    {
        B::convert_from(block)
    }

    /// Convert this [`Block`] into a [`Block`] from another [`Version`].
    #[inline]
    #[must_use]
    pub fn into_version<V2: Version>(self) -> Block<B, V2>
    where
        B: BlockConvert<V, V2>,
    {
        B::convert_into(self)
    }

    /// Convert the [`Block`] into an [`UntypedBlock`].
    #[inline]
    #[must_use]
    pub fn into_untyped(self) -> UntypedBlock<V> { self.into() }

    /// Create a [`Block`] from the given
    /// [`Attributes`](BlockTypeExt::Attributes).
    #[inline]
    #[must_use]
    pub fn from_attr(attributes: B::Attributes) -> Self {
        Self { state: RelativeBlockState::from(attributes.into_index()), _phantom: PhantomData }
    }

    /// Get the [`Attributes`](BlockTypeExt::Attributes) of the [`Block`].
    #[inline]
    #[must_use]
    pub fn into_attr(self) -> B::Attributes { B::Attributes::from_index(self.state.into()) }

    /// Modify the [`Attributes`](BlockTypeExt::Attributes) of the [`Block`].
    ///
    /// This is shorthand for calling
    /// [`Block::into_attr`] and [`Block::from_attr`].
    #[inline]
    pub fn scoped_attr(&mut self, f: fn(B::Attributes) -> B::Attributes) {
        *self = Self::from_attr(f(self.into_attr()));
    }

    /// Get the [`Attributes`](BlockTypeExt::Attributes) of the [`Block`].
    ///
    /// Returns `None` if the [`Attribute`] is not present.
    #[inline]
    #[must_use]
    pub fn get_attr<T: Attribute>(&self) -> Option<T> {
        B::Attributes::get_attr::<T>(&self.into_attr())
    }

    /// Get the string value of an [`Attribute`].
    ///
    /// Returns `None` if the [`Attribute`] is not present.
    #[must_use]
    pub fn get_attr_str(&self, attr: &str) -> Option<&'static str> {
        B::ATTRIBUTES
            .iter()
            .position(|&name| name == attr)
            .map(|i| self.into_attr().get_attr_str(i))
    }

    /// Set the string value of an [`Attribute`].
    ///
    /// Returns the previous value of the [`Attribute`],
    /// or `None` if the [`Attribute`] is not present.
    pub fn set_attr_str(&mut self, attr: &str, value: &'static str) -> Option<&'static str> {
        B::ATTRIBUTES
            .iter()
            .position(|&name| name == attr)
            .and_then(|i| self.into_attr().set_attr_str(i, value))
    }

    /// Get the identifier of the [`Block`].
    #[inline]
    #[must_use]
    pub fn identifier() -> &'static Identifier { B::as_static().identifier() }
}

impl<B: BlockTypeExt<V>, V: Version> Default for Block<B, V> {
    fn default() -> Self { Self::new(RelativeBlockState::from(B::DEFAULT)) }
}

impl<B: BlockTypeExt<V>, V: Version> TryFrom<UntypedBlock<V>> for Block<B, V> {
    type Error = UntypedBlock<V>;
    fn try_from(value: UntypedBlock<V>) -> Result<Self, Self::Error> {
        value.downcast().ok_or(value)
    }
}

/// An untyped block with a state.
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(no_field_bounds, from_reflect = false, PartialEq))]
pub struct UntypedBlock<V: Version> {
    state: RelativeBlockState,
    #[cfg_attr(feature = "bevy", reflect(ignore))]
    wrapper: BlockWrapper<V>,
}

impl<V: Version> UntypedBlock<V> {
    /// Create a new [`UntypedBlock`] from the given
    /// [`Block`] and [`BlockWrapper`].
    #[inline]
    #[must_use]
    pub(crate) const fn new(state: RelativeBlockState, wrapper: BlockWrapper<V>) -> Self {
        Self { state, wrapper }
    }

    /// Get the internal [`RelativeBlockState`] of the [`UntypedBlock`].
    #[inline]
    #[must_use]
    pub(crate) const fn state(&self) -> &RelativeBlockState { &self.state }
    /// Get the internal [`BlockWrapper`] of the [`UntypedBlock`].
    #[inline]
    #[must_use]
    pub(crate) const fn wrapper(&self) -> &BlockWrapper<V> { &self.wrapper }

    /// Resolve the [`UntypedBlock`] into a typed [`Block`].
    ///
    /// Returns `None` if the block is not in the resolver.
    #[inline]
    #[must_use]
    pub fn resolve<R: BlockResolver<V>>(self) -> Option<R::BlockEnum> { R::resolve(self) }

    /// Returns `true` if the [`Block`] is of a [`BlockType`].
    #[inline]
    #[must_use]
    pub fn is<B: BlockTypeExt<V>>(&self) -> bool {
        <dyn BlockType<V> as Downcast>::as_any(*self.wrapper).type_id()
            == <dyn BlockType<V> as Downcast>::as_any(B::as_static()).type_id()
    }

    /// Downcast the [`UntypedBlock`] into a [`Block`].
    ///
    /// Returns `None` if the [`Block`] is not of the given [`BlockType`].
    #[inline]
    #[must_use]
    pub fn downcast<B: BlockTypeExt<V>>(self) -> Option<Block<B, V>> {
        self.is::<B>().then(|| Block::new(self.state))
    }

    /// Get the identifier of the [`UntypedBlock`].
    #[inline]
    #[must_use]
    pub fn identifier(&self) -> &'static Identifier { self.wrapper.identifier() }
}

impl<B: BlockTypeExt<V>, V: Version> From<Block<B, V>> for UntypedBlock<V> {
    fn from(block: Block<B, V>) -> Self {
        UntypedBlock::new(block.state, BlockWrapper::new(B::as_static()))
    }
}

// ---- Manual trait implementations to avoid trait bounds ----

impl<B: BlockTypeExt<V>, V: Version> std::fmt::Debug for Block<B, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Block").field(&self.state).finish()
    }
}
impl<B: BlockTypeExt<V>, V: Version> Copy for Block<B, V> {}
#[allow(clippy::expl_impl_clone_on_copy, clippy::non_canonical_clone_impl)]
impl<B: BlockTypeExt<V>, V: Version> Clone for Block<B, V> {
    fn clone(&self) -> Self { Self { state: self.state, _phantom: PhantomData } }
}
impl<B: BlockTypeExt<V>, V: Version> Eq for Block<B, V> {}
impl<B: BlockTypeExt<V>, V: Version> PartialEq for Block<B, V> {
    fn eq(&self, other: &Self) -> bool { self.state == other.state }
}

impl<V: Version> std::fmt::Debug for UntypedBlock<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UntypedBlock").field(&self.state).field(&self.wrapper.identifier()).finish()
    }
}
impl<V: Version> Copy for UntypedBlock<V> {}
#[allow(clippy::expl_impl_clone_on_copy, clippy::non_canonical_clone_impl)]
impl<V: Version> Clone for UntypedBlock<V> {
    fn clone(&self) -> Self { Self { state: self.state, wrapper: self.wrapper } }
}
impl<V: Version> Eq for UntypedBlock<V> {}
impl<V: Version> PartialEq for UntypedBlock<V> {
    fn eq(&self, other: &Self) -> bool { self.state == other.state }
}
