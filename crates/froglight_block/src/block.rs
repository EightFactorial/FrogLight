//! TODO

use core::{
    borrow::Borrow,
    fmt::Debug,
    ops::{Deref, Range},
};

use froglight_common::version::Version;

#[cfg(feature = "nightly")]
use crate::attribute::BlockAttribute;
use crate::{
    attribute::BlockAttributes,
    info::{BlockDefinitionMap, BlockInfo},
    storage::Blocks,
};

/// A trait implemented by all block types, for each [`Version`] they exist.
pub trait BlockType<V: Version>: 'static {
    /// The attributes of this block type.
    type Attributes: BlockAttributes;
    /// The names of the attributes, in order.
    const ATTRIBUTE_NAMES: &'static [&'static str];

    /// Get a statuc reference to the block's information.
    fn info() -> &'static BlockInfo;
}

// -------------------------------------------------------------------------------------------------

/// A block in the world.
///
/// Contains both the block's state and information.
// TODO: Look into `#[repr(Rust, packed)]`
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Block {
    block_info: &'static BlockInfo,
    block_state: u16,
}

impl Block {
    /// Create a new [`Block`] with the default state.
    #[inline]
    #[must_use]
    pub fn new_default<B: BlockType<V>, V: Blocks>() -> Self {
        let block_info = B::info();
        Self { block_state: block_info.default_state(), block_info }
    }

    /// Create a new [`Block`] from the given attributes.
    #[inline]
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "Should never panic")]
    #[expect(clippy::cast_possible_truncation, reason = "A block will never have 65,535 states")]
    pub fn new_from<B: BlockType<V>, V: Blocks>(attrs: B::Attributes) -> Self {
        let block_info = B::info();
        let block_state = B::Attributes::into_index(attrs).expect("Invalid block attributes!");
        Self { block_state: block_state as u16, block_info }
    }

    /// Get the block's information.
    #[inline]
    #[must_use]
    pub const fn block_info(&self) -> &'static BlockInfo { self.block_info }

    /// Get the block's definition.
    #[inline]
    #[must_use]
    pub const fn block_definition(&self) -> &'static BlockDefinitionMap {
        self.block_info.definition()
    }

    /// Convert a [`Block`] into a [`GlobalBlockState`].
    #[must_use]
    pub fn into_global(self) -> GlobalBlockState {
        GlobalBlockState { block_state: u32::from(self.block_state) + self.block_info.base_id() }
    }

    /// Convert a [`Block`] into a [`RawBlockState`].
    #[must_use]
    pub const fn into_state(self) -> RawBlockState {
        RawBlockState { block_state: self.block_state }
    }

    /// Returns `true` if this block is of the given type.
    #[inline]
    #[must_use]
    #[cfg(feature = "nightly")]
    pub fn is_block<B: BlockType<V>, V: Version>(&self) -> bool {
        self.block_info.is_block::<B, V>()
    }

    /// Get the block's attribute with the given name as a string slice.
    #[inline]
    #[must_use]
    pub fn get_attribute_str(&self, attr: &str) -> Option<&'static str> {
        self.block_info.get_attr_name(self.block_state, attr)
    }

    /// Get all of the block's attributes as an iterator of name-value pairs.
    pub fn get_attribute_strs(&self) -> impl Iterator<Item = (&'static str, &'static str)> {
        self.block_info
            .attributes()
            .iter()
            .filter_map(|&name| self.get_attribute_str(name).map(|value| (name, value)))
    }

    /// Get the block's attribute at the given index as a string slice.
    #[inline]
    #[must_use]
    pub fn get_attribute_idx(&self, index: usize) -> Option<&'static str> {
        self.block_info.get_attr_index(self.block_state, index)
    }

    /// Set the block's attribute with the given name to a string slice.
    #[inline]
    #[must_use]
    pub fn set_attribute_str(&mut self, attr: &str, value: &'static str) -> Option<&'static str> {
        let (state, previous) = self.block_info.set_attr_name(self.block_state, attr, value)?;
        self.block_state = state;

        Some(previous)
    }

    /// Set the block's attribute at the given index to a string slice.
    #[inline]
    #[must_use]
    pub fn set_attribute_idx(&mut self, index: usize, value: &'static str) -> Option<&'static str> {
        let (state, previous) = self.block_info.set_attr_index(self.block_state, index, value)?;
        self.block_state = state;

        Some(previous)
    }

    /// Get the block's attributes, if it is of the given [`BlockType`].
    ///
    /// Returns `None` if the block is not of the given type.
    #[inline]
    #[must_use]
    #[cfg(feature = "nightly")]
    pub fn get_attributes<B: BlockType<V>, V: Version>(&self) -> Option<B::Attributes> {
        if self.is_block::<B, V>() {
            B::Attributes::from_state(usize::from(self.block_state))
        } else {
            None
        }
    }

    /// Set the block's attributes, if it is of the given [`BlockType`].
    ///
    /// Returns the previous attributes,
    /// or `None` if the block is not of the given type.
    #[inline]
    #[must_use]
    #[cfg(feature = "nightly")]
    #[expect(clippy::cast_possible_truncation, reason = "A block will never have 65,535 states")]
    pub fn set_attributes<B: BlockType<V>, V: Version>(
        &mut self,
        attributes: B::Attributes,
    ) -> Option<B::Attributes> {
        self.get_attributes::<B, V>().and_then(|attrs| {
            self.block_state = B::Attributes::into_index(attributes)? as u16;
            Some(attrs)
        })
    }

    /// Get a [`BlockAttribute`], if it is of the given [`BlockType`].
    ///
    /// Returns `None` if the block is not of the given type,
    /// or if the block does not have the attribute.
    #[inline]
    #[must_use]
    #[cfg(feature = "nightly")]
    pub fn get_attribute<A: BlockAttribute, B: BlockType<V>, V: Version>(&self) -> Option<A> {
        self.get_attributes::<B, V>().and_then(|attrs| attrs.get_attr::<A>())
    }

    /// Set a [`BlockAttribute`], if it is of the given [`BlockType`].
    ///
    /// Returns the previous attribute,
    /// or `None` if the block is not of the given type.
    #[inline]
    #[must_use]
    #[cfg(feature = "nightly")]
    #[expect(clippy::cast_possible_truncation, reason = "A block will never have 65,535 states")]
    pub fn set_attribute<A: BlockAttribute, B: BlockType<V>, V: Version>(
        &mut self,
        attr: A,
    ) -> Option<A> {
        self.get_attributes::<B, V>().and_then(|mut attrs| {
            let previous = attrs.set_attr(attr)?;
            self.block_state = B::Attributes::into_index(attrs)? as u16;
            Some(previous)
        })
    }

    /// Create a new [`Block`] from a raw state and block info.
    ///
    /// # SAFETY
    ///
    /// You must ensure that the block's state value is valid for the
    /// [`BlockInfo`].
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(block_info: &'static BlockInfo, block_state: u16) -> Self {
        Self { block_info, block_state }
    }
}

impl AsRef<u16> for Block {
    fn as_ref(&self) -> &u16 { &self.block_state }
}
impl Borrow<u16> for Block {
    fn borrow(&self) -> &u16 { &self.block_state }
}

impl Debug for Block {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Block({}, {})", self.block_info.identifier(), self.block_state)
    }
}

impl Deref for Block {
    type Target = u16;

    fn deref(&self) -> &Self::Target { &self.block_state }
}

// -------------------------------------------------------------------------------------------------

/// A block's global state.
///
/// Contains the block's state id relative to all block states in the same
/// [`Version`].
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalBlockState {
    block_state: u32,
}

impl GlobalBlockState {
    /// Convert the [`GlobalBlockState`] into a [`Block`].
    #[must_use]
    pub fn into_block<V: Blocks>(self) -> Option<Block> {
        V::blocks().read_blocking().get_block(self)
    }

    /// Convert the [`GlobalBlockState`] into a [`BlockInfo`].
    #[must_use]
    pub fn into_info<V: Blocks>(self) -> Option<&'static BlockInfo> {
        V::blocks().read_blocking().get_info(self)
    }

    /// Convert the [`GlobalBlockState`] into the range of valid
    /// [`GlobalBlockState`]s for the block state.
    #[must_use]
    pub fn into_range<V: Blocks>(self) -> Option<Range<GlobalBlockState>> {
        V::blocks().read_blocking().get_range(self)
    }
}

impl AsRef<u32> for GlobalBlockState {
    fn as_ref(&self) -> &u32 { &self.block_state }
}
impl Borrow<u32> for GlobalBlockState {
    fn borrow(&self) -> &u32 { &self.block_state }
}

impl From<u32> for GlobalBlockState {
    fn from(block_state: u32) -> Self { GlobalBlockState { block_state } }
}
impl From<GlobalBlockState> for u32 {
    fn from(global_block_state: GlobalBlockState) -> Self { global_block_state.block_state }
}

impl Debug for GlobalBlockState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "GlobalBlockState({})", self.block_state)
    }
}

impl Deref for GlobalBlockState {
    type Target = u32;

    fn deref(&self) -> &Self::Target { &self.block_state }
}

// -------------------------------------------------------------------------------------------------

/// A block's relative state.
///
/// Contains only the block's state id, without any associated
/// information. This makes it much smaller, but has almost no functionality.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawBlockState {
    block_state: u16,
}

impl RawBlockState {
    /// Convert a [`RawBlockState`] into a [`Block`].
    ///
    /// # SAFETY
    ///
    /// You must ensure that the block's state value is valid for the
    /// [`BlockInfo`].
    #[must_use]
    pub const unsafe fn with_info(self, info: &'static BlockInfo) -> Block {
        Block { block_info: info, block_state: self.block_state }
    }
}

impl AsRef<u16> for RawBlockState {
    fn as_ref(&self) -> &u16 { &self.block_state }
}
impl Borrow<u16> for RawBlockState {
    fn borrow(&self) -> &u16 { &self.block_state }
}

impl From<u16> for RawBlockState {
    fn from(block_state: u16) -> Self { RawBlockState { block_state } }
}
impl From<RawBlockState> for u16 {
    fn from(raw_block_state: RawBlockState) -> Self { raw_block_state.block_state }
}

impl Debug for RawBlockState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "RawBlockState({})", self.block_state)
    }
}

impl Deref for RawBlockState {
    type Target = u16;

    fn deref(&self) -> &Self::Target { &self.block_state }
}
