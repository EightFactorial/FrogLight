use core::{
    any::TypeId,
    fmt::{Debug, Display, Formatter},
};

use crate::{
    atomic::MaybeAtomicU32,
    block::{BlockAttr, BlockAttrs, BlockType, GlobalId, StateId},
    version::BlockVersion,
};

/// Metadata about a block type.
pub struct BlockMetadata {
    /// The string identifier of the block.
    identifier: &'static str,
    /// The lowest [`GlobalId`] assigned to this block.
    base_global: MaybeAtomicU32,
    /// The maximum [`StateId`] for this block.
    max_state: StateId,

    attr_data: &'static [(&'static str, TypeId)],
    get_attr_fn: fn(state: usize, attr: &str) -> Option<&'static str>,
    set_attr_fn: fn(state: usize, attr: &str, value: &'static str) -> Option<(usize, &'static str)>,

    /// The [`TypeId`] of the block type.
    block_ty: TypeId,
    /// The [`TypeId`] of the version type.
    version_ty: TypeId,
}

impl BlockMetadata {
    /// Create a new [`BlockData`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the `base_state` value is correct for the
    /// [`BlockStorage`](crate::storage::BlockStorage) it will be used in.
    #[must_use]
    pub const unsafe fn new<B: BlockType<V>, V: BlockVersion>(
        identifier: &'static str,
        base_state: MaybeAtomicU32,
    ) -> Self {
        BlockMetadata {
            identifier,
            base_global: base_state,
            max_state: StateId::new(B::Attributes::TOTAL),

            attr_data: B::ATTR_DATA,
            get_attr_fn: |state, name| {
                let attributes = B::Attributes::from_set_index(state)?;
                let index = B::ATTR_DATA.iter().position(|&(attr, _)| attr == name)?;
                attributes.get_attr_str(index)
            },
            set_attr_fn: |state, name, value| {
                let mut attributes = B::Attributes::from_set_index(state)?;
                let index = B::ATTR_DATA.iter().position(|&(attr, _)| attr == name)?;
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
    pub const fn identifier(&self) -> &'static str { self.identifier }

    /// Get the base [`GlobalId`] of this block.
    ///
    /// ## Note
    ///
    /// This is not equivalent to the block's default state!
    #[must_use]
    pub fn base_id(&self) -> GlobalId { GlobalId::new(self.base_global.get()) }

    /// Get the maximum valid [`StateId`] for this block.
    #[inline]
    #[must_use]
    pub fn state_max(&self) -> StateId { self.max_state }

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
        reason = "This should never panic unless something is catastrophically wrong
        "
    )]
    pub fn set_attribute_str(
        &self,
        state: StateId,
        name: &str,
        value: &'static str,
    ) -> Option<(StateId, &'static str)> {
        let (state, value) = (self.set_attr_fn)(usize::from(state.into_inner()), name, value)?;
        let state = StateId::new(u16::try_from(state).expect("Invalid StateId, overflowed!"));
        if state <= self.max_state { Some((state, value)) } else { None } // Validate in-range
    }

    /// Returns `true` if this block is of type `B`.
    #[must_use]
    pub fn is_block<B: BlockType<V>, V: BlockVersion>(&self) -> bool {
        self.block_ty == TypeId::of::<B>()
    }

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
            .field("base", &self.base_global)
            .finish_non_exhaustive()
    }
}
