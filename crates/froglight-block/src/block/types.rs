//! !TODO

use std::marker::PhantomData;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use downcast_rs::Downcast;
use froglight_common::{Identifier, Version};

use super::{BlockConvert, BlockType, BlockTypeExt};
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
    /// Create a new [`Block`] from the given [`RelativeBlockState`].
    pub(crate) const fn new(state: RelativeBlockState) -> Self {
        Self { state, _phantom: PhantomData }
    }

    /// Get the internal [`RelativeBlockState`] of the [`Block`].
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
    #[must_use]
    pub fn into_untyped(self) -> UntypedBlock<V> { self.into() }

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

    /// Get the identifier of the [`Block`].
    #[must_use]
    pub fn identifier() -> &'static Identifier { B::as_static().identifier() }
}

impl<B: BlockTypeExt<V>, V: Version> TryFrom<UntypedBlock<V>> for Block<B, V> {
    type Error = ();
    fn try_from(value: UntypedBlock<V>) -> Result<Self, Self::Error> {
        value.downcast().map_or(Err(()), |b| Ok(b))
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

    /// Returns `true` if the [`Block`] is of a [`BlockType`].
    #[inline]
    #[must_use]
    pub fn is<B: BlockType<V>>(&self) -> bool {
        <&'static dyn BlockType<V> as Downcast>::as_any(&self.wrapper).is::<B>()
    }

    /// Try to downcast an [`UntypedBlock`] into a [`Block`].
    ///
    /// Returns `None` if the [`BlockType`] does not match.
    #[inline]
    #[must_use]
    pub fn downcast<B: BlockTypeExt<V>>(&self) -> Option<Block<B, V>> {
        if self.is::<B>() {
            Some(Block::new(self.state))
        } else {
            None
        }
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
