//! TODO

use core::{any::TypeId, cmp::Ordering, fmt};

use froglight_common::identifier::Identifier;
use froglight_registry_template::implement_wrapper;

mod component;
pub use component::{attribute::BlockAttributes, behavior::BlockBehavior, shape::BlockShape};

mod metadata;
pub use metadata::BlockMetadata;

use crate::{
    attribute::{BlockAttribute, BlockAttributeBundle},
    state::{GlobalStateId, RelativeStateId},
    storage::BlockStorage,
    version::BlockVersion,
};

/// A block in the world.
#[derive(Clone, Copy)]
pub struct Block {
    state: RelativeStateId,
    metadata: &'static BlockMetadata,
}

impl Block {
    /// Create a new [`Block`] from the given attributes.
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "This should never panic")]
    pub fn new<B: BlockType<V>, V: BlockVersion>(attributes: B::Attributes) -> Block {
        let state = attributes.to_set_index().try_into().ok().map(RelativeStateId::new);
        state.and_then(Self::try_new_from::<B, V>).expect("Invalid Attribute RelativeStateId!")
    }

    /// Create a new [`Block`] with the default state.
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "This should never panic")]
    pub fn new_default<B: BlockType<V>, V: BlockVersion>() -> Block {
        Self::try_new_from::<B, V>(B::METADATA.state_default())
            .expect("Invalid Default RelativeStateId!")
    }

    /// Try to create a new [`Block`] from the given [`RelativeStateId`].
    ///
    /// Returns `None` if the [`RelativeStateId`] is invalid for the block type.
    #[must_use]
    pub const fn try_new_from<B: BlockType<V>, V: BlockVersion>(
        state: RelativeStateId,
    ) -> Option<Block> {
        let metadata = B::METADATA;
        if state.into_inner() < metadata.state_count() {
            Some(Self { state, metadata })
        } else {
            None
        }
    }

    /// Create a new [`Block`] from the given [`RelativeStateId`] and
    /// [`BlockMetadata`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the given `state` is valid for the metadata.
    #[must_use]
    pub const unsafe fn new_unchecked(
        state: RelativeStateId,
        metadata: &'static BlockMetadata,
    ) -> Self {
        Self { state, metadata }
    }

    /// Get the string identifier of this block.
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'static> { self.metadata.identifier() }

    /// Get the [`BlockMetadata`] of this block.
    #[inline]
    #[must_use]
    pub const fn metadata(&self) -> &'static BlockMetadata { self.metadata }

    /// Get the [`RelativeStateId`] of this block.
    ///
    /// ## Note
    ///
    /// This is only useful for comparing states of the same block.
    #[must_use]
    pub const fn state_id(&self) -> RelativeStateId { self.state }

    /// Get the [`GlobalStateId`] of this block.
    ///
    /// ## Note
    ///
    /// This is only useful for comparing blocks of the same version.
    #[must_use]
    pub fn global_id(&self) -> GlobalStateId {
        let state = u32::from(self.state.into_inner());
        let base = self.metadata.base_id().into_inner();
        GlobalStateId::new(base + state)
    }

    /// Attempt to migrate the block to another [`BlockVersion`].
    ///
    /// Returns `None` if there is no matching [`BlockType`].
    #[inline]
    #[must_use]
    pub fn using_version<V: BlockVersion>(self) -> Option<Block> {
        self.using_version_storage(&V::blocks())
    }

    /// Attempt to migrate the block to another [`Version`]'s [`BlockStorage`].
    ///
    /// Equivalent to [`Block::using_version`] but without the generic
    /// parameter.
    ///
    /// Returns `None` if there is no matching [`BlockType`].
    #[must_use]
    pub fn using_version_storage(self, blocks: &BlockStorage) -> Option<Block> {
        // If the `Version` is the same, do nothing.
        if self.version_ty() == blocks.version_ty() {
            return Some(self);
        }

        // Try the block with a matching identifier and type.
        if let Some(mut block) = blocks.get_block_by_identifier(self.identifier())
            && self.block_ty() == block.block_ty()
        {
            self.apply_attributes(&mut block);
            return Some(block);
        }

        // Otherwise, iterate over all blocks for a matching type.
        blocks.metadata().iter().find_map(|meta| {
            if self.block_ty() == meta.block_ty() {
                blocks.get_block_by_identifier(meta.identifier())
            } else {
                None
            }
        })
    }

    /// Apply the attributes of the current [`Block`] to another.
    ///
    /// If an attribute does not exist, or the value is not valid for the block,
    /// it will not be applied.
    pub fn apply_attributes(&self, block: &mut Block) {
        for (name, value) in self.get_attributes() {
            block.set_attribute_str(name, value);
        }
    }

    /// Set the value of an attribute for this block.
    ///
    /// Returns the old value of the attribute if it was set successfully.
    pub fn set_attribute<A: BlockAttribute>(&mut self, value: A) -> Option<A> {
        let (new_state, old_value) = self.metadata.set_attribute::<A>(self.state, value)?;
        self.state = new_state;
        Some(old_value)
    }

    /// Set the value of an attribute as a string for this block.
    ///
    /// Returns the old value of the attribute if it was set successfully.
    pub fn set_attribute_str(&mut self, name: &str, value: &str) -> Option<&'static str> {
        let (new_state, old_value) = self.metadata.set_attribute_str(self.state, name, value)?;
        self.state = new_state;
        Some(old_value)
    }
}

implement_wrapper! {
    impl Block {
        [ () => metadata ]: {
            /// Returns `true` if this block is of type `T`.
            #[inline]
            #[must_use]
            pub fn is_block<B: 'static>(&self) -> bool;

            /// Returns `true` if this block is of version `V`.
            #[inline]
            #[must_use]
            pub fn is_version<V: 'static>(&self) -> bool;

            /// Get the [`TypeId`] of the block type.
            #[inline]
            #[must_use]
            pub fn block_ty(&self) -> TypeId;

            /// Get the [`TypeId`] of the version type.
            #[inline]
            #[must_use]
            pub fn version_ty(&self) -> TypeId;
        }

        [ state => metadata ]: {
            /// Get the value of an attribute for this block.
            ///
            /// Returns `None` if no attribute of type `A` exists.
            #[must_use]
            pub fn get_attribute<A: BlockAttribute>(&self) -> Option<A>;

            /// Get the value of an attribute as a string for this block.
            ///
            /// Returns `None` if no attribute with the given name exists.
            #[must_use]
            pub fn get_attribute_str(&self, name: &str) -> Option<&'static str>;

            /// Get an iterator over all block attributes and their values.
            pub fn get_attributes(&self) -> impl Iterator<Item = (&'static str, &'static str)>;
        }

        [ state => metadata.behavior() ]: {
            /// Returns `true` if this block is air.
            #[must_use]
            pub fn is_air(&self) -> bool;

            /// Returns `true` if this block is solid.
            #[must_use]
            pub fn is_solid(&self) -> bool;

            /// Returns `true` if this block is liquid.
            #[must_use]
            pub fn is_liquid(&self) -> bool;

            /// Returns `true` if this block has collision.
            #[must_use]
            pub fn has_collision(&self) -> bool;

            /// Returns `true` if this block has occlusion.
            #[must_use]
            pub fn has_occlusion(&self) -> bool;

            /// Returns `true` if this block is transparent.
            #[must_use]
            pub fn is_transparent(&self) -> bool;

            /// Returns the [`BlockShape`] of this block.
            #[must_use]
            pub fn shape_of(&self) -> &'static BlockShape<'static>;
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Eq for Block {}
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
            && self.metadata.block_ty() == other.metadata.block_ty()
            && self.metadata.version_ty() == other.metadata.version_ty()
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.metadata.version_ty() == other.metadata.version_ty() {
            self.global_id().partial_cmp(&other.global_id())
        } else {
            None
        }
    }
}

impl fmt::Display for Block {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Identifier as fmt::Display>::fmt(self.identifier(), f)
    }
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Block")
            .field(self.identifier())
            .field(&self.global_id().into_inner())
            .finish_non_exhaustive()
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait implemented by all block types.
pub trait BlockType<V: BlockVersion>: 'static {
    /// The attribute set for this block type.
    type Attributes: BlockAttributeBundle;

    /// The names and types of the attributes for this block type.
    const ATTRDATA: &'static [(&'static str, TypeId)];
    /// The [`BlockMetadata`] for this block type.
    const METADATA: &'static BlockMetadata;

    /// Returns `true` if the given block is air.
    #[must_use]
    fn is_air(_: RelativeStateId) -> bool { false }

    /// Returns `true` if the given block is solid.
    #[must_use]
    fn is_solid(_: RelativeStateId) -> bool { true }

    /// Returns `true` if the given block is liquid.
    #[must_use]
    fn is_liquid(_: RelativeStateId) -> bool { false }

    /// Returns `true` if the given block has collision.
    #[must_use]
    fn has_collision(_: RelativeStateId) -> bool { true }

    /// Returns `true` if the given block is transparent.
    #[must_use]
    fn is_transparent(_: RelativeStateId) -> bool { false }

    /// Returns `true` if the given block has occlusion.
    #[must_use]
    fn has_occlusion(_: RelativeStateId) -> bool { true }

    /// Returns the block's light emission level.
    #[must_use]
    fn light_emission(_: RelativeStateId) -> u8 { 0u8 }

    /// Returns the shape of the given block state.
    #[must_use]
    fn shape_of(_: RelativeStateId) -> &'static BlockShape<'static> { &BlockShape::FULL }
}
