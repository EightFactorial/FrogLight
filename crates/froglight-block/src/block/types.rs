//! !TODO

use core::marker::PhantomData;

#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use downcast_rs::Downcast;
use froglight_common::{identifier::Identifier, version::Version};

use super::{BlockType, BlockTypeExt};
use crate::{
    resolve::BlockResolver,
    storage::{Attribute, BlockAttributes, BlockWrapper, RelativeBlockState},
};

/// A block with a state.
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(no_field_bounds, Default, Clone, PartialEq))]
pub struct Block<B: BlockTypeExt<V>, V: Version> {
    state: RelativeBlockState,
    #[cfg_attr(feature = "reflect", reflect(ignore))]
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

    /// Get the [`Attributes`](BlockTypeExt::Attributes) of the [`Block`].
    ///
    /// Returns `None` if the [`Attribute`] is not present.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let block = Block::<block::GrassBlock, V1_21_4>::default();
    ///     // Grass does have the `SnowyBool` attribute
    ///     assert_eq!(block.get_attr::<attribute::SnowyBool>(), Some(attribute::SnowyBool::False));
    ///     // Grass does not have the `WaterloggedBool` attribute
    ///     assert_eq!(block.get_attr::<attribute::WaterloggedBool>(), None);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn get_attr<T: Attribute>(&self) -> Option<T> {
        B::Attributes::get_attr::<T>(&self.into_attr())
    }

    /// Modify the [`Attributes`](BlockTypeExt::Attributes) of the [`Block`].
    ///
    /// This is shorthand for calling
    /// [`Block::into_attr`] and [`Block::from_attr`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let mut block = Block::<block::GrassBlock, V1_21_4>::default();
    ///     // Get the default attribute
    ///     assert_eq!(block.into_attr(), attribute::SnowyBool::False);
    ///     // Set the attribute to `true`
    ///     block.scoped_attr(|_snowy| attribute::SnowyBool::True);
    ///     // Verify the attribute was set
    ///     assert_eq!(block.into_attr(), attribute::SnowyBool::True);
    /// }
    /// ```
    #[inline]
    pub fn scoped_attr(&mut self, f: fn(B::Attributes) -> B::Attributes) {
        *self = Self::from_attr(f(self.into_attr()));
    }

    /// Create a [`Block`] from the given
    /// [`Attributes`](BlockTypeExt::Attributes).
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let block = Block::<block::GrassBlock, V1_21_4>::default();
    ///
    ///     // Get the attributes of the block
    ///     let snowy: attribute::SnowyBool = block.into_attr();
    ///     assert_eq!(snowy, attribute::SnowyBool::False);
    ///
    ///     // Create a new block from the same attributes
    ///     let new_block = Block::from_attr(snowy);
    ///     assert_eq!(block, new_block);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn from_attr(attributes: B::Attributes) -> Self {
        Self { state: RelativeBlockState::from(attributes.into_index()), _phantom: PhantomData }
    }

    /// Get the [`Attributes`](BlockTypeExt::Attributes) of the [`Block`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let block = Block::<block::GrassBlock, V1_21_4>::default();
    ///
    ///     // Get the attributes of the block
    ///     let snowy: attribute::SnowyBool = block.into_attr();
    ///     assert_eq!(snowy, attribute::SnowyBool::False);
    ///
    ///     // Create a new block from the same attributes
    ///     let new_block = Block::from_attr(snowy);
    ///     assert_eq!(block, new_block);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn into_attr(self) -> B::Attributes { B::Attributes::from_index(usize::from(self.state)) }

    /// Get the value of an [`Attribute`] as a string.
    ///
    /// Returns `None` if the [`Attribute`] is not present.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let mut block = Block::<block::GrassBlock, V1_21_4>::default();
    ///
    ///     // Check the default value of the attribute
    ///     assert_eq!(block.get_attr_str("snowy"), Some("false"));
    ///     // Set the attribute to `true`
    ///     assert_eq!(block.set_attr_str("snowy", "true"), Some("false"));
    ///     // Verify the attribute was set
    ///     assert_eq!(block.get_attr_str("snowy"), Some("true"));
    /// }
    /// ```
    #[must_use]
    pub fn get_attr_str(&self, attr: &str) -> Option<&'static str> {
        B::ATTRIBUTES
            .iter()
            .position(|&name| name == attr)
            .map(|i| self.into_attr().get_attr_str(i))
    }

    /// Set the value of an [`Attribute`] with a string.
    ///
    /// Returns the previous value of the [`Attribute`],
    /// or `None` if the [`Attribute`] is not present.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let mut block = Block::<block::GrassBlock, V1_21_4>::default();
    ///
    ///     // Check the default value of the attribute
    ///     assert_eq!(block.get_attr_str("snowy"), Some("false"));
    ///     // Set the attribute to `true`
    ///     assert_eq!(block.set_attr_str("snowy", "true"), Some("false"));
    ///     // Verify the attribute was set
    ///     assert_eq!(block.get_attr_str("snowy"), Some("true"));
    /// }
    /// ```
    pub fn set_attr_str(&mut self, attr: &str, value: &'static str) -> Option<&'static str> {
        B::ATTRIBUTES.iter().position(|&name| name == attr).and_then(|i| {
            let mut attr = self.into_attr();
            attr.set_attr_str(i, value).inspect(|_| *self = Self::from_attr(attr))
        })
    }

    /// Iterate over the [`Attributes`](BlockTypeExt::Attributes) of the
    /// [`Block`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // `Air` does not have any attributes
    ///     let air = Block::<block::Air, V1_21_4>::default();
    ///     assert!(air.iter_attr_str().collect::<Vec<_>>().is_empty());
    ///
    ///     // `GrassBlock` has the `SnowyBool` attribute
    ///     let grass = Block::<block::GrassBlock, V1_21_4>::default();
    ///     assert_eq!(grass.iter_attr_str().collect::<Vec<_>>(), vec![("snowy", "false")]);
    ///
    ///     // `OakLeaves` has the `DistanceInt`, `PersistentBool`, and `WaterloggedBool` attributes
    ///     let leaves = Block::<block::OakLeaves, V1_21_4>::default();
    ///     assert_eq!(
    ///         leaves.iter_attr_str().collect::<Vec<_>>(),
    ///         vec![("distance", "7"), ("persistent", "false"), ("waterlogged", "false")]
    ///     );
    /// }
    /// ```
    pub fn iter_attr_str(&self) -> impl Iterator<Item = (&'static str, &'static str)> {
        let attr = self.into_attr();
        B::ATTRIBUTES.iter().enumerate().map(move |(i, name)| (*name, attr.get_attr_str(i)))
    }

    /// Convert the [`Block`] into an [`UntypedBlock`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let mut blocks: Vec<UntypedBlock<V1_21_4>> = Vec::with_capacity(4);
    ///     blocks.push(Block::<block::Air, V1_21_4>::default().into_untyped());
    ///     blocks.push(Block::<block::Stone, V1_21_4>::default().into_untyped());
    ///     blocks.push(Block::<block::Dirt, V1_21_4>::default().into_untyped());
    ///     blocks.push(Block::<block::GrassBlock, V1_21_4>::default().into_untyped());
    ///
    ///     assert_eq!(blocks.len(), 4);
    ///     assert_eq!(blocks[0].identifier(), "minecraft:air");
    ///     assert_eq!(blocks[1].identifier(), "minecraft:stone");
    ///     assert_eq!(blocks[2].identifier(), "minecraft:dirt");
    ///     assert_eq!(blocks[3].identifier(), "minecraft:grass_block");
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn into_untyped(self) -> UntypedBlock<V> { self.into() }

    /// Get the identifier of the [`Block`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     assert_eq!(Block::<block::Air, V1_21_4>::const_identifier(), "minecraft:air");
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub const fn const_identifier() -> &'static str { B::IDENTIFIER }

    /// Get the identifier of the [`Block`].
    ///
    /// Matches [`UntypedBlock::identifier`] for consistency.
    ///
    /// If you need `const` access, see
    /// [`Block::const_identifier`] or [`BlockTypeExt::IDENTIFIER`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     assert_eq!(Block::<block::Air, V1_21_4>::default().identifier(), "minecraft:air");
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn identifier(&self) -> &'static Identifier { B::as_static().identifier() }

    /// Get whether the [`Block`] is a type of air.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     assert!(Block::<block::Air, V1_21_4>::const_is_air());
    ///     assert!(!Block::<block::Stone, V1_21_4>::const_is_air());
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub const fn const_is_air() -> bool { B::IS_AIR }

    /// Get whether the [`Block`] is a type of air.
    ///
    /// Matches [`UntypedBlock::is_air`] for consistency.
    ///
    /// If you need `const` access, see
    /// [`Block::const_is_air`] or [`BlockTypeExt::IS_AIR`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     assert!(Block::<block::Air, V1_21_4>::default().is_air());
    ///     assert!(!Block::<block::Stone, V1_21_4>::default().is_air());
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn is_air(&self) -> bool { B::as_static().is_air() }
}

impl<B: BlockTypeExt<V>, V: Version> Default for Block<B, V> {
    fn default() -> Self { Self::new(RelativeBlockState::new_unchecked(B::DEFAULT)) }
}

impl<B: BlockTypeExt<V>, V: Version> TryFrom<UntypedBlock<V>> for Block<B, V> {
    type Error = UntypedBlock<V>;

    fn try_from(value: UntypedBlock<V>) -> Result<Self, Self::Error> {
        if let Some(value) = value.downcast::<B>() { Ok(value) } else { Err(value) }
    }
}

// -------------------------------------------------------------------------------------------------

/// An untyped block with a state.
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(no_field_bounds, from_reflect = false, PartialEq))]
pub struct UntypedBlock<V: Version> {
    state: RelativeBlockState,
    #[cfg_attr(feature = "reflect", reflect(ignore))]
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
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    /// use froglight_common::vanilla::Vanilla;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_block::generated::v1_21_4::VersionBlocks;
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let block = Block::<block::Air, V1_21_4>::default();
    ///     assert_eq!(block.into_untyped().resolve::<Vanilla>(), Some(VersionBlocks::Air(block)));
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn resolve<R: BlockResolver<V>>(self) -> Option<R::BlockEnum> { R::resolve(self) }

    /// Returns `true` if the [`Block`] is of a [`BlockType`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    /// use froglight_common::vanilla::Vanilla;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let block = Block::<block::Air, V1_21_4>::default();
    ///     assert!(block.into_untyped().is::<block::Air>());
    /// }
    /// ```
    #[must_use]
    pub fn is<B: BlockTypeExt<V>>(&self) -> bool {
        <dyn BlockType<V> as Downcast>::as_any(*self.wrapper).type_id()
            == <dyn BlockType<V> as Downcast>::as_any(B::as_static()).type_id()
    }

    /// Downcast the [`UntypedBlock`] into a [`Block`].
    ///
    /// Returns `None` if the [`Block`] is not of the given [`BlockType`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    /// use froglight_common::vanilla::Vanilla;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let block = Block::<block::Air, V1_21_4>::default();
    ///     assert_eq!(block.into_untyped().downcast::<block::Air>(), Some(block));
    /// }
    /// ```
    #[must_use]
    pub fn downcast<B: BlockTypeExt<V>>(self) -> Option<Block<B, V>> {
        self.is::<B>().then(|| Block::new(self.state))
    }

    /// Get the identifier of the [`UntypedBlock`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    /// use froglight_common::vanilla::Vanilla;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let block = Block::<block::Air, V1_21_4>::default();
    ///     assert_eq!(block.into_untyped().identifier(), "minecraft:air");
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn identifier(&self) -> &'static Identifier { self.wrapper.identifier() }

    /// Get whether the [`UntypedBlock`] is a type of air.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let block = Block::<block::Air, V1_21_4>::default();
    ///     assert!(block.into_untyped().is_air());
    ///
    ///     let block = Block::<block::Stone, V1_21_4>::default();
    ///     assert!(!block.into_untyped().is_air());
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn is_air(&self) -> bool { self.wrapper.is_air() }

    /// Get the value of an [`Attribute`] as a string.
    ///
    /// Returns `None` if the [`Attribute`] is not present.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let mut block = Block::<block::GrassBlock, V1_21_4>::default().into_untyped();
    ///
    ///     // Check the default value of the attribute
    ///     assert_eq!(block.get_attr_str("snowy"), Some("false"));
    /// }
    /// ```
    #[must_use]
    pub fn get_attr_str(&self, attr: &str) -> Option<&'static str> {
        self.wrapper.get_attr_str(self.state.into(), attr)
    }
}

impl<B: BlockTypeExt<V>, V: Version> From<Block<B, V>> for UntypedBlock<V> {
    #[inline]
    fn from(block: Block<B, V>) -> Self {
        UntypedBlock::new(block.state, BlockWrapper::new(B::as_static()))
    }
}

// ------------- Manual trait implementations to avoid trait bounds -----------

impl<B: BlockTypeExt<V>, V: Version> core::fmt::Debug for Block<B, V> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Block").field(&self.state).field(&Self::const_identifier()).finish()
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

impl<V: Version> core::fmt::Debug for UntypedBlock<V> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
