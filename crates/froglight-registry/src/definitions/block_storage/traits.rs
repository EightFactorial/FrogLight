use std::fmt::Debug;

use bevy_reflect::Reflect;
use froglight_protocol::traits::Version;

use super::BlockStorage;

/// A block for a specific [`Version`].
pub trait BlockType<V>
where
    Self: 'static + Debug + Reflect + Send + Sync,
    V: Version,
{
    /// The block's [`ResourceKey`](froglight_protocol::common::ResourceKey).
    ///
    /// This is used to identify the block in the resource pack.
    ///
    /// # Example
    /// ```rust
    /// let air = "minecraft:air";
    /// let dirt = "minecraft:dirt";
    /// let stone = "minecraft:stone";
    /// ```
    #[must_use]
    fn to_key(&self) -> &'static str;
    /// The block's language key.
    ///
    /// This is used to identify the block in the language files.
    ///
    /// # Example
    /// ```rust
    /// let air = "block.minecraft.air";
    /// let dirt = "block.minecraft.dirt";
    /// let stone = "block.minecraft.stone";
    /// ```
    #[must_use]
    fn to_lang(&self) -> &'static str;

    /// Returns `true` if the block is air.
    #[must_use]
    fn is_air(&self) -> bool { false }
    /// Returns `true` if the block is opaque.
    #[must_use]
    fn is_opaque(&self) -> bool { true }
    /// Returns `true` if the block is collidable.
    #[must_use]
    fn is_collidable(&self) -> bool { true }
}

/// An extension trait for [`BlockType`].
pub trait BlockExt<V>
where
    Self: Sized + BlockType<V>,
    V: Version,
{
    /// The total number of states for this block.
    ///
    /// This is `1` by default, and is equal
    /// to the number of states each block attribute
    /// has multiplied together.
    ///
    /// # Example
    /// ```rust
    /// use froglight_registry::{
    ///     attributes::SnowyAttribute,
    ///     blocks::{Air, GrassBlock, Stone},
    ///     definitions::BlockExt,
    /// };
    ///
    /// // Air has no attributes, so it has `1` block state.
    /// assert_eq!(<Air as BlockExt<V1_21_0>>::BLOCK_STATES, 1);
    ///
    /// // Stone has no attributes, so it has `1` block state.
    /// assert_eq!(<Stone as BlockExt<V1_21_0>>::BLOCK_STATES, 1);
    ///
    /// // Grass has one attribute, `SnowyAttribute`.
    /// // `SnowyAttribute` has `2` states, `true` and `false`.
    /// assert_eq!(<GrassBlock as BlockExt<V1_21_0>>::BLOCK_STATES, 2);
    /// ```
    const BLOCK_STATES: u32 = 1u32;

    /// The block's default state.
    #[must_use]
    fn default_block() -> Self;

    /// Get a block from it's *relative* `block state id`.
    ///
    /// This is equivalent to subtracting the first `block state id`
    /// for this block from it's actual `block state id`.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::versions::v1_21_0::V1_21_0;
    /// use froglight_registry::{
    ///     attributes::SnowyAttribute,
    ///     blocks::GrassBlock,
    ///     definitions::{BlockExt, BlockStorage},
    /// };
    ///
    /// let storage = BlockStorage::<V1_21_0>::new();
    ///
    /// // Grass has two block state ids, `8` and `9`.
    /// let grass_default = <GrassBlock as BlockExt<V1_21_0>>::default_block();
    /// let grass_range = storage.blockstate_range(&grass_default).unwrap();
    ///
    /// // The first variant of grass has `SnowyAttribute(true)`.
    /// let first_relative = grass_range.start + 0;
    /// let grass_snowy = <GrassBlock as BlockExt<V1_21_0>>::from_relative_id(&first_relative).unwrap();
    /// assert_eq!(grass_snowy, GrassBlock { snowy: SnowyAttribute(true) });
    ///
    /// // The second variant of grass has `SnowyAttribute(false)`.
    /// let second_relative = grass_range.start + 1;
    /// let grass_normal =
    ///     <GrassBlock as BlockExt<V1_21_0>>::from_relative_id(&second_relative).unwrap();
    /// assert_eq!(grass_normal, GrassBlock { snowy: SnowyAttribute(false) });
    /// ```
    #[must_use]
    fn from_relative_id(relative_id: u32) -> Option<Self> {
        if relative_id == 0 {
            Some(Self::default_block())
        } else {
            None
        }
    }

    /// Get a block from it's `block state id`.
    ///
    /// Requires the block to have been registered in the [`BlockStorage`].
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::versions::v1_21_0::V1_21_0;
    /// use froglight_registry::{
    ///     attributes::SnowyAttribute,
    ///     blocks::GrassBlock,
    ///     definitions::{BlockExt, BlockStorage},
    /// };
    ///
    /// let storage = BlockStorage::<V1_21_0>::new();
    ///
    /// // Grass has two block state ids, `8` and `9`.
    /// let grass_snowy = <GrassBlock as BlockExt<V1_21_0>>::from_blockstate_id(8, &storage).unwrap();
    /// assert_eq!(grass_snowy, GrassBlock { snowy: SnowyAttribute(true) });
    ///
    /// let grass_normal = <GrassBlock as BlockExt<V1_21_0>>::from_blockstate_id(9, &storage).unwrap();
    /// assert_eq!(grass_normal, GrassBlock { snowy: SnowyAttribute(false) });
    /// ```
    #[must_use]
    fn from_blockstate_id(blockstate_id: u32, storage: &BlockStorage<V>) -> Option<Self> {
        let blockstate_range = storage.blockstate_range_of(blockstate_id)?;
        Self::from_relative_id(blockstate_id - blockstate_range.start)
    }

    /// Get the block's *relative* `block state id`.
    ///
    /// This is equivalent to subtracting the first `block state id`
    /// for this block from it's actual `block state id`.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::versions::v1_21_0::V1_21_0;
    /// use froglight_registry::{
    ///     attributes::SnowyAttribute,
    ///     blocks::GrassBlock,
    ///     definitions::{BlockExt, BlockStorage},
    /// };
    ///
    /// let storage = BlockStorage::<V1_21_0>::new();
    ///
    /// // The first variant of grass has `SnowyAttribute(true)`.
    /// let grass_snowy = GrassBlock { snowy: SnowyAttribute(true) }.to_relative_id();
    /// assert_eq!(grass_snowy, 0);
    ///
    /// // The second variant of grass has `SnowyAttribute(false)`.
    /// let grass_normal = GrassBlock { snowy: SnowyAttribute(false) }.to_relative_id();
    /// assert_eq!(grass_normal, 1);
    /// ```
    #[must_use]
    fn to_relative_id(&self) -> u32 { 0 }

    /// Get the block's `block state id`.
    ///
    /// Requires the block to have been registered in the [`BlockStorage`].
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::versions::v1_21_0::V1_21_0;
    /// use froglight_registry::{
    ///     attributes::SnowyAttribute,
    ///     blocks::GrassBlock,
    ///     definitions::{BlockExt, BlockStorage},
    /// };
    ///
    /// let storage = BlockStorage::<V1_21_0>::new();
    ///
    /// // The first variant of grass has `SnowyAttribute(true)`.
    /// let grass_snowy =
    ///     GrassBlock { snowy: SnowyAttribute(true) }.to_blockstate_id(&storage).unwrap();
    /// assert_eq!(grass_snowy, 8);
    ///
    /// // The second variant of grass has `SnowyAttribute(false)`.
    /// let grass_normal =
    ///     GrassBlock { snowy: SnowyAttribute(false) }.to_blockstate_id(&storage).unwrap();
    /// assert_eq!(grass_normal, 9);
    /// ```
    #[must_use]
    fn to_blockstate_id(&self, storage: &BlockStorage<V>) -> Option<u32> {
        let blockstate_range = storage.blockstate_range(self)?;
        Some(blockstate_range.start + self.to_relative_id())
    }
}

/// A block attribute for a specific [`Version`].
///
/// # Example
/// ```rust
/// use bevy_derive::{Deref, DerefMut};
/// use bevy_reflect::Reflect;
/// use froglight_protocol::versions::v1_21_0::V1_21_0;
/// use froglight_registry::definitions::BlockAttribute;
///
/// /// A custom boolean attribute.
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Reflect)]
/// struct MyAttribute(bool);
///
/// impl BlockAttribute<V1_21_0> for MyAttribute {
///     const ATTRIBUTE_STATES: u32 = 2;
/// }
/// ```
pub trait BlockAttribute<V>
where
    Self: 'static + Debug + Reflect + Send + Sync,
    V: Version,
{
    /// The total number of attribute states.
    ///
    /// For boolean attributes, this is `2`,
    /// `true` and `false`.
    ///
    /// For enum attributes, this is the number
    /// of variants the enum has.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_protocol::versions::v1_21_0::V1_21_0;
    /// use froglight_registry::{attributes::SnowyAttribute, definitions::BlockAttribute};
    ///
    /// // SnowyAttribute has two states, `true` and `false`.
    /// assert_eq!(<SnowyAttribute as BlockAttribute<V1_21_0>>::ATTRIBUTE_STATES, 2);
    /// ```
    const ATTRIBUTE_STATES: u32;
}

/// A block state resolver for a specific [`Version`].
///
/// # Example
/// ```rust
/// use bevy_reflect::Reflect;
/// use froglight_protocol::versions::v1_21_0::V1_21_0;
/// use froglight_registry::definitions::{BlockExt, BlockStateResolver, BlockStorage, BlockType};
///
/// /// A custom block type.
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
/// struct MyBlock;
///
/// impl BlockType<V1_21_0> for MyBlock {
///     fn to_key(&self) -> &'static str { "froglight:my_block" }
///     fn to_lang(&self) -> &'static str { "block.froglight.my_block" }
/// }
///
/// impl BlockExt<V1_21_0> for MyBlock {
///     fn default_block() -> Self { MyBlock }
/// }
///
/// /// A custom block state resolver.
/// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
/// struct MyResolver;
///
/// impl BlockStateResolver<V1_21_0> for MyResolver {
///     /// We only care about `MyBlock`.
///     type Resolved = Option<MyBlock>;
///
///     /// We only check for `MyBlock`, return `None` for all other blocks.
///     fn resolve_state(blockstate_id: u32, storage: &BlockStorage<V1_21_0>) -> Self::Resolved {
///         let default_dyn = storage.default_blockstate(blockstate_id).unwrap();
///         default_dyn.as_any().downcast_ref::<MyBlock>().cloned()
///     }
///
///     /// Register `MyBlock` with the storage.
///     fn register_blocks(storage: &mut BlockStorage<V1_21_0>) { storage.register::<MyBlock>(); }
/// }
/// ```
pub trait BlockStateResolver<V>
where
    Self: 'static + Debug + Send + Sync,
    V: Version,
{
    /// The type of block being resolved.
    type Resolved;

    /// Resolve a [`Self::Resolved`] from it's `block state id`.
    #[must_use]
    fn resolve_state(blockstate_id: u32, storage: &BlockStorage<V>) -> Self::Resolved;

    /// Register all blocks for this resolver.
    ///
    /// This should call [`BlockStorage::register`] for each block type.
    fn register_blocks(storage: &mut BlockStorage<V>);
}
