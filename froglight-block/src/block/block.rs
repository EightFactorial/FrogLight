use core::{
    any::TypeId,
    cmp::Ordering,
    fmt::{self, Debug, Display},
};

use froglight_common::identifier::Identifier;

use crate::{
    block::{BlockAttribute, BlockAttributes, BlockMetadata, GlobalId, StateId},
    version::BlockVersion,
};

/// A block in the world.
#[derive(Clone, Copy)]
pub struct Block {
    state: StateId,
    reference: &'static BlockMetadata,
}

impl Block {
    /// Create a new [`Block`] from the given attributes.
    #[must_use]
    #[expect(
        clippy::missing_panics_doc,
        reason = "This should never panic unless something is catastrophically wrong"
    )]
    pub fn new<B: BlockType<V>, V: BlockVersion>(attributes: B::Attributes) -> Block {
        let state = attributes.to_set_index().try_into().ok().map(StateId::new);
        state.and_then(Self::new_state::<B, V>).expect("Invalid Attribute StateId!")
    }

    /// Create a new [`Block`] with the default state.
    #[must_use]
    #[expect(
        clippy::missing_panics_doc,
        reason = "This should never panic unless something is catastrophically wrong"
    )]
    pub fn new_default<B: BlockType<V>, V: BlockVersion>() -> Block {
        Self::new_state::<B, V>(B::METADATA.state_default()).expect("Invalid Default StateId!")
    }

    /// Try to create a new [`Block`] from the given [`StateId`].
    ///
    /// Returns `None` if the [`StateId`] is invalid for the block type.
    #[must_use]
    pub fn new_state<B: BlockType<V>, V: BlockVersion>(state: StateId) -> Option<Block> {
        let metadata = B::METADATA;
        if state.into_inner() < metadata.state_count() {
            Some(Self { state, reference: metadata })
        } else {
            None
        }
    }

    /// Create a new [`Block`] from the given [`StateId`] and [`BlockMetadata`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the given `state` is valid for the metadata.
    #[must_use]
    pub const unsafe fn new_unchecked(state: StateId, metadata: &'static BlockMetadata) -> Self {
        Self { state, reference: metadata }
    }

    /// Get the string identifier of this block.
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'static> { self.reference.identifier() }

    /// Get the [`BlockMetadata`] of this block.
    #[inline]
    #[must_use]
    pub const fn metadata(&self) -> &'static BlockMetadata { self.reference }

    /// Get the [`GlobalId`] of this block.
    ///
    /// ## Note
    ///
    /// This is only useful for comparing blocks of the same version.
    #[must_use]
    pub fn global_id(&self) -> GlobalId {
        GlobalId::new(u32::from(self.state.into_inner()) + self.reference.base_id().into_inner())
    }

    /// Get the [`StateId`] of this block.
    ///
    /// ## Note
    ///
    /// This is only useful for comparing states of the same block.
    #[must_use]
    pub const fn state_id(&self) -> StateId { self.state }

    /// Get the value of an attribute for this block.
    ///
    /// Returns `None` if no attribute of type `A` exists.
    #[must_use]
    pub fn get_attribute<A: BlockAttribute>(&self) -> Option<A> {
        self.reference.get_attribute::<A>(self.state)
    }

    /// Get the value of an attribute as a string for this block.
    ///
    /// Returns `None` if no attribute with the given name exists.
    #[must_use]
    pub fn get_attribute_str(&self, name: &str) -> Option<&'static str> {
        self.reference.get_attribute_str(self.state, name)
    }

    /// Get an iterator over all block attributes and their values.
    pub fn get_attributes(&self) -> impl Iterator<Item = (&'static str, &'static str)> {
        self.reference.get_attributes(self.state)
    }

    /// Set the value of an attribute for this block.
    ///
    /// Returns the old value of the attribute if it was set successfully.
    pub fn set_attribute<A: BlockAttribute>(&mut self, value: A) -> Option<A> {
        let (new_state, old_value) = self.reference.set_attribute::<A>(self.state, value)?;
        self.state = new_state;
        Some(old_value)
    }

    /// Set the value of an attribute as a string for this block.
    ///
    /// Returns the old value of the attribute if it was set successfully.
    pub fn set_attribute_str(&mut self, name: &str, value: &str) -> Option<&'static str> {
        let (new_state, old_value) = self.reference.set_attribute_str(self.state, name, value)?;
        self.state = new_state;
        Some(old_value)
    }

    /// Returns `true` if this block is of type `T`.
    #[inline]
    #[must_use]
    pub fn is_block<B: 'static>(&self) -> bool { self.reference.is_block::<B>() }

    /// Returns `true` if this block is of version `V`.
    #[inline]
    #[must_use]
    pub fn is_version<V: 'static>(&self) -> bool { self.reference.is_version::<V>() }

    /// Get the [`TypeId`] of the block type.
    #[inline]
    #[must_use]
    pub const fn block_ty(&self) -> TypeId { self.reference.block_ty() }

    /// Get the [`TypeId`] of the version type.
    #[inline]
    #[must_use]
    pub const fn version_ty(&self) -> TypeId { self.reference.version_ty() }

    /// Returns `true` if this block is air.
    #[must_use]
    pub fn is_air(&self) -> bool { self.reference.behavior().is_air(self.state) }

    /// Returns `true` if this block is solid.
    #[must_use]
    pub fn is_solid(&self) -> bool { self.reference.behavior().is_solid(self.state) }

    /// Returns `true` if this block is transparent.
    #[must_use]
    pub fn is_transparent(&self) -> bool { self.reference.behavior().is_transparent(self.state) }
}

impl Eq for Block {}
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
            && self.reference.block_ty() == other.reference.block_ty()
            && self.reference.version_ty() == other.reference.version_ty()
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.reference.version_ty() == other.reference.version_ty() {
            self.global_id().partial_cmp(&other.global_id())
        } else {
            None
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { Display::fmt(self.identifier(), f) }
}

impl Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Block")
            .field(self.reference.identifier())
            .field(&self.global_id().into_inner())
            .finish_non_exhaustive()
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait implemented by all block types.
pub trait BlockType<V: BlockVersion>: 'static {
    /// The attribute set for this block type.
    type Attributes: BlockAttributes;

    /// The names and types of the attributes for this block type.
    const ATTRDATA: &'static [(&'static str, TypeId)];
    /// The [`BlockMetadata`] for this block type.
    const METADATA: &'static BlockMetadata;

    /// Returns `true` if the given block is air.
    #[must_use]
    fn is_air(_: StateId) -> bool { false }

    /// Returns `true` if the given block is solid.
    #[must_use]
    fn is_solid(_: StateId) -> bool { true }

    /// Returns `true` if the given block is liquid.
    #[must_use]
    fn is_liquid(_: StateId) -> bool { false }

    /// Returns `true` if the given block has collision.
    #[must_use]
    fn has_collision(_: StateId) -> bool { true }

    /// Returns `true` if the given block is transparent.
    #[must_use]
    fn is_transparent(_: StateId) -> bool { false }

    /// Returns `true` if the given block has occlusion.
    #[must_use]
    fn has_occlusion(_: StateId) -> bool { true }

    /// Returns the block's light emission level.
    #[must_use]
    fn light_emission(_: StateId) -> u8 { 0u8 }
}
