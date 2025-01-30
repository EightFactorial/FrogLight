//! !TODO

use std::marker::PhantomData;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_common::{Identifier, Version};

use super::BlockTypeExt;
use crate::storage::{BlockAttributes, BlockWrapper, RelativeBlockState};

/// A block with a state.
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(no_field_bounds, from_reflect = false, PartialEq))]
pub struct Block<B: BlockTypeExt<V>, V: Version> {
    state: RelativeBlockState,
    #[cfg_attr(feature = "bevy", reflect(ignore))]
    _phantom: PhantomData<(B, V)>,
}

impl<B: BlockTypeExt<V>, V: Version> Block<B, V> {
    /// Get the [`Attributes`](BlockTypeExt::Attributes) of the [`Block`].
    #[must_use]
    pub fn into_attr(self) -> B::Attributes { B::Attributes::from_index(self.state.into()) }

    /// Create a [`Block`] from the given
    /// [`Attributes`](BlockTypeExt::Attributes).
    #[must_use]
    pub fn from_attr(attributes: B::Attributes) -> Self {
        Self { state: RelativeBlockState::from(attributes.into_index()), _phantom: PhantomData }
    }

    /// Modify the [`Attributes`](BlockTypeExt::Attributes) of the [`Block`].
    ///
    /// This is shorthand for calling
    /// [`Block::into_attr`] and [`Block::from_attr`].
    pub fn scoped_attr(&mut self, f: fn(B::Attributes) -> B::Attributes) {
        *self = Self::from_attr(f(self.into_attr()));
    }

    /// Convert the [`Block`] into an [`UntypedBlock`].
    #[must_use]
    pub fn into_untyped(self) -> UntypedBlock<V> { self.into() }

    /// Get the identifier of the [`Block`].
    #[must_use]
    pub fn identifier() -> &'static Identifier { B::as_static().identifier() }
}

impl<A: BlockAttributes, B: BlockTypeExt<V, Attributes = A>, V: Version> Block<B, V> {
    /// Convert the [`Block`] into a [`Block`] of another [`Version`].
    #[must_use]
    pub const fn into_version<V2: Version>(self) -> Block<B, V2>
    where
        B: BlockTypeExt<V2, Attributes = A>,
    {
        Block { state: self.state, _phantom: PhantomData }
    }
}
impl<
        A: BlockAttributes,
        B: BlockTypeExt<V1, Attributes = A> + BlockTypeExt<V2, Attributes = A>,
        V1: Version,
        V2: Version,
    > From<&Block<B, V1>> for Block<B, V2>
{
    fn from(block: &Block<B, V1>) -> Block<B, V2> { block.into_version::<V2>() }
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
    #[must_use]
    pub(crate) const fn new(state: RelativeBlockState, wrapper: BlockWrapper<V>) -> Self {
        Self { state, wrapper }
    }

    /// Get the internal [`RelativeBlockState`] of the [`UntypedBlock`].
    #[must_use]
    pub(crate) const fn state(&self) -> &RelativeBlockState { &self.state }
    /// Get the internal [`BlockWrapper`] of the [`UntypedBlock`].
    #[must_use]
    pub(crate) const fn wrapper(&self) -> &BlockWrapper<V> { &self.wrapper }

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

impl<B: BlockTypeExt<V>, V: Version> Copy for Block<B, V> {}
#[allow(clippy::expl_impl_clone_on_copy, clippy::non_canonical_clone_impl)]
impl<B: BlockTypeExt<V>, V: Version> Clone for Block<B, V> {
    fn clone(&self) -> Self { Self { state: self.state, _phantom: PhantomData } }
}
impl<B: BlockTypeExt<V>, V: Version> Eq for Block<B, V> {}
impl<B: BlockTypeExt<V>, V: Version> PartialEq for Block<B, V> {
    fn eq(&self, other: &Self) -> bool { self.state == other.state }
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
