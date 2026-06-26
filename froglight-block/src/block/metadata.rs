use core::{any::TypeId, fmt};

use froglight_common::identifier::Identifier;

use crate::{
    attribute::{BlockAttribute, BlockAttributeBundle},
    block::{BlockAttributes, BlockBehavior},
    prelude::{BlockType, BlockVersion},
    state::{GlobalStateId, RelativeStateId},
};

/// Metadata about a block type.
pub struct BlockMetadata {
    /// The string identifier of the block.
    identifier: Identifier<'static>,
    /// The lowest [`GlobalStateId`] assigned to this block.
    base_global_id: GlobalStateId,
    /// The default [`RelativeStateId`] for this block.
    default_state: RelativeStateId,

    /// The behavior of this block.
    behavior: BlockBehavior,
    /// The attributes of this block.
    attributes: BlockAttributes,

    /// The [`TypeId`] of the block type.
    block_ty: TypeId,
    /// The [`TypeId`] of the version type.
    version_ty: TypeId,
}

impl BlockMetadata {
    /// Create a new [`BlockMetadata`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the `base_id` value is correct for the
    /// [`BlockStorage`](crate::storage::BlockStorage) it will be used in.
    ///
    /// # Panics
    ///
    /// Panics if the `default_state` is out of range for the block type.
    #[must_use]
    pub const unsafe fn new<B: BlockType<V>, V: BlockVersion>(
        identifier: Identifier<'static>,
        base_global_id: GlobalStateId,
        default_state: RelativeStateId,
    ) -> Self {
        assert!(
            default_state.into_inner() < B::Attributes::TOTAL,
            "Default StateId is out of range!"
        );

        BlockMetadata {
            identifier,
            base_global_id,
            default_state,

            behavior: BlockBehavior::new::<B, V>(),
            attributes: BlockAttributes::new::<B, V>(),

            block_ty: TypeId::of::<B>(),
            version_ty: TypeId::of::<V>(),
        }
    }

    /// Convert a state using this metadata into another metadata.
    ///
    /// If an invalid state is given the default state will be used instead.
    #[must_use]
    pub fn try_using_metadata(
        &self,
        mut state: RelativeStateId,
        metadata: &'static BlockMetadata,
    ) -> RelativeStateId {
        if self.state_count() < state.into_inner() {
            state = self.state_default();
        }

        let mut other = metadata.state_default();
        for (name, value) in self.get_attributes(state) {
            if let Some((updated, _)) = metadata.set_attribute_str(other, name, value) {
                other = updated;
            }
        }

        other
    }

    /// Get the string identifier of this block.
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'static> { &self.identifier }

    /// Get the behavior of this block.
    #[inline]
    #[must_use]
    pub const fn behavior(&self) -> &BlockBehavior { &self.behavior }

    /// Get the base [`GlobalStateId`] of this block.
    ///
    /// ## Note
    ///
    /// This is not equivalent to the block's default state!
    #[must_use]
    pub const fn base_id(&self) -> GlobalStateId { self.base_global_id }

    /// Get the default [`StateId`] for this block.
    #[must_use]
    pub const fn state_default(&self) -> RelativeStateId { self.default_state }

    /// Get the default [`GlobalStateId`] of this block.
    #[must_use]
    pub const fn default_id(&self) -> GlobalStateId {
        let base_global = self.base_global_id.into_inner();
        let default_state = self.default_state.into_inner() as u32;
        GlobalStateId::new(base_global + default_state)
    }

    /// Get the number of [`StateId`]s for this block.
    ///
    /// # Note
    ///
    /// This value is *not* zero-indexed.
    /// Blocks with no attributes have a state count of `1`.
    #[inline]
    #[must_use]
    pub const fn state_count(&self) -> u16 { self.attributes.states }

    /// Get the value of an attribute for a given state.
    #[must_use]
    pub fn get_attribute<A: BlockAttribute>(&self, state: RelativeStateId) -> Option<A> {
        self.attributes.list.iter().find_map(|(name, ty)| {
            if *ty == TypeId::of::<A>() {
                self.get_attribute_str(state, name).and_then(A::from_name)
            } else {
                None
            }
        })
    }

    /// Get the value of an attribute as a string for a given state.
    #[must_use]
    pub fn get_attribute_str(&self, state: RelativeStateId, name: &str) -> Option<&'static str> {
        (self.attributes.get_attr_fn)(usize::from(state.into_inner()), name)
    }

    /// Get the value of all block attributes for a given state.
    pub fn get_attributes(
        &self,
        state: RelativeStateId,
    ) -> impl Iterator<Item = (&'static str, &'static str)> {
        self.attributes.list.iter().filter_map(move |(name, _)| {
            self.get_attribute_str(state, name).map(|value| (*name, value))
        })
    }

    /// Set the value of an attribute for a given state.
    ///
    /// Returns the new [`StateId`] and the old value if successful.
    #[must_use]
    pub fn set_attribute<A: BlockAttribute>(
        &self,
        state: RelativeStateId,
        attribute: A,
    ) -> Option<(RelativeStateId, A)> {
        self.attributes.list.iter().find_map(|(name, ty)| {
            if *ty == TypeId::of::<A>() {
                let value = attribute.to_name();
                self.set_attribute_str(state, name, value).and_then(|(new_state, old_value)| {
                    A::from_name(old_value).map(|old_attr| (new_state, old_attr))
                })
            } else {
                None
            }
        })
    }

    /// Set the value of an attribute as a string for a given state.
    ///
    /// Returns the new [`StateId`] and the old value if successful.
    #[must_use]
    #[expect(
        clippy::missing_panics_doc,
        reason = "This should never panic unless something is catastrophically wrong"
    )]
    pub fn set_attribute_str(
        &self,
        state: RelativeStateId,
        name: &str,
        value: &str,
    ) -> Option<(RelativeStateId, &'static str)> {
        let (state, value) =
            (self.attributes.set_attr_fn)(usize::from(state.into_inner()), name, value)?;
        let state =
            RelativeStateId::new(u16::try_from(state).expect("Invalid StateId, overflowed!"));

        if state.into_inner() < self.attributes.states { Some((state, value)) } else { None }
    }

    /// Returns `true` if this block is of type `B`.
    #[must_use]
    pub fn is_block<B: 'static>(&self) -> bool { self.block_ty == TypeId::of::<B>() }

    /// Returns `true` if this block is of version `V`.
    #[must_use]
    pub fn is_version<V: 'static>(&self) -> bool { self.version_ty == TypeId::of::<V>() }

    /// Get the [`TypeId`] of the block type.
    #[inline]
    #[must_use]
    pub const fn block_ty(&self) -> TypeId { self.block_ty }

    /// Get the [`TypeId`] of the version type.
    #[inline]
    #[must_use]
    pub const fn version_ty(&self) -> TypeId { self.version_ty }
}

impl fmt::Display for BlockMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.identifier) }
}

impl fmt::Debug for BlockMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BlockMetadata")
            .field("identifier", &self.identifier)
            .field("base", &self.base_global_id)
            .finish_non_exhaustive()
    }
}
