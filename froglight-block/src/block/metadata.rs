use core::{
    any::TypeId,
    fmt::{Debug, Display, Formatter},
};

use froglight_common::identifier::Identifier;

use crate::{
    atomic::{MaybeAtomicU16, MaybeAtomicU32},
    block::{BlockAttr, BlockAttrs, BlockBehavior, BlockType, GlobalId, StateId},
    version::BlockVersion,
};

/// Metadata about a block type.
pub struct BlockMetadata {
    /// The string identifier of the block.
    identifier: Identifier<'static>,
    /// The lowest [`GlobalId`] assigned to this block.
    base_id: MaybeAtomicU32,
    /// The behavior of this block.
    behavior: BlockBehavior,

    /// The number of states for this block.
    state_count: u16,
    /// The default [`StateId`] for this block.
    state_default: MaybeAtomicU16,

    attr_data: &'static [(&'static str, TypeId)],
    get_attr_fn: fn(state: usize, attr: &str) -> Option<&'static str>,
    set_attr_fn: fn(state: usize, attr: &str, value: &str) -> Option<(usize, &'static str)>,

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
        base_id: u32,
        default_state: u16,
        behavior: BlockBehavior,
    ) -> Self {
        assert!(default_state < B::Attributes::TOTAL, "Default StateId is out of range!");

        BlockMetadata {
            identifier,
            base_id: MaybeAtomicU32::new(base_id),
            behavior,

            state_count: B::Attributes::TOTAL,
            state_default: MaybeAtomicU16::new(default_state),

            attr_data: B::ATTRDATA,
            get_attr_fn: |state, name| {
                let attributes = B::Attributes::from_set_index(state)?;
                let index = B::ATTRDATA.iter().position(|&(attr, _)| attr == name)?;
                attributes.get_attr_str(index)
            },
            set_attr_fn: |state, name, value| {
                let mut attributes = B::Attributes::from_set_index(state)?;
                let index = B::ATTRDATA.iter().position(|&(attr, _)| attr == name)?;
                let old_value = attributes.set_attr_str(index, value)?;
                let new_state = attributes.to_set_index();
                Some((new_state, old_value))
            },

            block_ty: TypeId::of::<B>(),
            version_ty: TypeId::of::<V>(),
        }
    }

    /// Get the string identifier of this block.
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'static> { &self.identifier }

    /// Get the behavior of this block.
    #[inline]
    #[must_use]
    pub const fn behavior(&self) -> &BlockBehavior { &self.behavior }

    /// Get the base [`GlobalId`] of this block.
    ///
    /// ## Note
    ///
    /// This is not equivalent to the block's default state!
    #[must_use]
    pub fn base_id(&self) -> GlobalId { GlobalId::new(self.base_id.get()) }

    /// Set the base [`GlobalId`] of this block.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the new id matches the indices in the
    /// [`BlockStorage`](crate::storage::BlockStorage) it is used in.
    #[cfg(feature = "atomic")]
    pub unsafe fn set_base_id(&self, id: GlobalId) { self.base_id.set_atomic(id.into_inner()) }

    /// Get the default [`StateId`] for this block.
    #[must_use]
    pub fn state_default(&self) -> StateId { StateId::new(self.state_default.get()) }

    /// Set the default [`StateId`] of this block.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the state is valid for this block.
    #[cfg(feature = "atomic")]
    pub unsafe fn set_state_default(&self, state: StateId) {
        self.state_default.set_atomic(state.into_inner());
    }

    /// Get the default [`GlobalId`] of this block.
    #[must_use]
    pub fn default_id(&self) -> GlobalId {
        GlobalId::new(self.base_id.get() + u32::from(self.state_default.get()))
    }

    /// Get the number of [`StateId`]s for this block.
    ///
    /// # Note
    ///
    /// This value is *not* zero-indexed.
    /// Blocks with no attributes have a state count of `1`.
    #[inline]
    #[must_use]
    pub fn state_count(&self) -> u16 { self.state_count }

    /// Get the value of an attribute for a given state.
    #[must_use]
    pub fn get_attribute<A: BlockAttr>(&self, state: StateId) -> Option<A> {
        self.attr_data.iter().find_map(|(name, ty)| {
            if *ty == TypeId::of::<A>() {
                self.get_attribute_str(state, name).and_then(A::from_name)
            } else {
                None
            }
        })
    }

    /// Get the value of an attribute as a string for a given state.
    #[must_use]
    pub fn get_attribute_str(&self, state: StateId, name: &str) -> Option<&'static str> {
        (self.get_attr_fn)(usize::from(state.into_inner()), name)
    }

    /// Get the value of all block attributes for a given state.
    pub fn get_attributes(
        &self,
        state: StateId,
    ) -> impl Iterator<Item = (&'static str, &'static str)> {
        self.attr_data.iter().filter_map(move |(name, _)| {
            self.get_attribute_str(state, name).map(|value| (*name, value))
        })
    }

    /// Set the value of an attribute for a given state.
    ///
    /// Returns the new [`StateId`] and the old value if successful.
    #[must_use]
    pub fn set_attribute<A: BlockAttr>(
        &self,
        state: StateId,
        attribute: A,
    ) -> Option<(StateId, A)> {
        self.attr_data.iter().find_map(|(name, ty)| {
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
        state: StateId,
        name: &str,
        value: &str,
    ) -> Option<(StateId, &'static str)> {
        let (state, value) = (self.set_attr_fn)(usize::from(state.into_inner()), name, value)?;
        let state = StateId::new(u16::try_from(state).expect("Invalid StateId, overflowed!"));
        if state.into_inner() < self.state_count { Some((state, value)) } else { None } // Validate in-range
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

impl Display for BlockMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { write!(f, "{}", self.identifier) }
}

impl Debug for BlockMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BlockMetadata")
            .field("identifier", &self.identifier)
            .field("base", &self.base_id)
            .finish_non_exhaustive()
    }
}
